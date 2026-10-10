//! The binary end to end against fixture transcripts and a temporary database.

use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::process::Command;

const SESSION: &str = include_str!("fixtures/session.jsonl");
const NESTING: &str = include_str!("fixtures/nesting.jsonl");
const SUBAGENT: &str = include_str!("fixtures/nesting-subagent.jsonl");

fn run(projects: &Path, db: &Path) {
    let output = Command::new(env!("CARGO_BIN_EXE_skill-metrics"))
        .arg("--projects-dir")
        .arg(projects)
        .arg("--db")
        .arg(db)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Runs the binary with `--db <db>` and `args`; returns exit success and stdout.
fn cli(args: &[&str], db: &Path) -> (bool, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_skill-metrics"))
        .arg("--db")
        .arg(db)
        .args(args)
        .output()
        .unwrap();
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

/// Runs the binary like `cli`, expects it to fail and returns stderr.
fn cli_error(args: &[&str], db: &Path) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_skill-metrics"))
        .arg("--db")
        .arg(db)
        .args(args)
        .output()
        .unwrap();
    assert!(!output.status.success());
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The start uuid of each line `open` printed.
fn listed(stdout: &str) -> Vec<&str> {
    stdout
        .lines()
        .filter_map(|line| line.split('\t').nth(1))
        .collect()
}

/// (start uuid, skill, session, end time, user prompts), ordered by start.
fn rows(db: &Path) -> Vec<(String, String, String, String, i64)> {
    let conn = Connection::open(db).unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT start_uuid, skill, session_id, ended_at, user_prompts FROM skill_calls
             ORDER BY started_at",
        )
        .unwrap();
    stmt.query_map([], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
    })
    .unwrap()
    .map(Result::unwrap)
    .collect()
}

#[test]
fn reruns_add_no_rows_and_keep_rows_of_deleted_transcripts() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("out/metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s1.jsonl");
    let subagent = projects.join("-home-x-repo/s1/subagents/agent-a1.jsonl");
    fs::create_dir_all(subagent.parent().unwrap()).unwrap();
    fs::write(&session, SESSION).unwrap();
    // Subagent transcripts are not sessions, even when they hold a skill command.
    fs::write(&subagent, SESSION.replace("\"u", "\"sub-u")).unwrap();

    run(&projects, &db);
    let first = rows(&db);
    assert_eq!(
        first
            .iter()
            .map(|r| (r.0.as_str(), r.1.as_str(), r.2.as_str()))
            .collect::<Vec<_>>(),
        [
            ("u02", "cj-review-loop", "s1"),
            ("u12", "cj-ci-triage", "s1"),
            ("u18", "unslop", "s1")
        ]
    );

    run(&projects, &db);
    assert_eq!(rows(&db), first);

    fs::remove_file(&session).unwrap();
    run(&projects, &db);
    assert_eq!(rows(&db), first);

    let conn = Connection::open(&db).unwrap();
    let version: i64 = conn
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    let parser: i64 = conn
        .query_row("SELECT DISTINCT parser_version FROM skill_calls", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!((version, parser), (9, 9));
}

#[test]
fn running_session_row_is_replaced_on_next_run() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s1.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    let lines: Vec<&str> = SESSION.lines().collect();
    // Cut inside line 15, as a session still being written.
    fs::write(
        &session,
        format!("{}\n{}", lines[..14].join("\n"), &lines[14][..40]),
    )
    .unwrap();

    run(&projects, &db);
    let running = &rows(&db)[1];
    assert_eq!(
        (running.3.as_str(), running.4),
        ("2026-10-01T10:10:00.000Z", 0)
    );

    fs::write(&session, SESSION).unwrap();
    run(&projects, &db);
    let done = rows(&db);
    assert_eq!(done.len(), 3);
    assert_eq!(
        (done[1].3.as_str(), done[1].4),
        ("2026-10-01T10:12:00.000Z", 1)
    );
}

#[test]
fn rows_of_an_older_parser_version_are_replaced() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s1.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, SESSION).unwrap();
    run(&projects, &db);
    let fresh = rows(&db);

    Connection::open(&db)
        .unwrap()
        .execute(
            "UPDATE skill_calls SET user_prompts = 99, ended_at = 'x', parser_version = 0",
            [],
        )
        .unwrap();
    run(&projects, &db);
    assert_eq!(rows(&db), fresh);
}

#[test]
fn rows_of_a_newer_parser_version_are_kept() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s1.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, SESSION).unwrap();
    run(&projects, &db);
    Connection::open(&db)
        .unwrap()
        .execute(
            "UPDATE skill_calls SET user_prompts = 99, parser_version = 99",
            [],
        )
        .unwrap();
    let newer = rows(&db);

    run(&projects, &db);
    assert_eq!(rows(&db), newer);
}

#[test]
fn tokens_duration_context_and_models_are_stored() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s3.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, include_str!("fixtures/usage.jsonl")).unwrap();
    run(&projects, &db);
    run(&projects, &db);

    let conn = Connection::open(&db).unwrap();
    let row: (i64, i64, i64, i64, i64, String, String, String, String) = conn
        .query_row(
            "SELECT input_tokens, output_tokens, cache_read_tokens, cache_write_tokens, active_ms,
                    claude_code_version, entrypoint, cwd, git_branch
             FROM skill_calls WHERE start_uuid = 'p01'",
            [],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                    r.get(7)?,
                    r.get(8)?,
                ))
            },
        )
        .unwrap();
    assert_eq!(
        row,
        (
            16,
            27,
            108,
            14,
            480_000,
            "2.1.290".into(),
            "cli".into(),
            "/home/x/repo".into(),
            "main".into()
        )
    );
    let mut stmt = conn
        .prepare("SELECT start_uuid, model FROM models ORDER BY start_uuid, model")
        .unwrap();
    let models: Vec<(String, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    assert_eq!(
        models,
        [
            ("p01".into(), "claude-opus-5-5".into()),
            ("p01".into(), "claude-sonnet-5-5".into()),
            ("p09".into(), "claude-opus-5-5".into())
        ]
    );
}

