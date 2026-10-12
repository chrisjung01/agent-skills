//! Reads every Claude Code session transcript and upserts one row per skill call into SQLite, and
//! holds whetstone's status per skill call. `--session` also finds a Codex session (in
//! `--codex-dir`, default `$CODEX_HOME/sessions` or `~/.codex/sessions`).
//!
//! Usage:
//!   skill-metrics [extract] [--session <id>] [--projects-dir <dir>] [--codex-dir <dir>] [--db <file>]
//!   skill-metrics open [<skill>] [--db <file>]
//!   skill-metrics closed [<skill>] [--db <file>]
//!   skill-metrics status [--db <file>] [--] <start uuid> <status> <finding>
//!   skill-metrics show [--projects-dir <dir>] [--codex-dir <dir>] [--db <file>] <start uuid>
//!   skill-metrics import-notes [<notes dir>] [--db <file>]
//!   skill-metrics report [--out <file>] [--db <file>]
//!   skill-metrics hook    (PreToolUse hook: payload on stdin, blocks a bare choice question)
//!
//! Pipeline: transcript or codex (JSONL into events) → window (events into skill calls) → metrics (what
//! happened in each window) → version (git) → store (SQLite).
//! Everything between reading the files and writing the database is pure, except git in version.

mod codex;
mod metrics;
mod notes;
mod question;
mod report;
mod store;
mod transcript;
mod user_text;
mod version;
mod window;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, io, process::ExitCode};
use user_text::redact_keys;

/// Bump whenever what is extracted changes: rows of an older parser version are replaced.
const PARSER_VERSION: i64 = 10;

fn main() -> ExitCode {
    // Before any argument or database work: the hook exits 0 whatever happens.
    if env::args().skip(1).eq(["hook"]) {
        let mut payload = String::new();
        if io::Read::read_to_string(&mut io::stdin(), &mut payload).is_ok() {
            if let Some(decision) = question::hook(&payload) {
                println!("{decision}");
            }
        }
        return ExitCode::SUCCESS;
    }
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("skill-metrics: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let (mut projects, mut codex_dir, mut db, mut session, mut out, mut positional) =
        (None, None, None, None, None, Vec::new());
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        // After `--` everything is positional, so a finding may start with `--`.
        if arg == "--" {
            positional.extend(args.by_ref());
            break;
        }
        if !arg.starts_with("--") {
            positional.push(arg);
            continue;
        }
        let value = args.next().ok_or(format!("{arg} needs a value"))?;
        match arg.as_str() {
            "--projects-dir" => projects = Some(PathBuf::from(value)),
            "--codex-dir" => codex_dir = Some(PathBuf::from(value)),
            "--db" => db = Some(PathBuf::from(value)),
            "--session" => session = Some(value),
            "--out" => out = Some(PathBuf::from(value)),
            _ => return Err(format!("unknown argument {arg}").into()),
        }
    }
    let home = || env::var("HOME").map(PathBuf::from);
    let db = match db {
        Some(path) => path,
        None => home()?.join(".claude/whetstone/skill-metrics.sqlite"),
    };
    let projects = || match projects {
        Some(path) => Ok(path),
        None => home().map(|home| home.join(".claude/projects")),
    };
    let codex_dir = || match codex_dir {
        Some(path) => Ok(path),
        None => match env::var("CODEX_HOME") {
            Ok(codex_home) => Ok(PathBuf::from(codex_home).join("sessions")),
            Err(_) => home().map(|home| home.join(".codex/sessions")),
        },
    };
    let positional: Vec<&str> = positional.iter().map(String::as_str).collect();
    match positional.as_slice() {
        [] | ["extract"] => extract(&projects()?, &codex_dir()?, &db, session.as_deref()),
        ["open"] => open(&db, None),
        ["open", skill] => open(&db, Some(skill)),
        ["closed"] => closed(&db, None),
        ["closed", skill] => closed(&db, Some(skill)),
        ["status", start_uuid, status, finding] => set_status(&db, start_uuid, status, finding),
        ["show", start_uuid] => show(&db, &projects()?, &codex_dir()?, start_uuid),
        ["import-notes"] => import_notes(&db, &home()?.join(".claude/whetstone/notes")),
        ["import-notes", folder] => import_notes(&db, Path::new(folder)),
        ["report"] => {
            let out = match out {
                Some(path) => path,
                None => home()?.join(".claude/whetstone/report.html"),
            };
            report(&db, &out)
        }
        _ => Err(format!("unknown command {}", positional.join(" ")).into()),
    }
}

