//! The skill version of each skill call: the oldest commit whose `skills/<skill>/SKILL.md` in the
//! repo that holds this crate matches the loaded skill text, read through the git CLI.

use crate::window::{body, SkillCall};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The skill version when the repo has the skill but no revision matches the loaded text.
pub const UNCOMMITTED: &str = "uncommitted";
/// Claude Code appends the command's arguments to the loaded text after this.
const ARGUMENTS_MARKER: &str = "ARGUMENTS: ";

/// A revision of `skills/<skill>/SKILL.md`: (commit, content).
type Revision = (String, String);

/// The skill version of a loaded text, from the skill's revisions oldest first. `None` when the
/// skill never had a `SKILL.md` in the repo.
pub fn version(loaded: &str, revisions: &[Revision]) -> Option<String> {
    if revisions.is_empty() {
        return None;
    }
    let matching = revisions
        .iter()
        .find(|(_, content)| matches(loaded, content));
    Some(
        matching
            .map_or(UNCOMMITTED, |(commit, _)| commit)
            .to_owned(),
    )
}

/// Whether the loaded text is the revision without its frontmatter, behind the first line (the
/// skill's folder) and before an arguments block. Matching from the revision's side keeps
/// arguments and skill bodies that hold the marker themselves apart.
fn matches(loaded: &str, revision: &str) -> bool {
    body(loaded)
        .trim_start()
        .strip_prefix(without_frontmatter(revision))
        .is_some_and(|rest| {
            rest.trim().is_empty()
                || rest.starts_with('\n') && rest.trim_start().starts_with(ARGUMENTS_MARKER)
        })
}

fn without_frontmatter(content: &str) -> &str {
    let body = content
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .map_or(content, |(_, body)| body);
    body.trim()
}

/// Revisions of each skill, read once per run. Without a repo every skill has no version.
pub struct Versions {
    repo: Option<PathBuf>,
    revisions: HashMap<String, Vec<Revision>>,
}

impl Versions {
    /// The repo that holds `dir`; `None` when git is missing or `dir` is in no repo.
    pub fn in_repo_of(dir: &Path) -> Versions {
        let top = git(dir, &["rev-parse", "--show-toplevel"]);
        if top.is_none() {
            eprintln!(
                "skill-metrics: no git repo at {}, no skill versions",
                dir.display()
            );
        }
        Versions {
            repo: top.map(|top| PathBuf::from(top.trim())),
            revisions: HashMap::new(),
        }
    }

    /// Fills the skill version of each call and each of its nested skills.
    pub fn fill(&mut self, calls: &mut [SkillCall]) {
        for call in calls {
            call.skill_version = self.of(&call.skill, &call.text);
            for nested in &mut call.nested {
                nested.skill_version = self.of(&nested.skill, &nested.text);
            }
        }
    }

    fn of(&mut self, skill: &str, loaded: &str) -> Option<String> {
        let repo = self.repo.as_deref()?;
        let revisions = self
            .revisions
            .entry(skill.to_owned())
            .or_insert_with(|| revisions(repo, skill));
        version(loaded, revisions)
    }
}

/// On every branch, so a skill committed after the session or deleted since still matches; a
/// stash is no revision. `--full-history` keeps revisions a merge dropped, `--date-order` keeps
/// a parent before its child across refs. A commit that deletes the file has no content
/// and is left out.
fn revisions(repo: &Path, skill: &str) -> Vec<Revision> {
    let path = format!("skills/{skill}/SKILL.md");
    let args = [
        "log",
        "--exclude=refs/stash",
        "--all",
        "--full-history",
        "--date-order",
        "--reverse",
        "--format=%H",
        "--",
        &path,
    ];
    // ponytail: a failed log reads as "never in the repo" (NULL), it only warns.
    let Some(log) = git(repo, &args) else {
        eprintln!("skill-metrics: git log failed for {path}, no skill version");
        return Vec::new();
    };
    log.lines()
        .filter_map(|commit| {
            let content = git(repo, &["show", &format!("{commit}:{path}")])?;
            Some((commit.to_owned(), content))
        })
        .collect()
}

/// Commit time (Unix seconds) and day of each commit, from the repo that holds `dir`. A commit
/// the repo no longer has, or every commit without a repo, is left out.
pub fn commit_dates<'a>(
    dir: &Path,
    commits: impl IntoIterator<Item = &'a str>,
) -> HashMap<String, (i64, String)> {
    commits
        .into_iter()
        .filter_map(|commit| {
            let out = git(
                dir,
                &["show", "-s", "--format=%ct %cs", "--end-of-options", commit],
            )?;
            let (time, day) = out.trim().split_once(' ')?;
            Some((commit.to_owned(), (time.parse().ok()?, day.to_owned())))
        })
        .collect()
}