#[test]
fn tools_denials_questions_and_prs_are_stored() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s4.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, include_str!("fixtures/tools.jsonl")).unwrap();
    run(&projects, &db);
    run(&projects, &db);

    let conn = Connection::open(&db).unwrap();
    let counts: Vec<Vec<i64>> = conn
        .prepare(
            "SELECT tool_calls, tool_errors, interrupts, questions, recommended_answers,
                    own_word_answers, subagents, commits, pushes, prs_created
             FROM skill_calls ORDER BY started_at",
        )
        .unwrap()
        .query_map([], |r| (0..10).map(|i| r.get(i)).collect())
        .unwrap()
        .map(Result::unwrap)
        .collect();
    assert_eq!(
        counts,
        [
            vec![10, 1, 1, 4, 1, 1, 1, 1, 1, 1],
            vec![3, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        ]
    );
    let pairs = |sql: &str| -> Vec<(String, String)> {
        conn.prepare(sql)
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect()
    };
    assert_eq!(
        pairs("SELECT start_uuid, kind || '=' || count FROM denials ORDER BY start_uuid, kind"),
        [
            ("x01".into(), "cancelled=1".into()),
            ("x01".into(), "permission-rule=1".into()),
            ("x01".into(), "user-rejected=1".into())
        ]
    );
    assert_eq!(
        pairs("SELECT start_uuid, url FROM pull_requests ORDER BY start_uuid, url"),
        [
            ("x01".into(), "https://github.com/o/r/pull/7".into()),
            ("x24".into(), "https://github.com/o/r/pull/7".into()),
            ("x24".into(), "https://github.com/o/r/pull/8".into())
        ]
    );
}

#[test]
fn user_texts_are_stored_redacted_and_replaced_on_rerun() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s5.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    let fixture = include_str!("fixtures/user-text.jsonl");
    fs::write(&session, fixture).unwrap();
    run(&projects, &db);
    run(&projects, &db);

    let texts = || -> Vec<(String, String)> {
        Connection::open(&db)
            .unwrap()
            .prepare("SELECT kind, text FROM user_texts WHERE start_uuid = 't01' ORDER BY position")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect()
    };
    // No text from the skill, the agent or a tool, none for denials without a reason, the
    // "clarify these questions" denial, the recommended option or the dismissed question.
    assert_eq!(
        texts(),
        [
            (
                "prompt".into(),
                "Use token [redacted] and [redacted] here, key [redacted].".into()
            ),
            (
                "denial reason".into(),
                "Not this file, edit b.rs instead".into()
            ),
            ("own-word answer".into(), "After CI is green".into()),
            // The picked options of a multi-select answer are no own words.
            ("own-word answer".into(), "and check the logs".into()),
        ]
    );

    fs::write(&session, fixture.replace("edit b.rs instead", "edit c.rs")).unwrap();
    run(&projects, &db);
    let rerun = texts();
    assert_eq!(rerun.len(), 4);
    assert_eq!(rerun[1].1, "Not this file, edit c.rs");
}

#[test]
fn user_texts_carry_correction_flags_and_queued_prompts_count() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s6.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, include_str!("fixtures/corrections.jsonl")).unwrap();
    run(&projects, &db);

    let texts: Vec<(String, String, bool, bool)> = Connection::open(&db)
        .unwrap()
        .prepare(
            "SELECT kind, text, correction, strong FROM user_texts WHERE start_uuid = 'c01'
             ORDER BY position",
        )
        .unwrap()
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    // The queued task notifications and the queued agent message are no user text.
    assert_eq!(
        texts,
        [
            ("prompt".into(), "NEIN, nicht so".into(), true, false),
            // A strong pattern alone is a correction, as in the old hook.
            (
                "prompt".into(),
                "mach das bitte immer zuerst mit Tests".into(),
                true,
                true
            ),
            // "no" and "nein" only as whole words.
            (
                "prompt".into(),
                "keine Einwände, nobody knows, sieht gut aus".into(),
                false,
                false
            ),
            ("prompt".into(), "stop, wrong file".into(), true, false),
        ]
    );
    assert_eq!(rows(&db)[0].4, 4);
}

#[test]
fn rows_an_older_parser_wrote_read_null_for_new_metrics() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    fs::create_dir_all(&projects).unwrap();
    // A database the first build wrote; the row's transcript is gone since.
    let conn = Connection::open(&db).unwrap();
    conn.execute_batch(
        "CREATE TABLE skill_calls (
            start_uuid TEXT PRIMARY KEY, skill TEXT NOT NULL, session_id TEXT NOT NULL,
            started_at TEXT NOT NULL, ended_at TEXT NOT NULL, user_prompts INTEGER NOT NULL,
            parser_version INTEGER NOT NULL);
         INSERT INTO skill_calls VALUES ('u02', 'cj-review-loop', 's1', 'a', 'b', 2, 1);
         PRAGMA user_version = 1;",
    )
    .unwrap();
    drop(conn);

    run(&projects, &db);
    let (tokens, active, tools): (Option<i64>, Option<i64>, Option<i64>) = Connection::open(&db)
        .unwrap()
        .query_row(
            "SELECT input_tokens, active_ms, tool_calls FROM skill_calls WHERE start_uuid = 'u02'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap();
    assert_eq!((tokens, active, tools), (None, None, None));

    // A user text an older parser stored holds no flags: not measured, not "no correction".
    let conn = Connection::open(&db).unwrap();
    conn.execute(
        "INSERT INTO user_texts (start_uuid, position, kind, text) VALUES ('u02', 0, 'prompt', 'nein')",
        [],
    )
    .unwrap();
    let flags: (Option<bool>, Option<bool>) = conn
        .query_row("SELECT correction, strong FROM user_texts", [], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .unwrap();
    assert_eq!(flags, (None, None));
}

#[test]
fn a_database_newer_than_the_build_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    fs::create_dir_all(&projects).unwrap();
    Connection::open(&db)
        .unwrap()
        .pragma_update(None, "user_version", 99)
        .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_skill-metrics"))
        .args(["--projects-dir".as_ref(), projects.as_os_str()])
        .args(["--db".as_ref(), db.as_os_str()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("newer than this build"));
}

#[cfg(unix)]
#[test]
fn an_unreadable_transcript_is_skipped() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s1.jsonl");
    let unreadable = projects.join("-home-x-repo/s0.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, SESSION).unwrap();
    fs::write(&unreadable, "").unwrap();
    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o000)).unwrap();

    run(&projects, &db);
    assert_eq!(rows(&db).len(), 3);
}

/// (skill call start, nested skill), in load order.
fn nested(db: &Path) -> Vec<(String, String)> {
    let conn = Connection::open(db).unwrap();
    let mut stmt = conn
        .prepare("SELECT start_uuid, skill FROM nested_skills ORDER BY start_uuid, position")
        .unwrap();
    stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect()
}

#[test]
fn nested_skills_are_stored_once_per_rerun() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s2.jsonl");
    let subagent = projects.join("-home-x-repo/s2/subagents/agent-a1.jsonl");
    fs::create_dir_all(subagent.parent().unwrap()).unwrap();
    fs::write(&session, NESTING).unwrap();
    fs::write(&subagent, SUBAGENT).unwrap();

    run(&projects, &db);
    let expected: Vec<(String, String)> = [
        "cj-reviewing",
        "cj-filing-issues",
        "cj-project-context",
        "unslop",
    ]
    .iter()
    .map(|skill| ("n01".to_owned(), skill.to_string()))
    .collect();
    assert_eq!(nested(&db), expected);
    run(&projects, &db);
    assert_eq!(nested(&db), expected);
}

