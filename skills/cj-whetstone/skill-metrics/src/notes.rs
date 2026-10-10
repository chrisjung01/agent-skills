//! The markdown notes the old SessionEnd hook wrote, `<notes>/<skill>.md`, read once by
//! `import-notes`.

use crate::store::STATUSES;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// One `## <date> · Session `<8 hex>`` entry with its `status:` line.
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub skill: String,
    /// The first 8 characters of the session id.
    pub session: String,
    /// `YYYY-MM-DD HH:MM` from the heading; orders a fork's entries.
    pub stamp: String,
    pub status: String,
    /// The text in the status's parentheses, empty without them.
    pub finding: String,
}

impl Entry {
    /// `status (finding)` as the notes wrote it, or `status` alone.
    pub fn label(&self) -> String {
        match self.finding.as_str() {
            "" => self.status.clone(),
            finding => format!("{} ({finding})", self.status),
        }
    }
}

/// A notes folder read.
pub struct Notes {
    pub files: usize,
    /// Ordered by stamp, file order for equal stamps: a later entry wins a fork.
    pub entries: Vec<Entry>,
    /// Entries that were skipped, one line each.
    pub problems: Vec<String>,
}

static HEADING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^## (\d{4}-\d{2}-\d{2}(?: \d{2}:\d{2})?)[^`]*Session `([0-9a-f]{8})`").unwrap()
});
static STATUS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^status: (\S+)(?: \((.*)\))?\s*$").unwrap());

/// Every `*.md` in `folder`.
pub fn read(folder: &Path) -> Result<Notes, Box<dyn Error>> {
    let mut files: Vec<PathBuf> = fs::read_dir(folder)
        .map_err(|err| format!("{}: {err}", folder.display()))?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect();
    files.sort();
    let (mut entries, mut problems) = (Vec::new(), Vec::new());
    for path in &files {
        let fallback = path.file_stem().unwrap_or_default().to_string_lossy();
        parse(
            &fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?,
            &fallback,
            &mut entries,
            &mut problems,
        );
    }
    // Stable, so equal stamps keep their file order.
    entries.sort_by(|a: &Entry, b| a.stamp.cmp(&b.stamp));
    Ok(Notes {
        files: files.len(),
        entries,
        problems,
    })
}

/// The entries of one notes file. The skill comes from the `# Randnotizen: <skill>` heading,
/// since the file name replaced characters like `:`; `fallback` when the heading is missing.
/// A heading without a valid status line goes to `problems`, as `status` would refuse it.
fn parse(text: &str, fallback: &str, entries: &mut Vec<Entry>, problems: &mut Vec<String>) {
    let skill = text
        .lines()
        .find_map(|line| line.strip_prefix("# Randnotizen: "))
        .map_or(fallback, str::trim);
    let mut heading: Option<(&str, &str)> = None;
    let mut skip = |heading: (&str, &str), why: &str| {
        let (_, session) = heading;
        problems.push(format!("{why}: {skill} {session}"));
    };
    for line in text.lines() {
        if let Some(caps) = HEADING.captures(line) {
            if let Some(open) = heading {
                skip(open, "no status line");
            }
            let (stamp, session) = (caps.get(1).unwrap(), caps.get(2).unwrap());
            heading = Some((stamp.as_str(), session.as_str()));
            continue;
        }
        let Some(open) = heading.filter(|_| line.starts_with("status:")) else {
            continue;
        };
        heading = None;
        let Some(caps) = STATUS.captures(line) else {
            skip(open, &format!("unreadable {line:?}"));
            continue;
        };
        let (status, finding) = (&caps[1], caps.get(2).map_or("", |m| m.as_str()));
        if !STATUSES.contains(&status) {
            skip(open, &format!("invalid status {status}"));
        } else if status != "offen" && finding.trim().is_empty() {
            skip(open, &format!("{status} without a finding"));
        } else {
            entries.push(Entry {
                skill: skill.to_owned(),
                session: open.1.to_owned(),
                stamp: open.0.to_owned(),
                status: status.to_owned(),
                finding: finding.to_owned(),
            });
        }
    }
    if let Some(open) = heading {
        skip(open, "no status line");
    }
}