/// Stdout of a git command in `dir`, `None` when it fails. Paths are taken literally, so a skill
/// name from a transcript is never a pathspec pattern.
fn git(dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("--literal-pathspecs")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const V1: &str = "---\nname: a\ndescription: Use when a.\n---\n\n# A\n\nFirst.\n";
    const V2: &str = "---\nname: a\ndescription: Use when a.\n---\n\n# A\n\nSecond.\n";

    fn loaded(body: &str, arguments: Option<&str>) -> String {
        let arguments = arguments.map_or(String::new(), |a| format!("\n\nARGUMENTS: {a}"));
        format!("Base directory for this skill: /home/x/.claude/skills/a\n\n{body}{arguments}")
    }

    /// Git with an identity, for commits and merges.
    fn run(repo: &Path, args: &[&str]) {
        let status = Command::new("git")
            .args(["-c", "user.name=t", "-c", "user.email=t@t"])
            .args(["-c", "commit.gpgsign=false", "-C"])
            .arg(repo)
            .args(args)
            .status()
            .unwrap();
        assert!(status.success());
    }

    fn commit(repo: &Path, message: &str) -> String {
        run(repo, &["add", "-A"]);
        run(repo, &["commit", "--quiet", "--no-verify", "-m", message]);
        git(repo, &["rev-parse", "HEAD"]).unwrap().trim().to_owned()
    }

    fn write(repo: &Path, skill: &str, content: &str) {
        let dir = repo.join("skills").join(skill);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), content).unwrap();
    }

    #[test]
    fn the_loaded_text_matches_without_folder_line_arguments_and_frontmatter() {
        let revisions = [("c1".to_owned(), V1.to_owned())];
        let c1 = Some("c1".to_owned());
        assert_eq!(version(&loaded("# A\n\nFirst.\n", None), &revisions), c1);
        // The last line of a body is no arguments block.
        assert_eq!(
            version(&loaded("# A\n\nFirst.ARGUMENTS: x", None), &revisions),
            Some(UNCOMMITTED.to_owned())
        );
        assert_eq!(
            version(
                &loaded("# A\n\nFirst.", Some("x\nARGUMENTS: y")),
                &revisions
            ),
            c1
        );
        assert_eq!(
            version(&loaded("# A\n\nChanged.", None), &revisions),
            Some(UNCOMMITTED.to_owned())
        );
        assert_eq!(version(&loaded("# A\n\nFirst.", None), &[]), None);
        // A body that holds the marker itself, loaded without arguments.
        let marked = [(
            "c3".to_owned(),
            "# A\n\nARGUMENTS: listed here.\n".to_owned(),
        )];
        assert_eq!(
            version(&loaded("# A\n\nARGUMENTS: listed here.\n", None), &marked),
            Some("c3".to_owned())
        );
        // A revision without frontmatter is compared whole.
        let plain = [("c2".to_owned(), "# A\n\nFirst.\n".to_owned())];
        assert_eq!(
            version(&loaded("# A\n\nFirst.", None), &plain),
            Some("c2".to_owned())
        );
    }

    #[test]
    fn the_oldest_matching_commit_on_any_branch_is_the_version() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "--quiet"]).unwrap();
        write(repo, "a", V1);
        let first = commit(repo, "v1");
        write(repo, "a", V2);
        let second = commit(repo, "v2");
        // V1 again, then the skill is renamed and finally deleted.
        write(repo, "a", V1);
        commit(repo, "back to v1");
        git(repo, &["mv", "skills/a", "skills/b"]).unwrap();
        let rename = commit(repo, "rename");
        fs::remove_dir_all(repo.join("skills/b")).unwrap();
        commit(repo, "delete");
        // A revision only on a branch that is not checked out.
        git(repo, &["checkout", "--quiet", "-b", "side"]).unwrap();
        write(repo, "a", "# A\n\nThird.\n");
        let side = commit(repo, "v3 on a branch");
        git(repo, &["checkout", "--quiet", "-"]).unwrap();
        // A revision a merge dropped, on a branch that is gone.
        write(repo, "m", "# M\n\nBase.\n");
        commit(repo, "m");
        git(repo, &["checkout", "--quiet", "-b", "feature"]).unwrap();
        write(repo, "m", "# M\n\nY.\n");
        commit(repo, "y");
        git(repo, &["checkout", "--quiet", "-"]).unwrap();
        write(repo, "m", "# M\n\nX.\n");
        let dropped = commit(repo, "x");
        run(
            repo,
            &["merge", "--quiet", "--no-edit", "-X", "theirs", "feature"],
        );
        git(repo, &["branch", "-D", "feature"]).unwrap();
        // A stashed edit is no commit.
        write(repo, "m", "# M\n\nStashed.\n");
        run(repo, &["stash", "--quiet"]);

        let mut versions = Versions::in_repo_of(repo);
        let mut of = |skill: &str, body: &str| versions.of(skill, &loaded(body, None));
        assert_eq!(of("a", "# A\n\nFirst."), Some(first.clone()));
        assert_eq!(of("a", "# A\n\nSecond."), Some(second));
        assert_eq!(of("a", "# A\n\nThird."), Some(side));
        assert_eq!(of("a", "# A\n\nEdited."), Some(UNCOMMITTED.to_owned()));
        assert_eq!(of("m", "# M\n\nX."), Some(dropped));
        assert_eq!(of("m", "# M\n\nStashed."), Some(UNCOMMITTED.to_owned()));
        // A renamed skill matches under the name it had in the session.
        assert_eq!(of("a", "# A\n\nFirst."), Some(first));
        assert_eq!(of("b", "# A\n\nFirst."), Some(rename));
        // A plugin skill or one from elsewhere has no SKILL.md in the repo.
        assert_eq!(of("superpowers:writing-skills", "# A\n\nFirst."), None);
        assert_eq!(of("*", "# A\n\nFirst."), None);
    }

    #[test]
    fn commit_dates_come_from_the_repo() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "--quiet"]).unwrap();
        write(repo, "a", V1);
        let first = commit(repo, "v1");
        let dates = commit_dates(repo, [first.as_str(), "0123456789abcdef", "--all"]);
        let (time, day) = &dates[&first];
        assert!(*time > 1_700_000_000);
        assert_eq!(day.len(), "2026-10-09".len());
        // A commit the repo does not have, and no option smuggled in as a commit.
        assert_eq!(dates.len(), 1);
        assert!(commit_dates(&repo.join("missing"), [first.as_str()]).is_empty());
    }

    #[test]
    fn outside_a_repo_no_skill_has_a_version() {
        let dir = tempfile::tempdir().unwrap();
        let mut versions = Versions::in_repo_of(dir.path());
        assert_eq!(versions.of("a", &loaded("# A", None)), None);
    }
}
