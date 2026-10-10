# Scenarios: cj-grill-with-docs

Skill under test: `cj-grill-with-docs`.

## 1. The closing reply tells the user what to type next

**Setup:** issue #5091 is grilled to the end; the user confirmed the decision list. The conversation is in German.
**Prompt:** "Passt so." (write the closing reply).
**Expected:** the reply ends with an instruction the user can follow without interpreting it: run `/cj-to-spec <issue>` now, in this session, and do not `/clear` first. A line that only names the command with a reason ("Nächster Schritt in dieser Session, vor jedem /clear: /cj-to-spec …") fails: it reads as "do this, then clear" or "clear, then do this".
**Origin:** session 8d055489, the user: "Was soll das? Das sind keine klaren Anweisungen."

## Runs

- 2026-10-09, before the change: one run. "Nächster Schritt: `/cj-to-spec …` hier in dieser Session, vor `/clear` oder einer Kompaktierung." Fail.
- 2026-10-09, after the change: one run. "Nächster Schritt: Führ jetzt `/cj-to-spec …` aus, in dieser Session. Vorher kein `/clear`: …" Pass.
