---
name: cj-html-report
description: Use when a result has several items whose structure, dependencies or trend is easier to see than to read, such as architecture candidates, tickets with blocked-by edges, findings by severity or weekly usage, and should be shown as a visual page next to the chat summary.
user-invocable: false
---

# HTML report

A **page** is one self-contained HTML file that shows a result at a glance: one **card** per item, the **diagram** carries the weight, prose stays in single sentences. The page is a second view. The chat still gets the summary, so nothing is lost when no browser is around. For that chat summary: Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

## The page

Copy [scaffold.html](scaffold.html) and fill it. Its parts, in order:

1. **Header**: subject, date, base (branch and short commit), one-line context (why these items, e.g. "hot spots from the last 200 commits"), and a legend for every visual convention the diagrams use.
2. **Overview card**, only when the items relate to each other (a dependency graph, a timeline, a trend across items).
3. **Cards**, one `<article>` per item, each with:
   - title naming the change or finding
   - badge row: one badge from an ordered scale (strength, severity, priority, wave), plus at most two tags
   - files as `path:line-range`, monospaced, when the item has files
   - the diagram (patterns in [DIAGRAMS.md](DIAGRAMS.md)); before and after side by side when the item is a change
   - problem or risk: one sentence
   - change or consequence: one sentence
   - wins, or what the item unlocks: bullets of six words or fewer
   - a callout only for a conflict with a recorded decision (ADR, agreement)
4. **Closing card**: the one item you would do first, or the verdict, with one sentence why and an anchor link to its card.

Badges: emerald for the first or strongest level, amber for the middle, slate for the last or weakest; red only for severity high. The legend names the scale.

Text is in the user's language and goes through the `unslop` skill; where that skill is not available, the one-sentence and six-word limits carry the same job. The calling skill's vocabulary is used as is; a page never introduces new names for things.

## Where it goes

- Path: the OS temp directory (`$TMPDIR`, else `/tmp`), `<skill>-<slug>-<YYYYMMDD-HHMMSS>.html`. Never inside a repo.
- Open it (`open` on macOS, `xdg-open` on Linux) and print the absolute path. In a background run without a display, print the path only.
- Scripts: only the Tailwind CDN and the Mermaid ESM import from the scaffold. No app code, no other network calls.
- Content the calling skill would redact in chat (secrets, customer names, personal data) is redacted on the page too.

## Done when

The file exists at the printed path, every item from the result has a card, every card has a diagram, the closing card links to an existing card, and the chat summary was given as well.

## Check it renders (optional, when a browser binary exists)

```bash
"<chrome>" --headless=new --disable-gpu --virtual-time-budget=10000 --window-size=1280,<height> --screenshot=<path>.png "file://<page>"
```

`<height>`: about 700 per card plus 600. Read the screenshot. A red "Diagram error" line or an empty diagram box means a syntax error in that diagram: fix it and render again. Keep the scaffold's render loop; Mermaid's `startOnLoad` leaves diagrams empty under this check.
