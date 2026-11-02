---
name: cj-teach
description: Teach a topic over several sessions, or walk through a real task with the why of each step.
disable-model-invocation: true
argument-hint: "What would you like to learn, or which task should I walk you through?"
---

The user has asked you to teach them something. Decide first which of two requests it is:

- **A topic** ("Rust", "functions in Go", "yoga"): the user intends to learn it over several sessions. Teach it through [lessons](#lessons), grounded in the [mission](#the-mission).
- **A task** ("set up an alert on our signup chart and send it to the team channel", "put this service behind a reverse proxy"): a concrete outcome in a real system, which the user wants to reach and understand along the way. Teach it through a [walkthrough](#walkthroughs). The task is the mission; do not interview the user about why they want to learn.

When a request could be both, ask which one the user means.

## Teaching Workspace

Treat the current directory as a teaching workspace. Workspace paths (`./lessons/` and the rest) resolve from the directory `/cj-teach` was run in; only the `*-FORMAT.md` links resolve from this skill's folder. The state of their learning is captured in this directory in several files, each created lazily, when it is first needed:

- `MISSION.md`: A document capturing the _reason_ the user is interested in the topic. This should be used to ground all teaching. Use the format in [MISSION-FORMAT.md](./MISSION-FORMAT.md).
- `./reference/*.html`: A directory of reference materials. These are the compressed learnings from the lessons - cheat sheets, reference algorithms, syntax, yoga poses. They are the raw units of learning. They should be beautiful documents which print out well, and are designed for quick reference.
- `GLOSSARY.md`: The canonical terms of the topic, once the user can use them. Use the format in [GLOSSARY-FORMAT.md](./GLOSSARY-FORMAT.md).
- `RESOURCES.md`: A list of resources which can be explored to ground your teaching in contextual knowledge, or to acquire knowledge and wisdom. Use the format in [RESOURCES-FORMAT.md](./RESOURCES-FORMAT.md).
- `./learning-records/*.md`: A directory of learning records, which capture what the user has learned. These are loosely equivalent to architectural decision records in software development - they capture non-obvious lessons and key insights that may need to be revised later, or drive future sessions. These should be used to calculate the zone of proximal development. They are titled `0001-<dash-case-name>.md`, where the number increments each time. Use the format in [LEARNING-RECORD-FORMAT.md](./LEARNING-RECORD-FORMAT.md).
- `./lessons/*.html`: A directory of lessons. A **lesson** is a single, self-contained HTML output that teaches one tightly-scoped concept tied to the mission. This is the primary unit of teaching a topic.
- `./walkthroughs/*.html`: A directory of walkthroughs. A **walkthrough** is a single HTML page that leads the user through one real task, step by step, with the reason behind every step. Titled `0001-<dash-case-name>.html`. Use the format in [WALKTHROUGH-FORMAT.md](./WALKTHROUGH-FORMAT.md).
- Exercises: the files the user works in, checked by a test or a command. They live where `NOTES.md` or the workspace's own instructions say; otherwise in `./exercises/0001-<dash-case-name>/`, numbered like the lesson they belong to.
- `./assets/*`: Reusable **components** shared across lessons and walkthroughs. See [Assets](#assets).
- `NOTES.md`: A scratchpad for you to jot down user preferences, or working notes.

## Philosophy

To learn at a deep level, the user needs three things:

- **Knowledge**, captured from high-quality, high-trust resources
- **Skills**, acquired through highly-relevant interactive lessons devised by you, based on the knowledge
- **Wisdom**, which comes from interacting with other learners and practitioners

Before the `RESOURCES.md` is well-populated, your focus should be to find high-quality resources which will help the user acquire knowledge. Never trust your parametric knowledge.

Some topics may require more skills than knowledge. Learning more about theoretical physics might be more knowledge-based. For yoga, more skills-based.

### Real tasks carry the learning

People learn best when the teaching is built around a real task they care about: their prior knowledge is activated, the new step is shown on the same kind of task, they do it themselves with feedback, and they use it in their own work. Two kinds of information support the task, and they belong in different places:

- **Supportive information** is the why: the mental model, how the thing works underneath, the trade-offs. Give it before the task and beside each step.
- **Procedural information** is the how: the exact command, click or line of code. Give it at the moment it is needed, where the user does the step: in the walkthrough step, in the exercise file's comments.

A lesson that only shows syntax, or a step that only says what to click, leaves out half. The user then can follow along but cannot do it again alone.

### Fluency vs Storage Strength

You should be careful to split between two types of learning:

- **Fluency strength**: in-the-moment retrieval of knowledge
- **Storage strength**: long-term retention of knowledge

Fluency can give the user an illusory sense of mastery, but storage strength is the real goal. Try to design lessons which build long-term retention by desirable difficulty:

- Using retrieval practice (recall from memory)
- Spacing (distributing practice over time)
- Interleaving (mixing up different but related topics in practice - for skills practice only)
- Self-explanation (the user explains in their own words why something works)

## Grounding In Sources

Every lesson and walkthrough is written from sources read in this session, not from memory. Before writing:

1. Find the primary source for the exact version the user works with: the official documentation, the language specification, the vendor's guide. Read the pages the lesson or task touches.
2. Note what the sources say about prerequisites, permissions, limits and versions; these are the facts a learner trips over.
3. Put each source in `RESOURCES.md` with its one-line annotation.

On the page, every non-trivial claim and every step links to the section it came from. Name the date the sources were read near the top, so the user can tell when the page may be out of date. Where a source is silent or ambiguous, say so instead of filling the gap.

Facts about the user's own organisation that a page needs, such as which product they use, which chat channel, which project, or who holds admin rights: call the Skill tool with `cj-project-context`. If it finds none, ask the user. Never invent one.

## Lessons

A lesson is the unit in which knowledge and skills of a topic reach the user. Each lesson is one self-contained HTML file, saved to `./lessons/` and titled `0001-<dash-case-name>.html` where the number increments each time.

A lesson should be **beautiful**, with clean, readable typography and layout, since the user will return to these later to review. Think Tufte.

### Scope and depth

A lesson covers one concept, and covers it in full. Keep the scope small, not the explanation. For the concept, the lesson answers:

- **What** it is, in plain words and in the glossary's terms.
- **Why** it exists: the problem it solves, and what goes wrong without it.
- **How it works underneath**: the mechanism, at the level the user needs to predict its behaviour.
- **When not** to use it, and what to use instead.
- **Typical mistakes**, with the error message or symptom the user will see.

Manage working memory by structure, not by cutting the why: short sections, one idea each, diagrams where the mechanism has moving parts, and deeper material in expandable `<details>` sections. Each lesson should give the user a single tangible win that they can build on, directly tied to the mission and in their zone of proximal development.

### From example to exercise

The lesson shows the skill on the same kind of task the user will practise, so nothing has to be translated between page and exercise. Settle the exercise first: read it if it exists, otherwise write it before the page, so the examples can take its shape. When the concept does not fit the existing exercise, write a new exercise that does rather than bend the example. Then the page runs:

1. **Recall**: from the second lesson on, the page opens with two or three questions on earlier lessons, answered from memory before a click reveals the answer.
2. **Worked example**: a complete solution to a task of the same shape as the exercise, with the reasoning written beside each line ("why `strings.Cut` and not `Split`: we need exactly one split and the found flag").
3. **Partly worked example**: the same kind of task with the key step left for the user to complete.
4. **Exercise**: the user's own task. When exercises live in files, the hints live there too: each one names the function or tool to use and shows one input with its output.

The more the learning records show the user already knows, the shorter the worked example's reasoning gets; the partly worked example always stays.

### Running a lesson

1. Write the page as above, save it to `./lessons/` and open it.
2. When the user has read it, ask one why-question about the concept in the chat and give feedback on the reasoning. Ask more, one per message, while the answers show gaps.
3. The user works on the exercise; answer questions in the chat.
4. Close as [Closing A Session](#closing-a-session) says.

### Around the lesson

Each lesson should link via HTML anchors to other lessons and reference documents.

Each lesson should recommend a primary source for the user to read or watch. This should be the most high-quality, high-trust resource you found on the topic.

Each lesson should contain a reminder to ask followup questions to the agent. The agent is their teacher, and can assist with anything that's unclear.

## Walkthroughs

A walkthrough teaches a task the user is about to do in a real system. The user does every step themselves; that is what makes it stick. You may read the system's state to check a step when a tool allows it, but you change nothing in it.

Before writing any file, check the current directory. When it is a project repository rather than a teaching workspace (no `MISSION.md`, no `./walkthroughs/`, but code of its own), ask the user where the workspace should live, a separate directory recommended, so no teaching files land in a shared repository.

1. Name the outcome in one sentence and the facts the task depends on (which product and version, accounts, plan or permission level, which project, which channel). Get the organisation's facts as [Grounding In Sources](#grounding-in-sources) says.
2. Read the official docs for every part of the chain, as [Grounding In Sources](#grounding-in-sources) says.
3. Write the page in the format of [WALKTHROUGH-FORMAT.md](./WALKTHROUGH-FORMAT.md), save it to `./walkthroughs/` and open it.
4. While the user works through it, answer questions in the chat. When they report a step as done, check it if you can.
5. After the last step, ask the why-questions at the end of the page, one per message, and give feedback on each answer.
6. Close as [Closing A Session](#closing-a-session) says. A walkthrough whose concepts the user wants to go deeper into can become the mission of a topic later.

## Assets

Lessons and walkthroughs are built from reusable **components**, stored in `./assets/`: stylesheets, quiz widgets, simulators, diagram helpers, and anything else a second page could reuse.

Reuse is the default, not the exception. Before authoring a page, read `./assets/` and build from the components already there. When a page needs something new and reusable, write it as a component in `./assets/` and link to it; never inline code a future page would duplicate.

A shared stylesheet is the first component every workspace earns: every page links it, so the pages look like one consistent course rather than a pile of one-offs. As the workspace grows, so should the component library.

## The Mission

Every lesson should be tied into the mission - the reason that the user is interested in learning about the topic.

If the user is unclear about the mission, or the `MISSION.md` is not populated, your first job should be to ask the user why they want to learn this. A walkthrough needs no `MISSION.md`: its task is its mission.

Failing to understand the mission will mean knowledge acquisition is not grounded in real-world goals. Lessons will feel too abstract. You will have no way of judging what the user should do next.

Missions may change as the user develops more skills and knowledge. This is normal - make sure to update the `MISSION.md` and add a learning record to capture the change. Confirm with the user before changing the mission.

## Zone Of Proximal Development

Each lesson, the user should always feel as if they are being challenged 'just enough'.

The user may specify an exact thing they want to learn. If they don't, figure out their zone of proximal development by:

- Reading their `learning-records` and `NOTES.md`
- Figuring out the right thing to teach them based on their mission
- Teach the most relevant thing that fits in their zone of proximal development

## Knowledge

Lessons should be designed around a skill the user is going to learn. The knowledge in the lesson should be what is required to acquire that skill and to understand why it works. You teach the knowledge first, then get the user to practice the skills via an interactive feedback loop.

Knowledge should first be gathered from trusted resources, as [Grounding In Sources](#grounding-in-sources) says. Lessons should be littered with citations - links to external resources to back up any claim made. This increases the trustworthiness of the lesson.

For acquiring knowledge, difficulty is the enemy. It eats working memory you need for understanding. Make the explanation easy to follow, not short.

## Skills

If knowledge is all about acquisition, skills are about durability and flexibility. Make the knowledge stick.

For skill acquisition, difficulty is the tool. Effortful retrieval is what builds storage strength. Skills should be taught through interactive lessons. There are several tools at your disposal:

- Exercises in the user's own files, checked by a test or a command
- Lessons which guide the user through a list of real-world steps to take (for instance, yoga poses)
- Light in-browser tasks and quizzes

Each of these should be based on a **feedback loop**, where the user receives feedback on their performance. This feedback loop should be as tight as possible, giving feedback immediately - and ideally automatically.

Prefer checks that make the user produce an answer over checks where they recognise one:

- **Predict, then reveal**: show code or a situation, ask what happens, hide the answer behind a click.
- **Why-questions**: ask why something works or what would break without it. The user answers in the chat, and you give feedback on the reasoning, not just the result.
- **Multiple choice** only to tell apart things that are easily confused. Each answer should be exactly the same number of words (and characters, if possible). Vary which position holds the correct answer across questions. Don't give the user any clues about the answer through formatting or order.

## Closing A Session

When the user has finished a lesson's exercises or a walkthrough's last step:

1. Write a learning record if [LEARNING-RECORD-FORMAT.md](./LEARNING-RECORD-FORMAT.md) says one is due.
2. Ask one question: what did the user have to look up elsewhere, or miss on the page, to get it done?
3. Write the answer into `NOTES.md` as a rule for the next page ("show the file skeleton before the first exercise"), and follow it from then on.

## Acquiring Wisdom

Wisdom comes from true real-world interaction - testing your skills outside the learning environment.

When the user asks a question that appears to require wisdom, your default posture should be to attempt to answer - but to ultimately delegate to a **community**.

A community is a place (online or offline) where the user can test their skills in the real world. This might be a forum, a subreddit, a real-world class (budget permitting) or a local interest group.

You should attempt to find high-reputation communities the user can join. If the user expresses a preference that they don't want to join a community, respect it.

## Reference Documents

While creating lessons, you should also create reference documents. Lessons can reference these documents - they are useful for tracking raw units of knowledge useful across lessons.

Lessons will rarely be revisited later - reference documents will be. They should be the compressed essence of the lesson, in a format designed for quick reference.

Some learning topics lend themselves to reference:

- Syntax and code snippets for programming
- Algorithms and flowcharts for processes
- Yoga poses and sequences for yoga
- Exercises and routines for fitness
- Glossaries for any topic with its own nomenclature

The glossary, `GLOSSARY.md`, in particular, is an essential reference. Once it is created, it should be adhered to in every lesson.

## Talking To The User

Status lines, summaries and questions in the chat follow this rule block:

Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

A why-question in the chat is one question per message, like any other, and it stays open: no numbered options, and no explanation in the question that gives the answer away. To open a file for the user, run a CLI command when one is available. Explaining the subject itself, on a page or in the chat, and feedback on the user's answers keep full sentences and the depth [Scope and depth](#scope-and-depth) asks for.

## `NOTES.md`

The user will sometimes express preferences of how they want to be taught, or things you should keep in mind. This is the place to record those preferences, so you can refer back to them when designing lessons or working with the user. Read it before every page you write.
