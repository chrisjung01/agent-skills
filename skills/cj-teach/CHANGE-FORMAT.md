# Change Review Format

A change review lives in `./changes/` as `0001-<dash-case-name>.html`, numbered like lessons. It explains one change that is already made (a pull request, a branch, a commit) well enough that the user can say what was done, why, and what else was possible, to someone who asks. It links the workspace's shared stylesheet from `./assets/`.

Every statement about the change comes from a source read in this session: the diff, the pull request text, the linked issue or spec, a decision record, a review thread, or the conversation in which the change was made. Each part below is a section of the page, in this order.

## 1. What changed

One paragraph in plain words: what the system does now that it did not do before. Below it: the pull request or commit the page describes, its state (open, merged), and the date it was read.

## 2. Before and after

One concrete case, shown twice: what a user or caller got before the change, and what they get now (a screen, a command and its output, a file). This is the example the user can repeat when asked what the change is for.

## 3. How the pieces fit

A diagram of the parts the change touches and how they call or feed each other, one box per part, the changed ones marked. Under it, one or two sentences per box. Use the project's own terms and define each in plain words on first use.

## 4. Decisions

The choices inside the change, most consequential first; three to five, not every line. Each has the same parts:

- **Question**: what had to be decided ("where does the time for the day come from?").
- **Chosen**: what the change does, with a link to the place in the diff.
- **Alternatives**: what else was possible, each with what it would have cost or gained.
- **Why this one**: the reason, with its source (decision record, pull request text, review thread, the conversation).
- **When it stops holding**: the condition under which the choice should be revisited.

Where no source records the alternatives or the reason, say "not recorded" and give your own reading below it, labelled as your reading. Never present a reconstructed reason as the author's.

## 5. Check it yourself

Two or three things the user does to see the change with their own eyes: the lines of the diff to read first, a command to run, a case to try. Each names what they should see.

## 6. Questions you may get

Three to five questions a colleague or reviewer is likely to ask ("why not a config flag?", "what happens to old data?"), each with an answer of two or three sentences the user can give, linked to the decision it rests on.

## 7. Open ends

What the change leaves untested, unfinished or risky, as its sources say: skipped cases, follow-up issues, known limits. Nothing found → say so.

## 8. Check your understanding

Three to five why-questions about the decisions, not about the code's wording ("why is the estimate a rule and not left to the model?"). Answers stay off the page: the user answers them in the chat, one per message, and gets feedback.
