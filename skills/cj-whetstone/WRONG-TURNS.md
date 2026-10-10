# Wrong turns

| Wrong turn | Instead |
|---|---|
| Reading only calls with a `correction` flag | Run `show` on every open call. The regex misses most criticism. |
| Treating every "nein" as a correction | "nein, setz das um" is an approval. Read the agent text before it. |
| Fixing the skill whose window the feedback fell into | Check what the words refer to. |
| Encoding a one-time experiment ("ich teste das nur einmal") | Verdict `one-off`, no change, no memory. |
| Counting tool errors as skill defects | Only when the skill's own instruction produced the failing command. |
| Editing in the main checkout or before approval | Worktree, branch, approval, then edit. |
| Setting calls to `verworfen` without reading them | Unread calls stay `offen`. |
| Reading one session once per skill it called | Group the `open` lines by session; read it once, file rows per skill. |
| A subagent's or your own status word ("erledigt", "done") | Only the vocabulary in step 5; `status` refuses anything else. |
