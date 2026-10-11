//! The SQLite database. The schema grows through numbered migrations tracked in
//! `PRAGMA user_version`, never by drop and rebuild: it is the only record of calls whose
//! transcripts Claude Code has deleted.

use crate::notes::Entry;
use crate::window::SkillCall;
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

/// Applied in order; a migration is never edited once merged, the next change appends one.
const MIGRATIONS: &[&str] = &[
    "CREATE TABLE skill_calls (
        start_uuid     TEXT PRIMARY KEY,
        skill          TEXT NOT NULL,
        session_id     TEXT NOT NULL,
        started_at     TEXT NOT NULL,
        ended_at       TEXT NOT NULL,
        user_prompts   INTEGER NOT NULL,
        parser_version INTEGER NOT NULL
    );",
    "CREATE TABLE nested_skills (
        start_uuid TEXT NOT NULL REFERENCES skill_calls (start_uuid) ON DELETE CASCADE,
        position   INTEGER NOT NULL,
        skill      TEXT NOT NULL,
        loaded_at  TEXT NOT NULL,
        PRIMARY KEY (start_uuid, position)
    );",
    // NULL on rows an older parser wrote and that are never re-extracted: not measured, not zero.
    "ALTER TABLE skill_calls ADD COLUMN input_tokens INTEGER;
    ALTER TABLE skill_calls ADD COLUMN output_tokens INTEGER;
    ALTER TABLE skill_calls ADD COLUMN cache_read_tokens INTEGER;
    ALTER TABLE skill_calls ADD COLUMN cache_write_tokens INTEGER;
    ALTER TABLE skill_calls ADD COLUMN active_ms INTEGER;
    ALTER TABLE skill_calls ADD COLUMN claude_code_version TEXT;
    ALTER TABLE skill_calls ADD COLUMN entrypoint TEXT;
    ALTER TABLE skill_calls ADD COLUMN cwd TEXT;
    ALTER TABLE skill_calls ADD COLUMN git_branch TEXT;
    CREATE TABLE models (
        start_uuid TEXT NOT NULL REFERENCES skill_calls (start_uuid) ON DELETE CASCADE,
        model      TEXT NOT NULL,
        PRIMARY KEY (start_uuid, model)
    );",
    "ALTER TABLE skill_calls ADD COLUMN tool_calls INTEGER;
    ALTER TABLE skill_calls ADD COLUMN tool_errors INTEGER;
    ALTER TABLE skill_calls ADD COLUMN interrupts INTEGER;
    ALTER TABLE skill_calls ADD COLUMN questions INTEGER;
    ALTER TABLE skill_calls ADD COLUMN recommended_answers INTEGER;
    ALTER TABLE skill_calls ADD COLUMN own_word_answers INTEGER;
    ALTER TABLE skill_calls ADD COLUMN subagents INTEGER;
    ALTER TABLE skill_calls ADD COLUMN commits INTEGER;
    ALTER TABLE skill_calls ADD COLUMN pushes INTEGER;
    ALTER TABLE skill_calls ADD COLUMN prs_created INTEGER;
    CREATE TABLE denials (
        start_uuid TEXT NOT NULL REFERENCES skill_calls (start_uuid) ON DELETE CASCADE,
        kind       TEXT NOT NULL,
        count      INTEGER NOT NULL,
        PRIMARY KEY (start_uuid, kind)
    );
    CREATE TABLE pull_requests (
        start_uuid TEXT NOT NULL REFERENCES skill_calls (start_uuid) ON DELETE CASCADE,
        url        TEXT NOT NULL,
        PRIMARY KEY (start_uuid, url)
    );",
    // NULL also for a skill that never had a SKILL.md in the repo (plugin skills).
    "ALTER TABLE skill_calls ADD COLUMN skill_version TEXT;
    ALTER TABLE nested_skills ADD COLUMN skill_version TEXT;",
    "CREATE TABLE user_texts (
        start_uuid TEXT NOT NULL REFERENCES skill_calls (start_uuid) ON DELETE CASCADE,
        position   INTEGER NOT NULL,
        kind       TEXT NOT NULL,
        text       TEXT NOT NULL,
        PRIMARY KEY (start_uuid, position)
    );",
    // Set by whetstone, never by extraction; NULL until set.
    "ALTER TABLE skill_calls ADD COLUMN status TEXT
        CHECK (status IN ('offen', 'beobachtet', 'umgesetzt', 'weitergegeben', 'verworfen'));
    ALTER TABLE skill_calls ADD COLUMN finding TEXT;",
    // NULL on rows an older parser wrote and that are never re-extracted: not measured.
    "ALTER TABLE user_texts ADD COLUMN correction INTEGER;
    ALTER TABLE user_texts ADD COLUMN strong INTEGER;",
    // NULL on rows an older parser wrote and that are never re-extracted.
    "ALTER TABLE skill_calls ADD COLUMN start_kind TEXT CHECK (start_kind IN ('slash', 'model'));",
    // Choice tool calls, not questions (`questions` counts those). NULL on rows an older parser
    // wrote and that are never re-extracted.
    "ALTER TABLE skill_calls ADD COLUMN choice_questions INTEGER;
    ALTER TABLE skill_calls ADD COLUMN bare_questions INTEGER;",
];