/// One line per open call: status, start uuid, skill, session, start time, counters.
fn open(db: &Path, skill: Option<&str>) -> Result<(), Box<dyn Error>> {
    for call in store::open_calls(&existing_db(db)?, skill)? {
        let counters: Vec<String> = call
            .counters
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect();
        println!(
            "offen\t{}\t{}\t{}\t{}\t{}",
            call.start_uuid,
            call.skill,
            call.session_id,
            call.started_at,
            counters.join(" ")
        );
    }
    Ok(())
}

/// One line per call with a status: status, start uuid, skill, session, start time, finding.
fn closed(db: &Path, skill: Option<&str>) -> Result<(), Box<dyn Error>> {
    for call in store::closed_calls(&existing_db(db)?, skill)? {
        println!("{}", call.join("\t"));
    }
    Ok(())
}

fn set_status(
    db: &Path,
    start_uuid: &str,
    status: &str,
    finding: &str,
) -> Result<(), Box<dyn Error>> {
    if !store::STATUSES.contains(&status) {
        let allowed = store::STATUSES.join(", ");
        return Err(format!("unknown status {status}, allowed: {allowed}").into());
    }
    if finding.contains('\n') {
        return Err("the finding is one line".into());
    }
    if status != "offen" && finding.trim().is_empty() {
        return Err(format!("status {status} needs a finding").into());
    }
    if !store::set_status(&existing_db(db)?, start_uuid, status, finding)? {
        return Err(format!("no skill call {start_uuid}").into());
    }
    Ok(())
}

/// The call's user texts from the database, then the agent text and tool calls of its window
/// from the transcript while it exists. Tool output is never printed. The transcript part gets
/// only the key and token patterns, so paths stay readable.
fn show(
    db: &Path,
    projects: &Path,
    codex_dir: &Path,
    start_uuid: &str,
) -> Result<(), Box<dyn Error>> {
    // A mistyped folder is an error, not a deleted transcript.
    if !projects.is_dir() {
        return Err(format!("no projects folder at {}", projects.display()).into());
    }
    let call = store::shown_call(&existing_db(db)?, start_uuid)?
        .ok_or(format!("no skill call {start_uuid}"))?;
    println!(
        "{} {start_uuid} session {} {} to {}",
        call.skill, call.session_id, call.started_at, call.ended_at
    );
    if let Some(status) = &call.status {
        println!(
            "status: {status}, {}",
            call.finding.as_deref().unwrap_or("")
        );
    }
    if call.user_texts.is_empty() {
        println!("user texts: none stored");
    } else {
        println!("user texts:");
    }
    for user_text in &call.user_texts {
        let flags: Vec<&str> = [
            (user_text.correction, "correction"),
            (user_text.strong, "strong"),
        ]
        .into_iter()
        .filter(|(set, _)| *set == Some(true))
        .map(|(_, name)| name)
        .collect();
        let flags = match flags.as_slice() {
            [] => String::new(),
            flags => format!(" [{}]", flags.join(", ")),
        };
        println!("  {}{flags}: {}", user_text.kind, indent(&user_text.text));
    }

    let claude_code = session_files(projects)?
        .into_iter()
        .find(|path| is_session(path, &call.session_id));
    let events = match claude_code {
        Some(path) => read(&path).map(|text| transcript::parse(&text)),
        None => codex_files(codex_dir)
            .into_iter()
            .find(|path| codex::session_id(path) == call.session_id)
            .and_then(|path| read(&path))
            .map(|text| codex::parse(&call.session_id, &text)),
    };
    let Some(events) = events else {
        println!("transcript deleted, only the user texts are left");
        return Ok(());
    };
    let window = window::skill_calls(&call.session_id, &events, &[])
        .into_iter()
        .find(|window| window.start_uuid == start_uuid);
    let Some(window) = window else {
        println!("transcript holds no window starting at {start_uuid}");
        return Ok(());
    };
    println!("transcript:");
    for event in &events[window.span] {
        let time = event.timestamp.get(11..19).unwrap_or(&event.timestamp);
        let line = match &event.kind {
            transcript::Kind::AgentText(text) => {
                format!("agent: {}", indent(&redact_keys(text)))
            }
            transcript::Kind::SkillTool { skill, .. } => format!("tool Skill: {skill}"),
            transcript::Kind::ToolUse { name, input, .. } => {
                // ponytail: a cut at a fixed length; a Write's whole content is rarely the point.
                let input: String = redact_keys(&input.to_string()).chars().take(300).collect();
                format!("tool {name}: {input}")
            }
            _ => continue,
        };
        println!("  {time} {line}");
    }
    Ok(())
}

