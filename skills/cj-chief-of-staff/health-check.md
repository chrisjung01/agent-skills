# Health check

Runs first in every mode. One cheap call per source the run's scope lists in `state/context.md`, before any real read, so a missing source is named instead of silently leaving a gap.

| Source | Probe |
|---|---|
| Chat | look up the user's own profile or search one message from them |
| Git host | `gh auth status`, then `gh api user` |
| External tracker | the tracker's current-user call |
| Error tracker | list organisations; the one in `context.md` must be there |
| Docs | fetch the user's own profile or the goals page from `context.md` |
| Notes vault | the vault path and its projects folder exist |

Result per source: `ok`, or `missing (<reason in a few words>)`, such as not connected, auth expired, project not found.

- A missing source: skip its reads; the brief's second line says `Missing: <source> (<reason>)`, one per source.
- All sources missing: reply with the missing lines only and stop.
- No retries, no reconnecting on the user's behalf. Name the fix in the reason when it is obvious ("reconnect the connector").
