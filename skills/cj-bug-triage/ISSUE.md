# Issue presets

Passed to `cj-filing-issues` (File) on "Confirmed bug" or "UX finding". Body content and writing rules come from that skill.

- Kind: bug, bug template (`issue-tracker`). UX finding: the UX finding template and its label instead.
- Title: `<Screen or module>: <symptom in one sentence>`.
- Labels: the bug type label; the tier mapped onto the impact or severity labels (none: the tier goes in the body); team from `teams` (fits several or none: leave it and say so).
- Body facts, error-tracker input: the error-tracker link, affected versions and the share of production users, the code path with `file:line`, the commit or release where it started if found, impact in one sentence.
- Body facts, human report: steps as traced or run (traced-only marked), expected / actual, source permalink or support reference, app / platform / version, suspected `file:line`, the regression test that would catch it, the tier with its reason.