/// Whetstone's statuses; a call without one is `offen`.
pub const STATUSES: [&str; 5] = [
    "offen",
    "beobachtet",
    "umgesetzt",
    "weitergegeben",
    "verworfen",
];

pub fn open(path: &Path) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "foreign_keys", true)?;
    migrate(&conn)?;
    Ok(conn)
}

fn migrate(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let applied: usize = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if applied > MIGRATIONS.len() {
        let built = MIGRATIONS.len();
        return Err(format!("database schema {applied} is newer than this build ({built})").into());
    }
    for (index, sql) in MIGRATIONS.iter().enumerate().skip(applied) {
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", index + 1)?;
        tx.commit()?;
    }
    Ok(())
}

/// Replaces each skill call with its child rows (cascading deletes) in one transaction, unless the
/// stored row comes from a newer parser version or from the same one with a later window end. So a
/// fork or resume that shares a start uuid counts once, with its longest window, in any file order.
/// Rows of skill calls whose transcript is gone stay untouched. Status and finding survive the
/// replacement.
pub fn upsert(
    conn: &mut Connection,
    calls: &[SkillCall],
    parser_version: i64,
) -> rusqlite::Result<()> {
    // Immediate: the hook and a full scan may write at once; the second waits for the lock
    // instead of failing on the upgrade from read to write.
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    for call in calls {
        let stored: Option<(i64, String, Option<String>, Option<String>)> = tx
            .query_row(
                "SELECT parser_version, ended_at, status, finding FROM skill_calls
                 WHERE start_uuid = ?1",
                [&call.start_uuid],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()?;
        if stored.as_ref().is_some_and(|(version, ended_at, ..)| {
            (*version, ended_at.as_str()) > (parser_version, call.ended_at.as_str())
        }) {
            continue;
        }
        let (status, finding) =
            stored.map_or((None, None), |(.., status, finding)| (status, finding));
        tx.execute(
            "DELETE FROM skill_calls WHERE start_uuid = ?1",
            [&call.start_uuid],
        )?;
        tx.execute(
            "INSERT INTO skill_calls (start_uuid, skill, session_id, started_at, ended_at,
                 user_prompts, parser_version, input_tokens, output_tokens, cache_read_tokens,
                 cache_write_tokens, active_ms, claude_code_version, entrypoint, cwd, git_branch,
                 tool_calls, tool_errors, interrupts, questions, recommended_answers,
                 own_word_answers, subagents, commits, pushes, prs_created, skill_version, status,
                 finding, start_kind, choice_questions, bare_questions)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16,
                 ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32)",
            params![
                call.start_uuid,
                call.skill,
                call.session_id,
                call.started_at,
                call.ended_at,
                call.user_prompts,
                parser_version,
                call.metrics.tokens.input,
                call.metrics.tokens.output,
                call.metrics.tokens.cache_read,
                call.metrics.tokens.cache_write,
                call.metrics.active_ms,
                call.metrics.context.version,
                call.metrics.context.entrypoint,
                call.metrics.context.cwd,
                call.metrics.context.git_branch,
                call.metrics.tool_calls,
                call.metrics.tool_errors,
                call.metrics.interrupts,
                call.metrics.questions,
                call.metrics.recommended_answers,
                call.metrics.own_word_answers,
                call.metrics.subagents,
                call.metrics.commits,
                call.metrics.pushes,
                call.metrics.prs_created,
                call.skill_version,
                status,
                finding,
                call.start_kind,
                call.metrics.choice_questions,
                call.metrics.bare_questions,
            ],
        )?;
        for (kind, count) in &call.metrics.denials {
            tx.execute(
                "INSERT INTO denials (start_uuid, kind, count) VALUES (?1, ?2, ?3)",
                params![call.start_uuid, kind, count],
            )?;
        }
        for url in &call.metrics.pr_urls {
            tx.execute(
                "INSERT INTO pull_requests (start_uuid, url) VALUES (?1, ?2)",
                params![call.start_uuid, url],
            )?;
        }
        for model in &call.metrics.models {
            tx.execute(
                "INSERT INTO models (start_uuid, model) VALUES (?1, ?2)",
                params![call.start_uuid, model],
            )?;
        }
        for (position, user_text) in call.metrics.user_texts.iter().enumerate() {
            tx.execute(
                "INSERT INTO user_texts (start_uuid, position, kind, text, correction, strong)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    call.start_uuid,
                    position,
                    user_text.kind,
                    user_text.text,
                    user_text.correction,
                    user_text.strong
                ],
            )?;
        }
        for (position, nested) in call.nested.iter().enumerate() {
            tx.execute(
                "INSERT INTO nested_skills (start_uuid, position, skill, loaded_at, skill_version)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    call.start_uuid,
                    position,
                    nested.skill,
                    nested.loaded_at,
                    nested.skill_version
                ],
            )?;
        }
    }
    tx.commit()
}

