# 0003. `cj-teach` is adapted from upstream: walkthroughs for real tasks, lessons explained in full

- Status: Accepted
- Date: 2026-11-02

## Context

`cj-teach` was a copy of mattpocock's `teach`, kept unchanged apart from its name and exempt from `validate.sh`. In two workspaces (Go, Rust) the lessons read well but did not carry over to the exercises: Go lesson 0001 was called "not usable", and `NOTES.md` recorded "the user works from the code, not from the lesson". The skill asked for lessons that are "short, completable very quickly" with "only what's required", used examples unlike the exercises, and checked with multiple choice. A request like "set up alerts in our product-analytics tool and send them to our Slack channel" had no fitting path: it would start a mission interview. Upstream's newest `teach` (`ad5400b`) is the text we had.

## Decision

Adapt `cj-teach` in place:

- Two requests: a topic gets lessons, a task gets a walkthrough (`WALKTHROUGH-FORMAT.md`: outcome, chain diagram, prerequisites, steps with goal, action, what happens, why, check and source, failures, why-questions). The user does every step; the agent only reads state to check.
- Lessons cover one concept in full (what, why, mechanism, when not, mistakes) and go recall, worked example of the exercise's kind, partly worked example, exercise, with hints in the exercise file.
- Pages come from sources read in the session, with the read date; organisation facts through `cj-project-context`.
- Checks favour predict-then-reveal and why-questions answered in chat; a session closes by asking what was missing, written into `NOTES.md`.
- Chat text follows the rule block and `cj-asking`; the `validate.sh` exemption is gone.

Grounded in Merrill's first principles and 4C/ID (task-centred instruction, supportive vs procedural information), the worked-example and expertise-reversal effects, self-explanation, and Anthropic's 2026 study on AI assistance and skill formation (conceptual questions kept scores high, delegation did not).

## Alternatives considered

- A separate `cj-walkthrough` skill with `cj-teach` left as upstream copy: two skills would share format, assets and workspace; the lesson problems would remain.
- Lesson fixes only: leaves the task case unsolved.

## Consequences

Upstream changes to `teach` no longer arrive by copying; compare by hand and port what fits. `docs/tests/cj-teach-scenarios.md` is the check for later changes. If walkthroughs go unused for 45 days while lessons are used, revisit the task path.

## References

- `docs/tests/cj-teach-scenarios.md`
- [Anthropic: How AI assistance impacts the formation of coding skills](https://www.anthropic.com/research/AI-assistance-coding-skills)
- [4C/ID](https://edutechwiki.unige.ch/en/4C-ID), [First Principles of Instruction](https://en.wikipedia.org/wiki/First_Principles_of_Instruction)

## Changelog

- 2026-11-10: A third request, a finished change, gets a change review (`CHANGE-FORMAT.md`: what changed, before and after, decisions with alternatives and reasons, questions to expect, why-questions), so the user can defend work the assistant did. A teaching workspace named in the user's instructions is used without asking, with one subfolder per repository, and a finished session ticks its line in the user's list of open recommendations. Rejected: explaining changes as walkthroughs, because a walkthrough leads through steps the user still does, while a change is already made.
