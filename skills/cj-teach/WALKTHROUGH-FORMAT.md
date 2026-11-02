# Walkthrough Format

A walkthrough lives in `./walkthroughs/` as `0001-<dash-case-name>.html`, numbered like lessons. It leads the user through one real task they do themselves, and explains every step well enough that they could do it again without the page. It links the workspace's shared stylesheet from `./assets/`.

The user reads it top to bottom, once while doing the task and later as a reference. Each part below is a section of the page, in this order.

## 1. Outcome

One sentence: what works when the user is done ("an alert posts to the team channel when daily signups fall below 50"). Below it: the date the sources were read, and the version of the product or tool they describe.

## 2. How the pieces fit

A diagram of the chain the task builds, one box per part, in the order data or control flows through them (for example: event, chart, alert, Slack integration, channel). Under it, one or two sentences per box: what this part is and what it does in the chain. Use the glossary's terms if the workspace has one; otherwise define each term here in plain words.

This is the mental model. Every step below refers back to a box.

## 3. Before you start

The prerequisites the sources name, each as a check the user can make: account and role, plan or tier, permissions on the other system (the Slack workspace), data that must already exist. Name what happens if one is missing ("without admin rights the Connect button is greyed out").

## 4. Steps

Numbered steps, one action each. Every step has the same parts:

- **Goal**: what this step achieves, and which box of the diagram it touches.
- **Do**: the exact action. Menu path, button label, command or code, copied from the source. For a value only the user knows, a placeholder with what to fill in.
- **What happens**: what the system does behind the action (stores a token, creates a webhook, schedules a check every hour).
- **Why this way**: the reason for this setting or order, and the alternative and its trade-off when one exists.
- **Check**: what the user should see now, and how to tell it worked.
- **Source**: a link to the section of the docs this step comes from.

Put longer background in an expandable `<details>` block inside the step, so the action stays easy to find.

## 5. What can go wrong

The failures the sources or the step's mechanism predict, each as symptom, cause and fix ("alert never fires: the insight's interval is weekly, so the threshold is checked weekly"). Include how to test the whole chain once without waiting for a real event, if the product allows it.

## 6. Check your understanding

Three to five why-questions about the chain, not about the clicks ("why does the alert belong to the insight and not to the channel?"). Answers stay off the page: the user answers them in the chat, one per message, and gets feedback.

## 7. Go further

The primary source to read next, related tasks the user may want next, and the reminder that the agent can answer follow-up questions.

## Rules

- **The user acts, the page explains.** Never tell the user that the agent will do a step for them.
- **Copy, don't paraphrase, what the user must type or click.** Labels and commands match the source exactly; where the source may be outdated, say so at that step.
- **No invented organisation facts.** Channel names, project names and account names come from the project context or from the user.
- **One task per walkthrough.** A second task is a second page, linked from the first.
