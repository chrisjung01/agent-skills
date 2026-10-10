# Glossary format

Write the glossary in the project's language. An existing glossary keeps its own shape; this is for a new one.

```md
# {Context name}

{One or two sentences: what this context covers.}

## {Cluster, e.g. Ordering}

**Order**:
{One or two sentences: what it is, not what it does.}
_Avoid_: Purchase, Transaction
_Code_: `OrderRecord`, `checkout()`

**Shipment**:
A set of order items that leaves the warehouse together; one order can have several.
_Avoid_: Delivery (means the hand-over at the door), Order
_Code_: `Parcel`

## Open

- **Account** vs **Customer**: the docs use both for whoever pays; not decided yet.
```

## Rules

- **Opinionated.** One term per concept; every other word for it goes under `_Avoid_`, with a short reason when the avoided word means something else here.
- **Tight.** One or two sentences per term. Define what it is.
- **Domain only.** Concepts a player, customer or domain expert would recognise. General programming concepts stay out even when the code uses them everywhere.
- **`_Code_`** only when identifiers differ from the term; it is the bridge between the language people speak and the names in the code.
- **Clusters** as `##` headings once the list grows; a flat list is fine while it is short.
- **Open** lists terms that are known to be ambiguous but not decided. A term leaves it only when decided.
- **Several contexts**: a `GLOSSARY-MAP.md` at the root lists each context's glossary, one line on what it covers, and how the contexts relate.
