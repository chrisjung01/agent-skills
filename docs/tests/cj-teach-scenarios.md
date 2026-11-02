# Scenarios: cj-teach

Skill under test: `cj-teach`. Every scenario is a paper exercise: the agent gets the skill text and the setup below, describes its next steps and the text and files it would write, and runs nothing. Tool output it would need (docs pages, file contents) is given in the setup or may be assumed to exist; the agent says which pages it would read.

Built from two real workspaces: a Go workspace where the user called lesson 0001 "not usable" for the exercises and later wrote in `NOTES.md` "the user works from the code, not from the lesson", and a request to learn how to send product-analytics alerts to the team's Slack channel.

## 1. A concrete task gets a walkthrough, not a mission interview

**Setup:** an empty directory, no `MISSION.md`. The user's instructions name no Slack channel.
**Prompt:** "/cj-teach I need to set up alerting on our signup chart in our product-analytics tool and get the alerts into our Slack channel. Explain what each step does and why."
**Expected:** no mission interview. Before writing, the agent reads the tool's official docs on alerts and on the Slack integration and names the pages. It writes `walkthroughs/0001-<slug>.html` that shows a diagram of the chain near the top (event, chart, alert, Slack integration, channel) and the prerequisites. Each step holds goal, the exact action, what happens behind it, why this way, a check of what the user should see, and a link to the docs section. The page names the date the docs were read and has a section on what can go wrong, and the chat asks why-questions one at a time. The Slack channel is looked up or asked, never invented. An agent that asks why the user wants to learn the tool, writes a short concept lesson, or invents a channel name fails.

## 2. The worked example is the same kind of task as the exercise

**Setup:** Go workspace with `MISSION.md` (backend servers), lessons 0001 and 0002, learning record 0003 ("can write a Go file with imports, stdlib calls and errors"). The next topic is functions; the exercises in `session03/` will parse an HTTP request line and headers.
**Prompt:** "/cj-teach functions"
**Expected:** the lesson's main example parses a line of the same shape (split with `strings.Cut`, return `(value, error)`), with the reasoning written as comments. A second, partly finished example leaves the key line for the user to complete. The exercise file's doc comments name the stdlib function to use and show one input and output. An agent that teaches functions with unrelated examples (halving numbers, a counter) and leaves the exercise without hints fails.

## 3. One concept explained in full, not just shown

**Setup:** as scenario 2.
**Prompt:** "/cj-teach defer"
**Expected:** the lesson explains `defer` with all five parts: what it is, why it exists (cleanup on every return path), how it works underneath (the call is registered, arguments evaluated at once, run last in first out when the function returns, also during a panic), when not to use it (in a long loop) and the typical mistakes. Deeper material sits in expandable sections, so the page stays readable. A lesson that shows syntax and three examples without the why and the mechanism fails.

## 4. A later lesson starts with recall and checks understanding with a why-question

**Setup:** as scenario 2, lesson 0003 on functions exists.
**Prompt:** "/cj-teach structs and methods"
**Expected:** the lesson opens with two or three questions on earlier lessons (functions, `(value, err)`), answered from memory before the new material. The checks ask the user to predict an output before revealing it, and at least one question asks why something works, which the user answers in chat and gets feedback on. A lesson whose only check is a multiple-choice quiz on today's material fails.

## 5. The session closes with what was missing

**Setup:** as scenario 2. The user solved `session03/` and writes "all green".
**Prompt:** "all green"
**Expected:** besides the learning record, the agent asks one question: what the user had to look up elsewhere or was missing from the lesson to solve the exercises. The answer goes into `NOTES.md` and changes how the next lesson is built. An agent that writes the learning record and moves on without asking fails.

## 6. A topic without a task still starts with the mission (regression guard)

**Setup:** an empty directory, no `MISSION.md`.
**Prompt:** "/cj-teach Rust"
**Expected:** the agent asks why the user wants to learn Rust before writing anything, one question in the message, and writes no lesson and no walkthrough yet.

## Results

Before: `main` at 858518c. After: branch `fix/teach-walkthrough`, uncommitted working tree, second run after the review fixes of the first. Both runs by a fresh paper-exercise agent.

1. Concrete task, walkthrough: before missed expected (mission interview), after met expected.
2. Worked example matches the exercise: before missed expected, after met expected.
3. One concept in full: before missed expected ("short, completable very quickly"), after met expected.
4. Recall and why-question: before missed expected, after met expected (first after-run missed: no required why-question in chat; fixed with "Running a lesson").
5. Session close asks what was missing: before missed expected, after met expected.
6. Topic starts with the mission: before met expected, after met expected; regression guard.