/// Continuation lines indented under the first.
fn indent(text: &str) -> String {
    text.replace('\n', "\n    ")
}

/// Moves the old markdown notes' status and finding onto the skill calls, once; the notes stay.
fn import_notes(db: &Path, folder: &Path) -> Result<(), Box<dyn Error>> {
    let notes = notes::read(folder)?;
    for problem in &notes.problems {
        println!("skipped: {problem}");
    }
    let entries = &notes.entries;
    let mut forks: Vec<(&str, &str)> = entries
        .iter()
        .map(|entry| (entry.skill.as_str(), entry.session.as_str()))
        .collect();
    forks.sort();
    forks.dedup();
    for (skill, session) in forks {
        let same: Vec<_> = entries
            .iter()
            .filter(|entry| entry.skill == skill && entry.session == session)
            .collect();
        if let [_, .., last] = same.as_slice() {
            let count = same.len();
            println!(
                "fork: {skill} {session}: {count} entries, kept {}",
                last.label()
            );
        }
    }

    // The import is one-way: a copy to go back to after checking its output.
    let backup = PathBuf::from(format!("{}.before-import", db.display()));
    let imported = store::import(&mut existing_db(db)?, entries, || {
        fs::copy(db, &backup).map(|_| ())
    })?;
    println!("database before the import: {}", backup.display());
    for (start_uuid, added) in &imported.merged {
        println!("merged into {start_uuid}: {added}");
    }
    for &index in &imported.unmatched {
        let entry = &entries[index];
        println!(
            "unmatched: {} {} {}",
            entry.skill,
            entry.session,
            entry.label()
        );
    }
    println!(
        "{} entries from {} files ({} skipped): {} skill calls set, {} merged, {} unmatched, \
         {} calls set to verworfen (vor Umstellung)",
        entries.len(),
        notes.files,
        notes.problems.len(),
        imported.set,
        imported.merged.len(),
        imported.unmatched.len(),
        imported.discarded
    );
    Ok(())
}

/// Writes the HTML report and prints its path.
fn report(db: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
    let calls = store::report_calls(&existing_db(db)?)?;
    let commits: std::collections::BTreeSet<&str> = calls
        .iter()
        .filter_map(|call| call.skill_version.as_deref())
        .filter(|&v| v != version::UNCOMMITTED)
        .collect();
    let dates = version::commit_dates(Path::new(env!("CARGO_MANIFEST_DIR")), commits);
    let page = report::html(&calls, &dates);
    if let Some(dir) = out.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(out, page)?;
    println!("{}", out.display());
    Ok(())
}

