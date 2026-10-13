# cj-static-analysis scenarios

Spec #62, ticket #64. Each review case fills the `cj-reviewing` brief once from the text before the change and once from the text after it, and gives it to a fresh reviewer on the Opus tier. For the after runs a separate fresh agent first followed `cj-static-analysis` as the caller and produced the `Lint:` and source lines the brief passes on. Runs on 2026-10-13, one per version unless noted.

Fixtures live outside the repo:
- **Lint repo**: a small Python shop with `AGENTS.md` (log through `shop.log`, never `print`; a `TODO` names its issue) and `scripts/lint.sh`, which reports `print(` as an error (exit 1) and a bare `TODO` as a warning. The CI workflow runs it; warnings do not fail CI. One commit adds `discount(total)` with a `print` and a bare `TODO`.
- **Broken lint repo**: the same, but `scripts/lint.sh` first checks for a tool that is not installed and exits 127.
- **grep repo**: the shop without a linter or a composition root. One commit changes `apply_tax(total)` to `apply_tax(total, rate)` and updates its one caller, adds a `print`, adds `on_refund`, which `dispatch` finds by name through `getattr`, and adds `format_receipt`, which only a test imports. `legacy_round` sat unused in a touched file before the commit.
- **Analyzer repo**: a Rust binary crate whose `AGENTS.md` names `cargo check` as analyzer. One commit adds `bulk_discount`, called only from its unit test; `cargo check` reports it as `dead_code`.
- **Root repo**: a shop whose `shop/app.py` serves only the routes its `ROUTES` table lists, with no linter. One commit adds `receipt_route`, which `ROUTES` never lists, and `loyalty_points`, which only a test imports.
- **Index repo**: the grep repo with one more commit before the base, plus a new `invoice_total` that calls `apply_tax` and is called by nothing. A script stands in for a connected code index: it prints its indexed commit and the references at that commit, either the base (current) or the commit before it (behind the base). Paper only; a real index service was not available (#62, test seam 4).

## 1. Blocking lint violation and a warning (AC-1, edge row non-blocking severity)

Expected: no finding for the `print` the linter blocks; the bare `TODO`, a warning that does not fail CI, stays a finding.

Results:
- before: missed expected. The `print` came back as a medium CORRECTNESS finding "Lint not run, and it would fail CI".
- after: met expected. No finding for the `print`, with a note that the Lint line names it; the `TODO` stays a medium STANDARDS finding.

Cases 2 to 4 share one run per version on the grep repo. The first after run used an earlier wording; ORPHANED also listed symbols that are not orphaned, one of them unused before the diff. Fixed in the brief: one line per orphaned symbol, none for any other. The after results of cases 2 to 4 and 9 below are from runs on the text after review round three. Two more review rounds then changed the text (test code is never orphaned, a missing WIRING line is cited under any source, the first checked source wins, an analyzer that leaves out exported symbols hands them on); only case 5 was run again on that text. A last round added that a symbol the diff deletes is never orphaned; no case ran on it. Cases 1, 6 to 8 and 10 ran on the text after review round three.

## 2. Orphaned symbol with only grep (AC-4)

Expected: `format_receipt` is a question, not a finding.

Results:
- before: missed expected. A high CORRECTNESS finding through WIRING, with no source named.
- after: met expected. `format_receipt` is an ORPHANED question.

## 3. Caller claims from grep (AC-5)

Expected: every claim about callers names `grep` and says unchecked.

Results:
- before: missed expected. "Only caller", "no place outside the test" with no source.
- after: met expected. Every ORPHANED line names "grep, unchecked".

## 4. Symbol reached by a string name, only grep (edge row)

Expected: `on_refund`, which `dispatch` finds by name, gets a question, never a finding.

Results:
- before: missed expected. No finding, but no question either: the reviewer decided on its own that the name lookup reaches it.
- after: met expected. `on_refund` is an ORPHANED question asking whether anything emits a "refund" event.

## 5. Analyzer as source (AC-3)

Expected: `bulk_discount` is a low finding titled "Orphaned symbol" that names `cargo check` as source.

Results:
- before: missed expected. A high WIRING finding; the reviewer guessed that `cargo check` would flag it but named no source.
- after: met expected. ORPHANED line `bulk_discount | src/pricing.rs:7 | analyzer cargo check | F3`, F3 a low STANDARDS finding. The gather run first took `cargo check` for the lint command; fixed in the skill: a command that only compiles or type-checks is no lint command. Rerun of the gather on that wording: `Lint: none, no lint command found`, source `analyzer cargo check`. On the round-three wording the brief got `callers: grep, unchecked; orphans: analyzer cargo check`, since `cargo check` answers orphans but lists no callers; the review gave no WIRING line, because `bulk_discount` is no entry point, and the same low finding. Rerun of gather and review on the text after round five, where `bulk_discount` is a `pub fn` in a binary crate that `cargo check` reports: same source line, and `bulk_discount | src/pricing.rs:7 | analyzer cargo check | F2`, F2 a low STANDARDS finding "Orphaned symbol".

## 6. Who calls a symbol, current code index (AC-6)

Fixture: index repo; the implementing skill asks who calls `apply_tax`.

Expected: the answer names `code index shopindex @ <indexed commit>`.

Results:
- before (no skill): missed expected. The agent dropped the index because it predates the diff and answered from `grep` alone.
- after: met expected. `apply_tax: shop/checkout.py:8, shop/invoice.py:5 | code index shopindex @ df0d2eb`. The run asked whether imports count and which line numbers to give; the skill now says calls and imports both count, at the diff's head.

## 7. Index on the base, the diff adds a caller (edge row)

Same run as case 6. The index lists one call at the base; the diff moves it and adds `invoice_total` as a second caller.

Expected: callers from the index plus the diff.

Results:
- before: missed expected. Both callers, but from `grep` alone with the index dropped (case 6).
- after: met expected; both callers, the moved one at its head line. The review of the same repo with the current index reported `invoice_total` and `format_receipt` as low "Orphaned symbol" findings naming the index and its commit.

## 8. Code index behind the base (edge row)

Expected: orphaned symbols are questions, not findings.

Results:
- before: missed expected. `invoice_total` and `format_receipt` were one high WIRING finding.
- after: met expected. Three ORPHANED questions naming the index commit as unchecked, filled in by `git grep` at the head.

## 9. Only grep, with a composition root (AC-4, review round two)

Fixture: root repo; spec #21 asks for the receipt route and for orders to earn loyalty points.

Expected: the unregistered route stays a high WIRING finding, because the reviewer read `ROUTES`; `loyalty_points` is a question, not a finding.

Results:
- before: missed expected. The route was high, right; `loyalty_points` was a WIRING line "missing" with no source, its finding moved to a medium SPEC finding.
- after: met expected. Route high with "composition root shop/app.py read directly"; `loyalty_points` an ORPHANED question. On the round-two wording spec #21's AC-2 (loyalty points) came back "not verifiable" pending that question; on the round-three wording it is partial, with a medium SPEC finding, as in the before run.

## 10. Lint command fails to run (edge row)

Fixture: broken lint repo.

Expected: a LINT line names the lint command and its error, and the `print` is reported from the documented rule.

Results:
- before: missed expected. The old brief has no LINT section, so no line names the command and its error; the reviewer opened `scripts/lint.sh` on its own, saw the exit 127 and reported the `print`.
- after: met expected. The LINT section says "did not run: shoplint: command not found", and the `print` is reported.

AC-2 (documented rule without a linter) and the edge row "symbol orphaned before the diff" have no case of their own: in the grep repo runs both versions reported the `print` as a medium STANDARDS finding and gave no line for `legacy_round`, so those runs tell the versions apart for neither.

## 11. A calling skill asks before a fix (AC-6, ticket #65)

Fixture: index repo, the current index named in the prompt as the user's `code-index` fact. `cj-review-loop` step 2 assesses one finding: `apply_tax` crashes when `rate` is None. Runs on 2026-10-13, one per version.

Expected: the callers answer comes from the code index and names `shopindex` with its indexed commit.

Results:
- before: missed expected. `grep` over the head found both callers; the index was not asked and no source was named.
- after: met expected. `apply_tax: shop/checkout.py:2, :8, shop/invoice.py:1, :5 | code index shopindex @ df0d2eb`, the index answer plus the diff.

## 12. A calling skill reads the affected code (AC-6, ticket #65)

Fixture and runs as in case 11. `cj-implement` step 1 reads the code for ticket #21, which changes how `apply_tax` rounds.

Expected: as in case 11.

Results:
- before: missed expected. `git grep` only; the agent noted the old text never asks for the code index.
- after: met expected. The same answer line as case 11, the index found current against the merge base.

## 13. Fix a review thread (AC-6, ticket #65)

Fixture and runs as in case 11. `cj-address-pr-feedback` phase 2 fixes one thread classified as fix: `apply_tax` crashes when `rate` is None.

Expected: as in case 11.

Results:
- before: missed expected. `grep` and `git grep` only; the agent noted phase 2 says grep and never names the index.
- after: met expected. The index answer plus the diff, source `code index shopindex @ df0d2eb`, cross-checked with `git grep`.

## 14. Read the code before grilling (AC-6, ticket #65)

Fixture and runs as in case 11. `cj-grill-with-docs` step 1 reads the code for issue #21.

Expected: as in case 11.

Results:
- before: missed expected. `grep` gave the callers; the agent ran the index on its own, called it stale and named no source.
- after: met expected. The same answer line as case 11, the index found current against the merge base.

## 15. Trace the callers of stack frames (AC-6, edge row index behind, ticket #65)

Fixture: index repo. A pasted error-tracker issue: `TypeError` in `apply_tax` called from `invoice_total`, release `f7a203b`, the head. `cj-bug-triage` runs the code bullets of its error-tracker phase 2.

Expected: callers at the release commit with a named source; an index on an older commit counts as unchecked and `grep` fills in.

Results:
- before: missed expected. The text asks for no callers; the agent searched with `git grep` on its own and never asked the index.
- after: met expected. Base and head are the release; source `code index shopindex @ df0d2eb, not the base f7a203b, unchecked`; `invoice_total` has "no callers (grep, unchecked)" and an open question about an outside caller.

## 16. Callers of a changed signature in a teammate's pull request (AC-6, ticket #65)

Fixture: a copy of the index repo per run, the last commit as a teammate's pull request. `cj-review-teammate-pr` phase 2 only.

Expected: the callers of `apply_tax`, whose signature the diff changes, with a named source; a worktree the review creates is removed.

Results:
- before: missed expected. `grep` on the head as "source of truth"; the agent asked the index on its own and set it aside as the base's view instead of adding the diff. No source line. Worktree removed.
- after: met expected. Source `code index shopindex @ df0d2eb` plus the diff; `Lint: none, no lint command found`; one worktree for local checks, created and removed.

A `cj-review-teammate-pr` run on the lint repo gave no finding for the blocked `print` in both versions: the old text already reached the LINT section through `cj-reviewing`, so that run tells the versions apart for nothing. A rerun on the final text, where phase 3 uses `cj-reviewing` for the severity scale only, took the `Lint:` lines from phase 2 and again gave no finding for the `print`. No case covers the worktree of a copy-only pull request.

## Not run: AC-7

The scenario 5 fixture of `docs/tests/cj-reviewing-scenarios.md` is a real pull request outside this repo. AC-7 stays open until the acceptance of #62.