#[test]
fn each_skill_call_stores_its_start_kind() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s1.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, SESSION).unwrap();

    run(&projects, &db);
    let conn = Connection::open(&db).unwrap();
    let mut stmt = conn
        .prepare("SELECT start_uuid, start_kind FROM skill_calls ORDER BY started_at")
        .unwrap();
    let kinds: Vec<(String, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    let kinds: Vec<(&str, &str)> = kinds
        .iter()
        .map(|(uuid, kind)| (uuid.as_str(), kind.as_str()))
        .collect();
    // u02 and u12 are slash commands, u18 is the model's Skill tool call outside any window.
    assert_eq!(
        kinds,
        [("u02", "slash"), ("u12", "slash"), ("u18", "model")]
    );
}

#[test]
fn a_fork_counts_once_with_the_latest_window_end_in_any_file_order() {
    let lines: Vec<&str> = SESSION.lines().collect();
    // The fork copies the session up to the first entry of the cj-ci-triage window.
    let fork = lines[..14].join("\n");
    for (first, second) in [(SESSION, fork.as_str()), (fork.as_str(), SESSION)] {
        let dir = tempfile::tempdir().unwrap();
        let (projects, db) = (
            dir.path().join("projects"),
            dir.path().join("metrics.sqlite"),
        );
        let (a, b) = (
            projects.join("-home-x-repo/a.jsonl"),
            projects.join("-home-x-repo/b.jsonl"),
        );
        fs::create_dir_all(a.parent().unwrap()).unwrap();
        fs::write(&a, first).unwrap();
        fs::write(&b, second).unwrap();

        run(&projects, &db);
        let longest = if first == SESSION { "a" } else { "b" };
        let triage = rows(&db).into_iter().find(|r| r.0 == "u12").unwrap();
        assert_eq!(
            (triage.2.as_str(), triage.3.as_str()),
            (longest, "2026-10-01T10:12:00.000Z")
        );
        assert_eq!(rows(&db).len(), 3);

        // The longer copy is deleted; the shorter one left does not shrink the stored window.
        fs::remove_file(projects.join(format!("-home-x-repo/{longest}.jsonl"))).unwrap();
        run(&projects, &db);
        let kept = rows(&db).into_iter().find(|r| r.0 == "u12").unwrap();
        assert_eq!(kept, triage);
    }
}

#[test]
fn skill_versions_are_stored() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s2.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, NESTING).unwrap();
    run(&projects, &db);

    // Against the crate's own repo, so this needs a git checkout of it: it has these skills, but
    // no revision with the fixtures' texts, and unslop is no skill of the repo.
    let conn = Connection::open(&db).unwrap();
    let versions = |sql: &str| -> Vec<(String, Option<String>)> {
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect()
    };
    let uncommitted = Some("uncommitted".to_owned());
    assert_eq!(
        versions("SELECT skill, skill_version FROM skill_calls ORDER BY started_at"),
        [
            ("cj-review-loop".to_owned(), uncommitted.clone()),
            ("cj-ci-triage".to_owned(), uncommitted.clone())
        ]
    );
    assert_eq!(
        versions("SELECT skill, skill_version FROM nested_skills ORDER BY start_uuid, position"),
        [
            ("cj-reviewing".to_owned(), uncommitted.clone()),
            ("cj-project-context".to_owned(), uncommitted),
            ("unslop".to_owned(), None)
        ]
    );
}

#[test]
fn open_lists_calls_without_status_and_status_sets_one_that_reruns_keep() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let repo = projects.join("-home-x-repo");
    fs::create_dir_all(repo.join("s2/subagents")).unwrap();
    fs::write(repo.join("s1.jsonl"), SESSION).unwrap();
    fs::write(repo.join("s2.jsonl"), NESTING).unwrap();
    fs::write(repo.join("s2/subagents/agent-a1.jsonl"), SUBAGENT).unwrap();
    run(&projects, &db);

    let (ok, all) = cli(&["open"], &db);
    assert!(ok);
    assert_eq!(listed(&all), ["u02", "u12", "u18", "n01", "n08a"]);
    let u02 = all.lines().next().unwrap();
    assert_eq!(
        u02,
        "offen\tu02\tcj-review-loop\ts1\t2026-10-01T10:01:00.000Z\t\
         prompts=2 interrupts=1 denials=0 tool_errors=0 questions=0 own_word_answers=0"
    );
    // unslop's own call and the call it was nested in.
    assert_eq!(listed(&cli(&["open", "unslop"], &db).1), ["u18", "n01"]);

    let (ok, _) = cli(
        &["status", "u02", "beobachtet", "asked twice for the base"],
        &db,
    );
    assert!(ok);
    assert_eq!(
        listed(&cli(&["open"], &db).1),
        ["u12", "u18", "n01", "n08a"]
    );
    run(&projects, &db);
    let stored: (String, String) = Connection::open(&db)
        .unwrap()
        .query_row(
            "SELECT status, finding FROM skill_calls WHERE start_uuid = 'u02'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(
        stored,
        ("beobachtet".into(), "asked twice for the base".into())
    );
    assert_eq!(
        listed(&cli(&["open"], &db).1),
        ["u12", "u18", "n01", "n08a"]
    );

    // Refused: an unknown status, an unknown call, a finding over two lines, a missing database.
    assert!(!cli(&["status", "u12", "erledigt", "x"], &db).0);
    assert!(!cli(&["status", "nope", "verworfen", "x"], &db).0);
    assert!(!cli(&["status", "u12", "verworfen", "a\nb"], &db).0);
    assert!(!cli(&["status", "u12", "verworfen", " "], &db).0);
    let conn = Connection::open(&db).unwrap();
    let bypass = "UPDATE skill_calls SET status = 'erledigt' WHERE start_uuid = 'u12'";
    assert!(conn.execute(bypass, []).is_err());
    drop(conn);
    let missing = dir.path().join("typo.sqlite");
    assert!(!cli(&["open"], &missing).0);
    assert!(!missing.exists());
    assert_eq!(
        listed(&cli(&["open"], &db).1),
        ["u12", "u18", "n01", "n08a"]
    );

    // After `--` a finding may start with `--`; `offen` lists the call again.
    assert!(cli(&["status", "--", "u12", "verworfen", "--db was wrong"], &db).0);
    assert_eq!(listed(&cli(&["open"], &db).1), ["u18", "n01", "n08a"]);
    assert!(cli(&["status", "u02", "offen", ""], &db).0);
    assert_eq!(
        listed(&cli(&["open"], &db).1),
        ["u02", "u18", "n01", "n08a"]
    );

    // `closed` lists the calls with a status and their finding, nested ones too.
    assert!(cli(&["status", "n01", "beobachtet", "unslop: kein Signal"], &db).0);
    let (ok, closed) = cli(&["closed"], &db);
    assert!(ok);
    assert_eq!(listed(&closed), ["u12", "n01"]);
    assert_eq!(
        closed.lines().next().unwrap(),
        "verworfen\tu12\tcj-ci-triage\ts1\t2026-10-01T10:10:00.000Z\t--db was wrong"
    );
    assert_eq!(listed(&cli(&["closed", "unslop"], &db).1), ["n01"]);
}