/// A skill call `open` lists.
pub struct OpenCall {
    pub start_uuid: String,
    pub skill: String,
    pub session_id: String,
    pub started_at: String,
    /// `name=value` per counter; NULL counters (rows of an older parser) are left out.
    pub counters: Vec<(&'static str, i64)>,
}

/// Calls without a status (or `offen`), by start time. With `skill`, the calls of that skill and
/// the calls it was a nested skill in.
pub fn open_calls(conn: &Connection, skill: Option<&str>) -> rusqlite::Result<Vec<OpenCall>> {
    const COUNTERS: [&str; 6] = [
        "prompts",
        "interrupts",
        "denials",
        "tool_errors",
        "questions",
        "own_word_answers",
    ];
    let mut stmt = conn.prepare(
        "SELECT start_uuid, skill, session_id, started_at, user_prompts, interrupts,
             -- An older parser wrote no denials: NULL, not 0.
             CASE WHEN tool_calls IS NOT NULL THEN
                 (SELECT coalesce(sum(count), 0) FROM denials d WHERE d.start_uuid = c.start_uuid)
             END,
             tool_errors, questions, own_word_answers
         FROM skill_calls c
         WHERE (status IS NULL OR status = 'offen')
           AND (?1 IS NULL OR skill = ?1
                OR start_uuid IN (SELECT start_uuid FROM nested_skills WHERE skill = ?1))
         ORDER BY started_at, start_uuid",
    )?;
    let calls = stmt.query_map([skill], |row| {
        let mut counters = Vec::new();
        for (index, name) in COUNTERS.into_iter().enumerate() {
            if let Some(value) = row.get::<_, Option<i64>>(4 + index)? {
                counters.push((name, value));
            }
        }
        Ok(OpenCall {
            start_uuid: row.get(0)?,
            skill: row.get(1)?,
            session_id: row.get(2)?,
            started_at: row.get(3)?,
            counters,
        })
    })?;
    calls.collect()
}

/// Calls with a status other than `offen`, by start time, as (status, start uuid, skill,
/// session, start time, finding). `skill` filters as in `open_calls`.
pub fn closed_calls(conn: &Connection, skill: Option<&str>) -> rusqlite::Result<Vec<[String; 6]>> {
    let mut stmt = conn.prepare(
        "SELECT status, start_uuid, skill, session_id, started_at, coalesce(finding, '')
         FROM skill_calls
         WHERE status IS NOT NULL AND status != 'offen'
           AND (?1 IS NULL OR skill = ?1
                OR start_uuid IN (SELECT start_uuid FROM nested_skills WHERE skill = ?1))
         ORDER BY started_at, start_uuid",
    )?;
    let calls = stmt.query_map([skill], |row| {
        Ok([
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ])
    })?;
    calls.collect()
}

/// A skill call `show` prints.
pub struct ShownCall {
    pub skill: String,
    pub session_id: String,
    pub started_at: String,
    pub ended_at: String,
    pub status: Option<String>,
    pub finding: Option<String>,
    /// In window order.
    pub user_texts: Vec<StoredUserText>,
}

pub struct StoredUserText {
    pub kind: String,
    pub text: String,
    /// NULL on rows of an older parser.
    pub correction: Option<bool>,
    pub strong: Option<bool>,
}

pub fn shown_call(conn: &Connection, start_uuid: &str) -> rusqlite::Result<Option<ShownCall>> {
    let Some(mut call) = conn
        .query_row(
            "SELECT skill, session_id, started_at, ended_at, status, finding FROM skill_calls
             WHERE start_uuid = ?1",
            [start_uuid],
            |row| {
                Ok(ShownCall {
                    skill: row.get(0)?,
                    session_id: row.get(1)?,
                    started_at: row.get(2)?,
                    ended_at: row.get(3)?,
                    status: row.get(4)?,
                    finding: row.get(5)?,
                    user_texts: Vec::new(),
                })
            },
        )
        .optional()?
    else {
        return Ok(None);
    };
    call.user_texts = conn
        .prepare(
            "SELECT kind, text, correction, strong FROM user_texts WHERE start_uuid = ?1
             ORDER BY position",
        )?
        .query_map([start_uuid], |row| {
            Ok(StoredUserText {
                kind: row.get(0)?,
                text: row.get(1)?,
                correction: row.get(2)?,
                strong: row.get(3)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    Ok(Some(call))
}

/// Sets a call's status and finding; false when no call has that start uuid.
pub fn set_status(
    conn: &Connection,
    start_uuid: &str,
    status: &str,
    finding: &str,
) -> rusqlite::Result<bool> {
    let changed = conn.execute(
        "UPDATE skill_calls SET status = ?2, finding = ?3 WHERE start_uuid = ?1",
        params![start_uuid, status, finding],
    )?;
    Ok(changed > 0)
}

/// What `import` did.
pub struct Imported {
    /// Indices into the entries that matched no skill call.
    pub unmatched: Vec<usize>,
    /// Calls that got an entry's status.
    pub set: usize,
    /// Calls that more than one skill's entry matched: start uuid, the entries added to the
    /// finding.
    pub merged: Vec<(String, String)>,
    /// Calls without a status that no entry matched, now `verworfen` with `vor Umstellung`.
    pub discarded: usize,
}

/// Puts each entry's status and finding on every call of its skill in its session, own or nested,
/// once: it refuses a database where a call already has a status. A call keeps one entry per
/// skill, the later one in `entries`. Its status comes from its own skill's entry, else from the
/// latest entry; the other skills' entries are added to the finding as `<skill>: <status
/// (finding)>`. Then every call still without a status is discarded, in one transaction.
///
/// `before` runs inside the transaction once no call has a status, for a backup copy.
pub fn import(
    conn: &mut Connection,
    entries: &[Entry],
    before: impl FnOnce() -> std::io::Result<()>,
) -> Result<Imported, Box<dyn Error>> {
    let tx = conn.transaction()?;
    let set: i64 = tx.query_row(
        "SELECT count(*) FROM skill_calls WHERE status IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    if set > 0 {
        return Err(
            format!("{set} skill calls already have a status; import-notes runs once").into(),
        );
    }
    before()?;
    // start uuid → (call's own skill, entry index per skill in order of last match)
    let mut matched: HashMap<String, (String, Vec<usize>)> = HashMap::new();
    let mut unmatched = Vec::new();
    {
        let mut calls = tx.prepare(
            "SELECT start_uuid, skill FROM skill_calls c
             WHERE substr(session_id, 1, 8) = ?2
               AND (skill = ?1 OR EXISTS (SELECT 1 FROM nested_skills n
                    WHERE n.start_uuid = c.start_uuid AND n.skill = ?1))",
        )?;
        for (index, entry) in entries.iter().enumerate() {
            let calls = calls
                .query_map(params![entry.skill, entry.session], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            if calls.is_empty() {
                unmatched.push(index);
            }
            for (start_uuid, skill) in calls {
                let (_, per_skill) = matched.entry(start_uuid).or_insert((skill, Vec::new()));
                per_skill.retain(|&other| entries[other].skill != entry.skill);
                per_skill.push(index);
            }
        }
    }
    let mut merged = Vec::new();
    for (start_uuid, (skill, per_skill)) in &matched {
        let winner = per_skill
            .iter()
            .copied()
            .find(|&index| entries[index].skill == *skill)
            .unwrap_or(*per_skill.last().unwrap());
        let added: Vec<String> = per_skill
            .iter()
            .filter(|&&index| index != winner)
            .map(|&index| format!("{}: {}", entries[index].skill, entries[index].label()))
            .collect();
        let added = added.join("; ");
        // A nested skill's entry that sets the status keeps its skill name, like the added ones.
        let own = match &entries[winner] {
            entry if entry.skill == *skill => entry.finding.clone(),
            entry if entry.finding.is_empty() => format!("{}: {}", entry.skill, entry.status),
            entry => format!("{}: {}", entry.skill, entry.finding),
        };
        let finding = [own.as_str(), added.as_str()]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("; ");
        if !added.is_empty() {
            merged.push((start_uuid.clone(), added));
        }
        tx.execute(
            "UPDATE skill_calls SET status = ?2, finding = ?3 WHERE start_uuid = ?1",
            params![start_uuid, entries[winner].status, finding],
        )?;
    }
    merged.sort();
    let discarded = tx.execute(
        "UPDATE skill_calls SET status = 'verworfen', finding = 'vor Umstellung'
         WHERE status IS NULL",
        [],
    )?;
    tx.commit()?;
    Ok(Imported {
        unmatched,
        set: matched.len(),
        merged,
        discarded,
    })
}

/// One skill call as the report counts it. `None` where the row's parser did not measure it.
pub struct ReportCall {
    pub skill: String,
    /// The window of another skill's call that loaded `skill`, which never had a call of its own.
    pub nested: bool,
    /// `slash` or `model`; for a nested row, that of the call whose window loaded it. `None` on
    /// a row an older parser wrote whose transcript is gone, so it was never re-extracted.
    pub start_kind: Option<String>,
    /// A commit, `uncommitted`, or `None` for a skill the repo never had; for a nested row, the
    /// version of its first load in the window.
    pub skill_version: Option<String>,
    /// ISO week of the call's start, `2026-W41`; for a nested row, of the call whose window
    /// loaded it.
    /// `None` when the start time does not parse.
    pub week: Option<String>,
    /// The Julian day number of that week's Monday, so weeks sit on a time axis.
    pub week_monday: Option<i64>,
    pub corrected: Option<bool>,
    pub interrupted: Option<bool>,
    pub denied: Option<bool>,
    pub tool_error: Option<bool>,
    pub bare_question: Option<bool>,
    pub questions: Option<i64>,
    pub recommended_answers: Option<i64>,
    /// Input, output, cache read and cache write summed.
    pub tokens: Option<i64>,
    pub active_ms: Option<i64>,
}

/// Correction flags came with parser version 7; an older row has none, not zero.
const FLAGS_SINCE_PARSER: i64 = 7;

/// Every skill call, for the report. Then, per skill that only ever ran nested, each window
/// that loaded it, once however often it loaded there. Those are no skill calls: a view other
/// than the overview filters `nested` out, or it counts their windows twice.
pub fn report_calls(conn: &Connection) -> rusqlite::Result<Vec<ReportCall>> {
    let mut stmt = conn.prepare(
        "WITH measured AS (
             SELECT start_uuid, skill, start_kind, skill_version,
                 strftime('%G-W%V', started_at) AS week,
                 CAST(julianday(started_at, 'start of day', 'weekday 0', '-6 days') AS INTEGER)
                     AS week_monday,
                 CASE WHEN parser_version >= ?1 THEN EXISTS (SELECT 1 FROM user_texts u
                     WHERE u.start_uuid = c.start_uuid AND u.correction) END AS corrected,
                 interrupts > 0 AS interrupted,
                 -- An older parser wrote no denials: NULL, not 0.
                 CASE WHEN tool_calls IS NOT NULL THEN
                     EXISTS (SELECT 1 FROM denials d
                         WHERE d.start_uuid = c.start_uuid AND d.count > 0)
                 END AS denied,
                 tool_errors > 0 AS tool_error,
                 bare_questions > 0 AS bare_question,
                 questions, recommended_answers,
                 input_tokens + output_tokens + cache_read_tokens + cache_write_tokens AS tokens,
                 active_ms
             FROM skill_calls c
         ),
         -- SQLite takes the bare skill_version from the row with the lowest position.
         nested_only AS (
             SELECT start_uuid, skill, skill_version, MIN(position) FROM nested_skills
             WHERE skill NOT IN (SELECT skill FROM skill_calls)
             GROUP BY start_uuid, skill
         )
         SELECT skill, 0, start_kind, skill_version, week, week_monday,
             corrected, interrupted, denied, tool_error, bare_question,
             questions, recommended_answers, tokens, active_ms
         FROM measured
         UNION ALL
         SELECT n.skill, 1, m.start_kind, n.skill_version, week,
             week_monday, corrected, interrupted, denied,
             tool_error, bare_question, questions, recommended_answers, tokens, active_ms
         FROM nested_only n JOIN measured m ON m.start_uuid = n.start_uuid",
    )?;
    let calls = stmt.query_map([FLAGS_SINCE_PARSER], |row| {
        Ok(ReportCall {
            skill: row.get(0)?,
            nested: row.get(1)?,
            start_kind: row.get(2)?,
            skill_version: row.get(3)?,
            week: row.get(4)?,
            week_monday: row.get(5)?,
            corrected: row.get(6)?,
            interrupted: row.get(7)?,
            denied: row.get(8)?,
            tool_error: row.get(9)?,
            bare_question: row.get(10)?,
            questions: row.get(11)?,
            recommended_answers: row.get(12)?,
            tokens: row.get(13)?,
            active_ms: row.get(14)?,
        })
    })?;
    calls.collect()
}
