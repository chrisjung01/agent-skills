# Challenge result

Read in the last step of [CHALLENGE.md](CHALLENGE.md). One format for a GitHub comment and a docs page, in the language from `conventions`, else the issue's language. Teammates who open the issue see only the recommendation and the kill criterion; the reasoning sits in a collapsed block anyone can expand. The `<summary>` label in that language too; a docs page that renders no `<details>` drops those two tags and the `<summary>` line.

```
## Challenge: [issue title]

**Recommendation:** [build / build differently / validate first / don't build], with reasoning in one sentence

**Kill criterion:** [signal, threshold, date; only for build / build differently]

<details>
<summary>Full challenge</summary>

**Core problem:** [1 to 2 sentences, without the proposed solution]

**For:**
- ...

**Against:**
- ...

**Goal link:** [team + objective or key result, or "none visible"]

**Data:** [what tracking or analytics shows, or "not available"]

**Smaller lever:** [the rung from step 5 that holds, or why none does]

**Pre-mortem:**
- [likely failure reason]: [how the plan prevents it, or "open"]

**Open gaps:** [each gate still open, or "none"]

</details>
```

GitHub renders Markdown inside `<details>` only with a blank line after `</summary>` and one before `</details>`; without them the bold labels and lists show as raw text.