#[test]
fn session_extracts_only_that_session_and_keeps_the_other_rows() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let repo = projects.join("-home-x-repo");
    fs::create_dir_all(repo.join("s2/subagents")).unwrap();
    fs::write(repo.join("s1.jsonl"), SESSION).unwrap();
    fs::write(repo.join("s2.jsonl"), NESTING).unwrap();
    fs::write(repo.join("s2/subagents/agent-a1.jsonl"), SUBAGENT).unwrap();
    let projects_arg = projects.to_str().unwrap();

    let (ok, _) = cli(&["--projects-dir", projects_arg, "--session", "s2"], &db);
    assert!(ok);
    let sessions: Vec<String> = rows(&db).into_iter().map(|r| r.2).collect();
    assert_eq!(sessions, ["s2", "s2"]);
    // The subagent transcript of the session counts, as in a full run.
    assert_eq!(nested(&db).len(), 4);

    run(&projects, &db);
    fs::remove_file(repo.join("s1.jsonl")).unwrap();
    let (ok, _) = cli(
        &["extract", "--projects-dir", projects_arg, "--session", "s2"],
        &db,
    );
    assert!(ok);
    assert_eq!(rows(&db).len(), 5);
    // An unknown session is an error, not a silent empty run.
    assert!(!cli(&["--projects-dir", projects_arg, "--session", "nope"], &db).0);
}

#[test]
fn full_scan_stores_killed_and_resumed_sessions_with_their_latest_window() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let repo = projects.join("-home-x-repo");
    fs::create_dir_all(repo.join("s2/subagents")).unwrap();
    let lines: Vec<&str> = SESSION.lines().collect();
    // s1 ended once and the hook extracted it; then it was resumed and went on.
    fs::write(repo.join("s1.jsonl"), lines[..14].join("\n")).unwrap();
    let projects_arg = projects.to_str().unwrap();
    assert!(cli(&["--projects-dir", projects_arg, "--session", "s1"], &db).0);
    assert_eq!(rows(&db)[1].3, "2026-10-01T10:10:00.000Z");
    fs::write(repo.join("s1.jsonl"), SESSION).unwrap();
    // s2 was killed, so no hook ran for it.
    fs::write(repo.join("s2.jsonl"), NESTING).unwrap();
    fs::write(repo.join("s2/subagents/agent-a1.jsonl"), SUBAGENT).unwrap();

    run(&projects, &db);
    let stored = rows(&db);
    let resumed = stored.iter().find(|r| r.0 == "u12").unwrap();
    assert_eq!(
        (resumed.3.as_str(), resumed.4),
        ("2026-10-01T10:12:00.000Z", 1)
    );
    assert_eq!(stored.iter().filter(|r| r.2 == "s2").count(), 2);
}

/// Extracts SESSION as session `aaaaaaaa-1` and NESTING as `bbbbbbbb-2`, writes `notes` as
/// `<file name>, text` into a notes folder and imports it. Returns exit success and stdout.
fn import(dir: &Path, notes: &[(&str, &str)]) -> (bool, String) {
    let (projects, db, folder) = (
        dir.join("projects"),
        dir.join("metrics.sqlite"),
        dir.join("notes"),
    );
    let repo = projects.join("-home-x-repo");
    fs::create_dir_all(repo.join("bbbbbbbb-2/subagents")).unwrap();
    fs::write(repo.join("aaaaaaaa-1.jsonl"), SESSION).unwrap();
    fs::write(repo.join("bbbbbbbb-2.jsonl"), NESTING).unwrap();
    fs::write(repo.join("bbbbbbbb-2/subagents/agent-a1.jsonl"), SUBAGENT).unwrap();
    run(&projects, &db);
    fs::create_dir_all(&folder).unwrap();
    for (name, text) in notes {
        fs::write(folder.join(name), text).unwrap();
    }
    cli(&["import-notes", folder.to_str().unwrap()], &db)
}

/// (start uuid, status, finding), ordered by start.
fn statuses(db: &Path) -> Vec<(String, String, String)> {
    let conn = Connection::open(db).unwrap();
    let mut stmt = conn
        .prepare("SELECT start_uuid, status, finding FROM skill_calls ORDER BY started_at")
        .unwrap();
    stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect()
}

#[test]
fn import_notes_moves_status_and_finding_onto_the_calls() {
    let dir = tempfile::tempdir().unwrap();
    let (ok, stdout) = import(
        dir.path(),
        &[
            (
                "cj-review-loop.md",
                "# Randnotizen: cj-review-loop\n\n\
                 ## 2026-10-01 11:00 · Session `aaaaaaaa` · 1× aufgerufen\n\
                 status: beobachtet (asked twice for the base)\n\
                 - Folge-Prompts ohne Korrektursignal: 2\n",
            ),
            (
                "unslop.md",
                "# Randnotizen: unslop\n\n\
                 ## 2026-10-01 11:00 · Session `aaaaaaaa` · 1× aufgerufen\n\
                 status: verworfen (noise)\n\
                 ## 2026-10-02 11:00 · Session `bbbbbbbb` · 1× aufgerufen\n\
                 status: umgesetzt (abc1234 (follow-up))\n",
            ),
            // A fork: two entries for the same session and skill, the later one wins.
            (
                "cj-ci-triage.md",
                "# Randnotizen: cj-ci-triage\n\n\
                 ## 2026-10-01 12:00 · Session `aaaaaaaa` · 1× aufgerufen\n\
                 status: verworfen (one-off)\n\
                 ## 2026-10-01 11:00 · Session `aaaaaaaa` · 1× aufgerufen\n\
                 status: offen\n",
            ),
            // The skill name comes from the heading, not the file name.
            (
                "superpowers_gone.md",
                "# Randnotizen: superpowers:gone\n\n\
                 ## 2026-09-01 11:00 · Session `cccccccc` · 1× aufgerufen\n\
                 status: offen\n",
            ),
        ],
    );
    assert!(ok, "{stdout}");
    let db = dir.path().join("metrics.sqlite");
    let expected = [
        ("u02", "beobachtet", "asked twice for the base"),
        ("u12", "verworfen", "one-off"),
        ("u18", "verworfen", "noise"),
        // unslop was only nested in n01.
        ("n01", "umgesetzt", "unslop: abc1234 (follow-up)"),
        // No entry matched.
        ("n08a", "verworfen", "vor Umstellung"),
    ]
    .map(|(a, b, c)| (a.to_owned(), b.to_owned(), c.to_owned()));
    assert_eq!(statuses(&db), expected);
    assert!(
        stdout.contains("unmatched: superpowers:gone cccccccc"),
        "{stdout}"
    );
    assert!(
        stdout.contains("cj-ci-triage aaaaaaaa: 2 entries, kept verworfen (one-off)"),
        "{stdout}"
    );
    assert_eq!(cli(&["open"], &db).1, "");
    // The notes stay; deleting them is a separate step.
    assert!(dir.path().join("notes/unslop.md").exists());
}

