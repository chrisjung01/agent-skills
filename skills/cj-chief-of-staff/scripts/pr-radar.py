"""Facts for the pull request radar: one line per open pull request.

Usage: python3 scripts/pr-radar.py <owner/repo> <login>
       python3 scripts/pr-radar.py --check

Output is tab-separated, longest idle first:
number, draft, author, idle days, last actor, last action, review decision,
CI state, open threads, requested reviewers, reviewers, involvement of <login>, title.
Bot accounts never count as activity.
"""
import datetime as dt
import json
import subprocess
import sys

QUERY = """query($owner:String!,$name:String!){repository(owner:$owner,name:$name){
pullRequests(states:OPEN,first:100){nodes{
number title isDraft author{login} reviewDecision
reviewRequests(first:10){nodes{requestedReviewer{... on User{login} ... on Team{name}}}}
commits(last:1){nodes{commit{committedDate statusCheckRollup{state}}}}
reviews(last:30){nodes{author{login} state submittedAt}}
comments(last:30){nodes{author{login} createdAt}}
reviewThreads(last:50){nodes{isResolved comments(last:1){nodes{author{login} createdAt}}}}
}}}}"""


def is_bot(login):
    return not login or login.endswith("[bot]") or login.endswith("-bot") or login.lower().startswith("copilot") or login == "github-actions"


def when(s):
    return dt.datetime.fromisoformat(s.replace("Z", "+00:00"))


def row(pr, me, now):
    author = (pr["author"] or {}).get("login", "")
    commit = pr["commits"]["nodes"][0]["commit"]
    events = [(when(commit["committedDate"]), author, "commit")]
    for r in pr["reviews"]["nodes"]:
        who = (r["author"] or {}).get("login")
        if r["submittedAt"] and not is_bot(who):
            events.append((when(r["submittedAt"]), who, "review:" + r["state"].lower()))
    for c in pr["comments"]["nodes"]:
        who = (c["author"] or {}).get("login")
        if not is_bot(who):
            events.append((when(c["createdAt"]), who, "comment"))
    open_threads = 0
    for t in pr["reviewThreads"]["nodes"]:
        open_threads += not t["isResolved"]
        for c in t["comments"]["nodes"]:
            who = (c["author"] or {}).get("login")
            if not is_bot(who):
                events.append((when(c["createdAt"]), who, "thread"))
    last = max(events)
    requested = [n["requestedReviewer"].get("login") or n["requestedReviewer"].get("name")
                 for n in pr["reviewRequests"]["nodes"] if n["requestedReviewer"]]
    reviewers = sorted({(r["author"] or {}).get("login") for r in pr["reviews"]["nodes"]} - {author})
    reviewers = [r for r in reviewers if not is_bot(r)]
    involved = []
    if author == me:
        involved.append("author")
    if me in requested:
        involved.append("requested")
    if me in reviewers:
        involved.append("reviewed")
    if author != me and me not in reviewers and any(w == me for _, w, _ in events):
        involved.append("commented")
    idle = (now - last[0]).total_seconds() / 86400
    return [pr["number"], "draft" if pr["isDraft"] else "ready", author, f"{idle:.1f}",
            last[1], last[2], pr["reviewDecision"] or "", (commit["statusCheckRollup"] or {}).get("state", ""),
            open_threads, ",".join(requested), ",".join(reviewers), "+".join(involved), pr["title"]]


def check():
    now = when("2026-11-03T08:00:00Z")
    pr = {"number": 1, "title": "t", "isDraft": False, "author": {"login": "eve"}, "reviewDecision": None,
          "reviewRequests": {"nodes": []},
          "commits": {"nodes": [{"commit": {"committedDate": "2026-10-29T08:00:00Z", "statusCheckRollup": {"state": "SUCCESS"}}}]},
          "reviews": {"nodes": [{"author": {"login": "me"}, "state": "APPROVED", "submittedAt": "2026-10-30T08:00:00Z"},
                              {"author": {"login": "copilot-pull-request-reviewer"}, "state": "COMMENTED", "submittedAt": "2026-11-02T08:00:00Z"}]},
          "comments": {"nodes": [{"author": {"login": "renovate[bot]"}, "createdAt": "2026-11-03T07:00:00Z"}]},
          "reviewThreads": {"nodes": []}}
    r = row(pr, "me", now)
    assert r[3] == "4.0" and r[4] == "me" and r[5] == "review:approved", r  # bot comment and Copilot review ignored
    assert r[10] == "me" and r[11] == "reviewed", r
    print("ok")


def main():
    if sys.argv[1:] == ["--check"]:
        return check()
    repo, me = sys.argv[1], sys.argv[2]
    owner, name = repo.split("/")
    out = subprocess.run(["gh", "api", "graphql", "-f", "query=" + QUERY, "-F", f"owner={owner}", "-F", f"name={name}"],
                         capture_output=True, text=True, check=True).stdout
    now = dt.datetime.now(dt.timezone.utc)
    rows = [row(p, me, now) for p in json.loads(out)["data"]["repository"]["pullRequests"]["nodes"]]
    for r in sorted(rows, key=lambda r: -float(r[3])):
        print("\t".join(map(str, r)))


if __name__ == "__main__":
    main()
