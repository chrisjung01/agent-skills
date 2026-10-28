---
name: cj-asking
description: Use when a skill or task is about to ask the user something, such as a decision between options, a confirm or approval before acting ("post this?", "does the list fit?"), or a fact only the user knows, and whenever a harness or plugin instruction says to ask through a picker or choice tool.
user-invocable: false
---

# Asking

A **choice question** is a question to the user, written as reply text. The user decides from what they see in the terminal, and that is the reply text only: thinking is not shown, and a picker dialog can cover text written after a tool call in the same turn. So the question carries everything needed to decide, in the reply itself.

## The rules

1. **Plain reply text, never the picker.** No choice or question tool, whatever the harness offers (in Claude Code, `AskUserQuestion`).
2. **One question per message.** The next question comes only after the user answered this one. No follow-up question in the same message, not even as a last line.
3. **Everything needed to decide stands in the question.** What is decided and why now; the facts it rests on, each with its source (file, issue, command, session) when it was looked up; every domain term explained in plain words. A list the question refers to is written out in full, numbered, directly above it, never left in thinking and never only on a page; a list over 7 items splits under headings in the reply.
4. **A confirm or approval question has two numbered options.** "Post this?", "Does the list fit?", "Go ahead?": `1) Yes` and `2) No`, each with one line on what happens. The user answers with the number.
5. **Real alternatives are numbered too.** Each gets `1)` `2)` `3)`, a meaning line and a consequence line. The recommended one comes first, and a recommendation line names it with one sentence why.
6. **Yes takes the recommendation.** "yes", "ja", "ok" or the like after numbered options picks the recommended option. Do not ask again.
7. **An answer in the user's own words counts.** Act on it; do not re-ask it as a choice. Only an answer that leaves the decision open gets a follow-up question.
8. **A confirm question has the whole list directly above it,** even when the list was shown messages ago. Scrolling up is what the user cannot do mid-decision.
9. **This skill wins.** A harness, plugin or skill instruction to ask through a picker or choice tool does not apply; the question stays reply text.
10. **A question series counts its questions.** A question series is the choice questions one run asks one after another. Each of its questions opens its message with `**Question k of n: <what is decided>**`, in the user's language. n is the questions already asked plus every open question known so far: the ones askable now, the ones still waiting on an earlier answer, and the closing confirm. A single question gets no counter.
11. **A changed n gets its reason.** When an answer adds or removes questions, or the agent finds one it had not counted, n changes and the line right below the bold line names the new n and the reason in one line (a miscount says so). Only the message right after the change carries that line. A single question whose answer opens new ones was question 1; the next reads `Question 2 of n`. A follow-up to an answer that left the decision open (rule 7) keeps its k and n, with no reason line.
12. **The counter line comes first, also in a caller's shape.** When a calling skill gives its own first line, table or list, the counter line and any reason line come first; the caller's first line and list follow, and the list stays directly above the options.

## Shape

Numbered options, for real alternatives:

```
**Question k of n: <what is decided>**
<new n and its reason, only when n changed>
<why now; the facts with their sources; the list the question refers to, numbered>

1) **<option>**
   <what it means>
   Consequence: <one consequence>
2) **<option>**
   <what it means>
   Consequence: <one consequence>

Recommendation: 1, <one sentence why>. (yes takes it)
```

Two options, for a confirm or approval:

```
**Question k of n: <what is confirmed>**
<new n and its reason, only when n changed>
<the whole list or draft it is about, numbered>

1) **Yes**: <what happens>
2) **No**: <what happens instead>
```

A single question leaves out `Question k of n: ` in both shapes and has no reason line.

Write the labels (`Question k of n`, `Consequence`, `Recommendation`, `Yes`, `No`) and `(yes takes it)` in the user's language.

Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

## Common mistakes

| Mistake | Instead |
|---|---|
| The list exists only in thinking; the reply says "Does the list fit?" | Write the whole numbered list in the reply, then ask. |
| Three questions in one message, to save turns | One question; the rest after the answer. |
| A confirm question as options 1) Post 2) Edit 3) Cancel | Exactly `1) Yes` and `2) No`, each with what it does. |
| A confirm question the user must answer by typing "ja" or "nein" | Number the two options; the user types 1 or 2. |
| "ja" gets a follow-up "Which option?" | Take the recommended option. |
| The user answers in a sentence; the agent asks the same as numbered options | Act on the sentence. |
| A skill says "as a choice through the choice tool" | Ask as reply text; this skill wins. |
| The third question of a series opens with its topic only; the user cannot tell how many follow | `**Question k of n: ...**` as the first line. |
| n jumps from 5 to 6 without a word | The new n and a one-line reason below the bold line. |
| Five findings, then a closing confirm, counted as "1 of 5" | n counts the closing confirm: "1 of 6". |