#[test]
fn import_notes_merges_entries_of_nested_skills_and_runs_once() {
    let dir = tempfile::tempdir().unwrap();
    let (ok, stdout) = import(
        dir.path(),
        &[
            (
                "cj-review-loop.md",
                "# Randnotizen: cj-review-loop\n\n\
                 ## 2026-10-02 09:00 · Session `bbbbbbbb` · 1× aufgerufen\n\
                 status: offen\n\
                 ## 2026-10-01 09:00 · Session `aaaaaaaa` · 1× aufgerufen\n\
                 status: erledigt\n\
                 ## 2026-10-01 09:10 · Session `aaaaaaaa` · 1× aufgerufen\n\
                 status: beobachtet\n\
                 ## 2026-10-01 09:20 · Session `aaaaaaaa` · 1× aufgerufen\n\
                 status: verworfen noise\n\
                 ## 2026-10-01 09:30 · Session `aaaaaaaa` · 1× aufgerufen\n",
            ),
            (
                "unslop.md",
                "# Randnotizen: unslop\n\n\
                 ## 2026-10-02 11:00 · Session `bbbbbbbb` · 1× aufgerufen\n\
                 status: umgesetzt (abc1234)\n",
            ),
            (
                "cj-reviewing.md",
                "# Randnotizen: cj-reviewing\n\n\
                 ## 2026-10-02 12:00 · Session `bbbbbbbb` · 1× aufgerufen\n\
                 status: verworfen (noise)\n",
            ),
        ],
    );
    assert!(ok, "{stdout}");
    let db = dir.path().join("metrics.sqlite");
    // n01's own entry sets the status; the nested skills' entries are kept in the finding.
    let n01 = statuses(&db).into_iter().find(|r| r.0 == "n01").unwrap();
    let added = "unslop: umgesetzt (abc1234); cj-reviewing: verworfen (noise)";
    assert_eq!((n01.1.as_str(), n01.2.as_str()), ("offen", added));
    assert!(
        stdout.contains(&format!("merged into n01: {added}")),
        "{stdout}"
    );
    for skipped in [
        "skipped: invalid status erledigt: cj-review-loop aaaaaaaa",
        "skipped: beobachtet without a finding: cj-review-loop aaaaaaaa",
        "skipped: unreadable \"status: verworfen noise\": cj-review-loop aaaaaaaa",
        "skipped: no status line: cj-review-loop aaaaaaaa",
    ] {
        assert!(stdout.contains(skipped), "{skipped} in {stdout}");
    }
    // The call with only skipped entries counts as unmatched by the import.
    let u02 = statuses(&db).into_iter().find(|r| r.0 == "u02").unwrap();
    assert_eq!(
        (u02.1.as_str(), u02.2.as_str()),
        ("verworfen", "vor Umstellung")
    );
    assert_eq!(listed(&cli(&["open"], &db).1), ["n01"]);

    // A second run would overwrite statuses set since and discard new calls: refused.
    let before = statuses(&db);
    let folder = dir.path().join("notes");
    assert!(!cli(&["import-notes", folder.to_str().unwrap()], &db).0);
    assert_eq!(statuses(&db), before);
    // The copy from before the first import has no status.
    let backup = dir.path().join("metrics.sqlite.before-import");
    let set: i64 = Connection::open(backup)
        .unwrap()
        .query_row(
            "SELECT count(*) FROM skill_calls WHERE status IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(set, 0);
}

#[test]
fn show_prints_user_texts_and_the_window_while_the_transcript_exists() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
    );
    let session = projects.join("-home-x-repo/s5.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    let fixture = include_str!("fixtures/user-text.jsonl").replace(
        "\"cat out.txt\"",
        "\"GH_TOKEN=ghp_abcdefghijklmnopqrstuvwxyz0123 cat /home/x/a/long/path/to/the/repo/src/main.rs\"",
    );
    fs::write(&session, &fixture).unwrap();
    run(&projects, &db);
    assert!(cli(&["status", "t01", "beobachtet", "asked for the token"], &db).0);
    let show = || {
        cli(
            &["show", "--projects-dir", projects.to_str().unwrap(), "t01"],
            &db,
        )
    };

    let (ok, out) = show();
    assert!(ok);
    let user_texts = "\
cj-implement t01 session s5 2026-10-05T10:01:00.000Z to 2026-10-05T10:18:00.000Z
status: beobachtet, asked for the token
user texts:
  prompt: Use token [redacted] and [redacted] here, key [redacted].
  denial reason [correction]: Not this file, edit b.rs instead
  own-word answer: After CI is green
  own-word answer: and check the logs
transcript:
";
    assert!(out.starts_with(user_texts), "{out}");
    let window = &out[user_texts.len()..];
    assert!(
        window.starts_with("  10:04:00 agent: AGENT TEXT never stored.\n"),
        "{out}"
    );
    assert!(window.contains("  10:05:00 tool Edit: {"), "{out}");
    // Keys are redacted, long paths are not.
    assert!(
        window.contains(
            "  10:09:00 tool Bash: {\"command\":\"GH_[redacted] cat /home/x/a/long/path/to/the/repo/src/main.rs\"}\n"
        ),
        "{out}"
    );
    // Tool output, skill text and the user's own prompts from the transcript are not printed.
    for absent in ["TOOL OUTPUT", "SKILL TEXT", "hunter2", "Not this file"] {
        assert!(!window.contains(absent), "{absent} in {out}");
    }

    fs::remove_file(&session).unwrap();
    let (ok, out) = show();
    assert!(ok);
    assert_eq!(
        out,
        user_texts.replace(
            "transcript:\n",
            "transcript deleted, only the user texts are left\n"
        )
    );

    // Both flags, and a transcript in which the call starts no window.
    let conn = Connection::open(&db).unwrap();
    conn.execute("UPDATE user_texts SET strong = 1 WHERE position = 1", [])
        .unwrap();
    drop(conn);
    fs::write(&session, fixture.replace("\"t01\"", "\"t00\"")).unwrap();
    let (ok, out) = show();
    assert!(ok);
    assert!(
        out.contains("  denial reason [correction, strong]: Not this file")
            && out.ends_with("transcript holds no window starting at t01\n"),
        "{out}"
    );
    fs::remove_file(&session).unwrap();

    // A row an older parser wrote has no user texts.
    let conn = Connection::open(&db).unwrap();
    conn.execute("DELETE FROM user_texts", []).unwrap();
    drop(conn);
    let (ok, out) = show();
    assert!(ok);
    assert!(
        out.contains("user texts: none stored\ntranscript deleted"),
        "{out}"
    );

    let projects_arg = projects.to_str().unwrap();
    let error = cli_error(&["show", "--projects-dir", projects_arg, "nope"], &db);
    assert!(error.contains("no skill call nope"), "{error}");
    // A mistyped folder is no deleted transcript.
    let missing = dir.path().join("typo");
    let error = cli_error(
        &["show", "--projects-dir", missing.to_str().unwrap(), "t01"],
        &db,
    );
    assert!(error.contains("no projects folder"), "{error}");
}