/// The database every command but `extract` reads; it must exist, so a mistyped `--db`
/// is not an empty list.
fn existing_db(db: &Path) -> Result<rusqlite::Connection, Box<dyn Error>> {
    if !db.is_file() {
        return Err(format!("no database at {}, run extract first", db.display()).into());
    }
    store::open(db)
}

fn open_db(db: &Path) -> Result<rusqlite::Connection, Box<dyn Error>> {
    if let Some(dir) = db.parent() {
        fs::create_dir_all(dir)?;
    }
    store::open(db)
}

/// Every Claude Code transcript, or with `session` only that one, a Codex session included; rows
/// of other sessions stay.
fn extract(
    projects: &Path,
    codex_dir: &Path,
    db: &Path,
    session: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut files = session_files(projects)?;
    let mut codex_sessions = Vec::new();
    if let Some(session) = session {
        files.retain(|path| is_session(path, session));
        codex_sessions = codex_files(codex_dir);
        codex_sessions.retain(|path| codex::session_id(path) == session);
        if files.is_empty() && codex_sessions.is_empty() {
            return Err(format!("no transcript for session {session}").into());
        }
    }
    let mut versions = version::Versions::in_repo_of(Path::new(env!("CARGO_MANIFEST_DIR")));
    let mut calls = Vec::new();
    for path in &files {
        let Some(text) = read(path) else {
            continue;
        };
        let subagents: Vec<_> = subagent_files(&path.with_extension(""))
            .iter()
            .filter_map(|file| read(file))
            .map(|text| transcript::parse_subagent(&text))
            .collect();
        let session_id = path.file_stem().unwrap_or_default().to_string_lossy();
        let events = transcript::parse(&text);
        let mut session_calls = window::skill_calls(&session_id, &events, &subagents);
        metrics::measure(&mut session_calls, &events);
        versions.fill(&mut session_calls);
        calls.extend(session_calls);
    }
    for path in &codex_sessions {
        let Some(text) = read(path) else {
            continue;
        };
        let session_id = codex::session_id(path);
        let events = codex::parse(&session_id, &text);
        let mut session_calls = window::skill_calls(&session_id, &events, &[]);
        metrics::measure(&mut session_calls, &events);
        versions.fill(&mut session_calls);
        for call in &mut session_calls {
            call.source = codex::SOURCE;
        }
        calls.extend(session_calls);
    }

    store::upsert(&mut open_db(db)?, &calls, PARSER_VERSION)?;
    println!(
        "{} skill calls extracted from {} transcripts into {}",
        calls.len(),
        files.len() + codex_sessions.len(),
        db.display()
    );
    Ok(())
}

/// `<sessions>/YYYY/MM/DD/rollout-*.jsonl`, at any depth; none when the folder is missing.
fn codex_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs = vec![dir.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for path in entries.flatten().map(|entry| entry.path()) {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if path.is_dir() {
                dirs.push(path);
            } else if name.starts_with("rollout-") && name.ends_with(".jsonl") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn is_session(path: &Path, session: &str) -> bool {
    path.file_stem().is_some_and(|stem| stem == session)
}

/// Claude Code's cleanup may delete a file mid-run; its stored rows stay.
fn read(path: &Path) -> Option<String> {
    match fs::read(path) {
        Ok(bytes) => Some(String::from_utf8_lossy(&bytes).into_owned()),
        Err(err) => {
            eprintln!("skill-metrics: skipped {}: {err}", path.display());
            None
        }
    }
}

/// `<session>/subagents/*.jsonl`; none when the folder is missing.
fn subagent_files(session_dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(session_dir.join("subagents")) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "jsonl"))
        .collect();
    files.sort();
    files
}

/// `<projects>/<project>/<session>.jsonl`. Subagent transcripts sit one level deeper
/// (`<session>/subagents/`) and are not sessions.
fn session_files(projects: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for project in fs::read_dir(projects)?.flatten() {
        let project = project.path();
        if !project.is_dir() {
            continue;
        }
        let Ok(entries) = fs::read_dir(&project) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "jsonl") {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}