/// The overview's rows, one per `<tr>` of its body, as cell texts without tags.
fn overview(html: &str) -> Vec<Vec<String>> {
    let body = html
        .split_once("<tbody>")
        .and_then(|(_, rest)| rest.split_once("</tbody>"))
        .map_or("", |(body, _)| body);
    body.split("<tr")
        .skip(1)
        .map(|row| {
            row.split("</t")
                .filter_map(|cell| cell.rsplit_once('>').map(|(_, text)| text.to_owned()))
                .filter(|text| !text.trim().is_empty())
                .collect()
        })
        .collect()
}

#[test]
fn report_writes_the_overview_with_intervals_and_no_url() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db, out) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
        dir.path().join("out/report.html"),
    );
    let session = projects.join("-home-x-repo/s1.jsonl");
    fs::create_dir_all(session.parent().unwrap()).unwrap();
    fs::write(&session, SESSION).unwrap();
    fs::write(
        projects.join("-home-x-repo/s6.jsonl"),
        include_str!("fixtures/corrections.jsonl"),
    )
    .unwrap();
    run(&projects, &db);

    // Skill x: three measured calls, one interrupted, one with a denial, plus a row an older
    // parser wrote whose counters are NULL. Its questions: 2 of 4 recommended.
    let conn = Connection::open(&db).unwrap();
    conn.execute_batch(
        "INSERT INTO skill_calls (start_uuid, skill, session_id, started_at, ended_at,
             user_prompts, parser_version, input_tokens, output_tokens, cache_read_tokens,
             cache_write_tokens, active_ms, tool_calls, tool_errors, interrupts, questions,
             recommended_answers)
         VALUES
             ('x1', 'x', 's9', '2026-10-01T10:00:00Z', '2026-10-01T10:01:00Z', 1, 9,
              100, 10, 0, 0, 30000, 5, 0, 1, 3, 1),
             ('x2', 'x', 's9', '2026-10-01T11:00:00Z', '2026-10-01T11:01:00Z', 1, 9,
              200, 20, 0, 0, 120000, 5, 0, 0, 1, 1),
             ('x3', 'x', 's9', '2026-10-01T12:00:00Z', '2026-10-01T12:01:00Z', 1, 9,
              300, 30, 0, 0, 600000, 5, 0, 0, 0, 0),
             ('x4', 'x', 's9', '2026-10-01T13:00:00Z', '2026-10-01T13:01:00Z', 1, 2,
              NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
         INSERT INTO denials (start_uuid, kind, count) VALUES ('x2', 'permission', 2);",
    )
    .unwrap();

    let (ok, stdout) = cli(&["report", "--out", out.to_str().unwrap()], &db);
    assert!(ok);
    assert_eq!(stdout.trim(), out.to_str().unwrap());
    let html = fs::read_to_string(&out).unwrap();
    assert!(!html.contains("://") && !html.contains("src=") && !html.contains("href="));

    let rows = overview(&html);
    // Most calls first, equal counts by name.
    let skills: Vec<&str> = rows.iter().map(|row| row[0].as_str()).collect();
    assert_eq!(
        skills,
        [
            "x",
            "cj-ci-triage",
            "cj-implement",
            "cj-review-loop",
            "unslop"
        ]
    );
    // Calls, correction, interrupt, denial, tool error, recommended, median tokens, median active
    // duration. The NULL row counts as a call and stays out of every denominator.
    assert_eq!(
        rows[0][1..],
        [
            "4",
            "0 % (0–56 %)",
            "33 % (6–79 %)",
            "33 % (6–79 %)",
            "0 % (0–56 %)",
            "50 % (15–85 %)",
            "220",
            "2 min"
        ]
    );
    // The correction flag of the fixture's first prompt.
    assert_eq!(rows[2][2], "100 % (21–100 %)");
    // No questions at all.
    assert_eq!(rows[4][6], "–");
}

#[test]
fn report_of_an_empty_database_says_so() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db, out) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
        dir.path().join("report.html"),
    );
    fs::create_dir_all(&projects).unwrap();
    run(&projects, &db);
    let (ok, _) = cli(&["report", "--out", out.to_str().unwrap()], &db);
    assert!(ok);
    assert!(fs::read_to_string(&out)
        .unwrap()
        .contains("No skill calls yet."));
}

#[test]
fn report_gives_skills_only_ever_nested_their_own_row() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db, out) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
        dir.path().join("report.html"),
    );
    let folder = projects.join("-home-x-repo");
    fs::create_dir_all(&folder).unwrap();
    // cj-project-context is loaded a second time in the cj-review-loop window: lines 7 to 9
    // (its Skill call, result and text) again, with a new tool id, uuids and time.
    let lines: Vec<&str> = NESTING.lines().collect();
    let again: Vec<String> = lines[6..9]
        .iter()
        .map(|line| {
            line.replace("\"t2\"", "\"t2b\"")
                .replace("n05", "n05x")
                .replace("10:03:00", "10:03:30")
        })
        .collect();
    let nesting = [
        lines[..9].join("\n"),
        again.join("\n"),
        lines[9..].join("\n"),
    ]
    .join("\n");
    fs::write(folder.join("s2.jsonl"), nesting).unwrap();
    // unslop also has a call of its own here, so it keeps only that row.
    fs::write(folder.join("s1.jsonl"), SESSION).unwrap();
    run(&projects, &db);
    let conn = Connection::open(&db).unwrap();
    conn.execute(
        "UPDATE skill_calls SET interrupts = 1 WHERE start_uuid = 'n01'",
        [],
    )
    .unwrap();
    let nested_loads: i64 = conn
        .query_row(
            "SELECT count(*) FROM nested_skills WHERE skill = 'cj-project-context'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(nested_loads, 2);

    let (ok, _) = cli(&["report", "--out", out.to_str().unwrap()], &db);
    assert!(ok);
    let rows = overview(&fs::read_to_string(&out).unwrap());
    let calls_and_interrupts: Vec<(&str, &str, &str)> = rows
        .iter()
        .map(|row| (row[0].as_str(), row[1].as_str(), row[3].as_str()))
        .collect();
    assert_eq!(
        calls_and_interrupts,
        [
            ("cj-ci-triage", "2", "0 % (0–66 %)"),
            // n01 and u02 (interrupted in the fixture); the nested rows add nothing here.
            ("cj-review-loop", "2", "100 % (34–100 %)"),
            // Loaded twice in one window: one window.
            ("cj-project-context (nested)", "1", "100 % (21–100 %)"),
            ("cj-reviewing (nested)", "1", "100 % (21–100 %)"),
            ("unslop", "1", "0 % (0–79 %)"),
        ]
    );
}

/// The page's view for one start-kind setting: `all`, `slash` or `model`.
fn view<'a>(html: &'a str, kind: &str) -> &'a str {
    let start = format!("<section id=\"view-{kind}\"");
    html.split_once(start.as_str())
        .and_then(|(_, rest)| rest.split_once("</section>"))
        .map_or("", |(view, _)| view)
}

#[test]
fn report_switch_limits_every_number_to_one_start_kind() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db, out) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
        dir.path().join("report.html"),
    );
    let folder = projects.join("-home-x-repo");
    fs::create_dir_all(&folder).unwrap();
    // Slash: u02, u12, n01 (whose window loads the nested skills). Model: u18, n08a.
    fs::write(folder.join("s1.jsonl"), SESSION).unwrap();
    fs::write(folder.join("s2.jsonl"), NESTING).unwrap();
    run(&projects, &db);
    // A row an older parser wrote, its transcript gone: no start kind, counted only under all.
    Connection::open(&db)
        .unwrap()
        .execute_batch(
            "INSERT INTO skill_calls (start_uuid, skill, session_id, started_at, ended_at,
                 user_prompts, parser_version)
             VALUES ('old1', 'old', 's9', '2026-09-01T10:00:00Z', '2026-09-01T10:01:00Z', 1, 2);",
        )
        .unwrap();

    let (ok, _) = cli(&["report", "--out", out.to_str().unwrap()], &db);
    assert!(ok);
    let html = fs::read_to_string(&out).unwrap();
    assert!(html.contains("id=\"kind-all\" checked"));
    let calls = |kind: &str| -> Vec<(String, String, String)> {
        overview(view(&html, kind))
            .into_iter()
            .map(|row| (row[0].clone(), row[1].clone(), row[3].clone()))
            .collect()
    };
    let owned = |rows: &[(&str, &str, &str)]| -> Vec<(String, String, String)> {
        rows.iter()
            .map(|(a, b, c)| (a.to_string(), b.to_string(), c.to_string()))
            .collect()
    };
    assert_eq!(
        calls("all"),
        owned(&[
            ("cj-ci-triage", "2", "0 % (0–66 %)"),
            ("cj-review-loop", "2", "50 % (9–91 %)"),
            ("cj-project-context (nested)", "1", "0 % (0–79 %)"),
            ("cj-reviewing (nested)", "1", "0 % (0–79 %)"),
            ("old", "1", "–"),
            ("unslop", "1", "0 % (0–79 %)"),
        ])
    );
    // Nested rows follow their window's call; a skill filtered down to no calls keeps its row.
    assert_eq!(
        calls("slash"),
        owned(&[
            ("cj-review-loop", "2", "50 % (9–91 %)"),
            ("cj-ci-triage", "1", "0 % (0–79 %)"),
            ("cj-project-context (nested)", "1", "0 % (0–79 %)"),
            ("cj-reviewing (nested)", "1", "0 % (0–79 %)"),
            ("old", "0", "–"),
            ("unslop", "0", "–"),
        ])
    );
    assert_eq!(
        calls("model"),
        owned(&[
            ("cj-ci-triage", "1", "0 % (0–79 %)"),
            ("unslop", "1", "0 % (0–79 %)"),
            ("cj-project-context (nested)", "0", "–"),
            ("cj-review-loop", "0", "–"),
            ("cj-reviewing (nested)", "0", "–"),
            ("old", "0", "–"),
        ])
    );
}

/// Column heads and rows of `skill`'s version table in a view, cells with tags stripped.
fn version_table(view: &str, skill: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let start = format!("<summary>{skill} · ");
    let table = view
        .split_once(start.as_str())
        .and_then(|(_, rest)| rest.split_once("</details>"))
        .map_or("", |(table, _)| table);
    let text = |cell: &str| cell.replace("<br>", " ").trim().to_owned();
    let head = table
        .split_once("<thead>")
        .and_then(|(_, rest)| rest.split_once("</thead>"))
        .map_or("", |(head, _)| head);
    let heads = head
        .split("<th scope=\"col\">")
        .skip(2)
        .map(|cell| text(cell.split("</th>").next().unwrap()))
        .collect();
    (heads, overview(table))
}

#[test]
fn report_shows_each_skill_version_side_by_side() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db, out) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
        dir.path().join("report.html"),
    );
    let folder = projects.join("-home-x-repo");
    fs::create_dir_all(&folder).unwrap();
    fs::write(folder.join("s1.jsonl"), SESSION).unwrap();
    run(&projects, &db);

    // Skill x under HEAD~1 and twice under HEAD of the crate's repo (one interrupted), a commit
    // the repo does not have, an uncommitted edit and no version, inserted newest commit first.
    // Its window x4 loads `helper` twice, first under HEAD~1, then uncommitted.
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .unwrap();
        String::from_utf8(out.stdout).unwrap().trim().to_owned()
    };
    let (head, parent) = (git(&["rev-parse", "HEAD"]), git(&["rev-parse", "HEAD~1"]));
    let gone = "0123456789abcdef0123456789abcdef01234567";
    Connection::open(&db)
        .unwrap()
        .execute_batch(&format!(
            "INSERT INTO skill_calls (start_uuid, skill, session_id, started_at, ended_at,
                 user_prompts, parser_version, interrupts, start_kind, skill_version)
             VALUES
                 ('x1', 'x', 's9', '2026-10-01T10:00:00Z', '2026-10-01T10:01:00Z', 1, 9, 0,
                  'slash', NULL),
                 ('x2', 'x', 's9', '2026-10-01T11:00:00Z', '2026-10-01T11:01:00Z', 1, 9, 0,
                  'slash', 'uncommitted'),
                 ('x3', 'x', 's9', '2026-10-01T12:00:00Z', '2026-10-01T12:01:00Z', 1, 9, 0,
                  'slash', '{gone}'),
                 ('x4', 'x', 's9', '2026-10-01T13:00:00Z', '2026-10-01T13:01:00Z', 1, 9, 1,
                  'slash', '{head}'),
                 ('x5', 'x', 's9', '2026-10-01T14:00:00Z', '2026-10-01T14:01:00Z', 1, 9, 0,
                  'model', '{head}'),
                 ('x6', 'x', 's9', '2026-10-01T15:00:00Z', '2026-10-01T15:01:00Z', 1, 9, 0,
                  'slash', '{parent}');
             INSERT INTO nested_skills (start_uuid, position, skill, loaded_at, skill_version)
             VALUES ('x4', 1, 'helper', '2026-10-01T13:00:30Z', 'uncommitted'),
                 ('x4', 0, 'helper', '2026-10-01T13:00:10Z', '{parent}');"
        ))
        .unwrap();

    let (ok, _) = cli(&["report", "--out", out.to_str().unwrap()], &db);
    assert!(ok);
    let html = fs::read_to_string(&out).unwrap();
    let column = |commit: &str| {
        let day = git(&["show", "-s", "--format=%cs", commit]);
        format!("{} {day}", &commit[..7])
    };
    // Commit dates order the columns; equal times fall back to the hash.
    let mut dated = [parent.clone(), head.clone()];
    dated.sort_by_key(|commit| (git(&["show", "-s", "--format=%ct", commit]), commit.clone()));

    let (heads, rows) = version_table(view(&html, "all"), "x");
    assert_eq!(
        heads,
        [
            column(&dated[0]),
            column(&dated[1]),
            gone[..7].to_owned(),
            "uncommitted".to_owned(),
            "no version".to_owned()
        ]
    );
    let calls = |commit: &str| if *commit == head { "2" } else { "1" };
    assert_eq!(
        rows[0],
        ["Calls", calls(&dated[0]), calls(&dated[1]), "1", "1", "1"]
    );
    let head_column = 1 + dated.iter().position(|c| *c == head).unwrap();
    assert_eq!(rows[2][head_column], "50 % (9–91 %)");
    // The switch: model keeps only x5.
    let (heads, rows) = version_table(view(&html, "model"), "x");
    assert_eq!(heads, [column(&head)]);
    assert_eq!(rows[0], ["Calls", "1"]);
    // A nested-only skill takes the version of its first load in the window.
    let (heads, _) = version_table(view(&html, "all"), "helper (nested)");
    assert_eq!(heads, [column(&parent)]);
    // unslop is no skill of the repo: only the no-version column.
    let (heads, _) = version_table(view(&html, "all"), "unslop");
    assert_eq!(heads, ["no version"]);
}

/// Titles of the points in `skill`'s trend chart, in document order.
fn trend_points(view: &str, skill: &str) -> Vec<String> {
    let start = format!("<summary>{skill} · ");
    let details = view
        .split_once(start.as_str())
        .and_then(|(_, rest)| rest.split_once("</details>"))
        .map_or("", |(details, _)| details);
    details
        .split("<circle")
        .skip(1)
        .filter_map(|point| point.split_once("<title>"))
        .map(|(_, rest)| rest.split("</title>").next().unwrap().to_owned())
        .collect()
}

#[test]
fn report_charts_a_weekly_trend_per_skill_and_a_scatter_across_skills() {
    let dir = tempfile::tempdir().unwrap();
    let (projects, db, out) = (
        dir.path().join("projects"),
        dir.path().join("metrics.sqlite"),
        dir.path().join("report.html"),
    );
    let folder = projects.join("-home-x-repo");
    fs::create_dir_all(&folder).unwrap();
    fs::write(folder.join("s1.jsonl"), SESSION).unwrap();
    fs::write(folder.join("s2.jsonl"), NESTING).unwrap();
    run(&projects, &db);
    // Skill x in three ISO weeks: two calls in 2026-W40 (one interrupted), one in 2026-W41,
    // one on 2027-01-01, which is 2026-W53. The last one's parser measured no interrupts.
    Connection::open(&db)
        .unwrap()
        .execute_batch(
            "INSERT INTO skill_calls (start_uuid, skill, session_id, started_at, ended_at,
                 user_prompts, parser_version, interrupts, tool_calls, tool_errors, start_kind)
             VALUES
                 ('x1', 'x', 's9', '2026-09-28T10:00:00Z', '2026-09-28T10:01:00Z', 1, 9, 1, 1, 0,
                  'slash'),
                 ('x2', 'x', 's9', '2026-10-01T10:00:00Z', '2026-10-01T10:01:00Z', 1, 9, 0, 1, 0,
                  'slash'),
                 ('x3', 'x', 's9', '2026-10-06T10:00:00Z', '2026-10-06T10:01:00Z', 1, 9, 0, 1, 1,
                  'slash'),
                 ('x4', 'x', 's9', '2027-01-01T10:00:00Z', '2027-01-01T10:01:00Z', 1, 9, NULL,
                  1, 0, 'model'),
                 -- A start time that does not parse: counted in the tables, not in the trend.
                 ('y1', 'y', 's9', '10 o''clock', '10 o''clock', 1, 9, 0, 1, 0, 'slash');",
        )
        .unwrap();

    let (ok, _) = cli(&["report", "--out", out.to_str().unwrap()], &db);
    assert!(ok);
    let html = fs::read_to_string(&out).unwrap();
    assert!(!html.contains("://") && !html.contains("src=") && !html.contains("href="));

    let points = trend_points(view(&html, "all"), "x");
    let of = |rate: &str| -> Vec<&str> {
        points
            .iter()
            .filter(|p| p.starts_with(rate))
            .map(String::as_str)
            .collect()
    };
    assert_eq!(
        of("Interrupt"),
        [
            "Interrupt, 2026-W40: 50 % (9–91 %) of 2 calls",
            "Interrupt, 2026-W41: 0 % (0–79 %) of 1 call",
        ]
    );
    assert_eq!(of("Tool error").len(), 3);
    assert_eq!(
        of("Tool error")[2],
        "Tool error, 2026-W53: 0 % (0–79 %) of 1 call"
    );
    // Weeks on a time axis: W41 is one week after W40, W53 thirteen.
    let details = view(&html, "all").split("<summary>x · ").nth(1).unwrap();
    let details = details.split("</details>").next().unwrap();
    let xs: Vec<f64> = details
        .split("<circle class=\"s1\" cx=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap().parse().unwrap())
        .collect();
    assert_eq!(xs.len(), 3);
    assert!(((xs[1] - xs[0]) * 13.0 - (xs[2] - xs[0])).abs() < 0.5);
    assert!(trend_points(view(&html, "all"), "y").is_empty());
    assert_eq!(of("Correction").len(), 3);
    assert_eq!(of("Denial").len(), 3);
    // The switch: model keeps only the W53 call.
    assert_eq!(trend_points(view(&html, "model"), "x").len(), 3);

    // The scatter: one point per skill with calls of its own, nested rows left out, in
    // overview order; skills at the same point share it.
    let scatter = |kind: &str| -> Vec<String> {
        view(&html, kind)
            .split("<circle class=\"dot\"")
            .skip(1)
            .map(|p| {
                p.split_once("<title>")
                    .unwrap()
                    .1
                    .split(':')
                    .next()
                    .unwrap()
                    .to_owned()
            })
            .collect()
    };
    assert_eq!(
        scatter("all"),
        ["x", "cj-ci-triage, cj-review-loop", "unslop, y"]
    );
    assert_eq!(scatter("model"), ["cj-ci-triage, unslop, x"]);
}
