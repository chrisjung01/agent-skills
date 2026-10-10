# Diagram patterns

Pick the pattern that makes the item's point; vary them across a page so cards do not all look the same. Keep each diagram around 320px tall so before and after sit side by side.

## Mermaid flowchart: dependencies and call flow

The default for "X calls Y calls Z". Colour problems with a class, never with prose.

```
flowchart LR
  A[Handler] --> B[Validator]
  B --> C[Repository]
  C -.leak.-> D[PricingClient]
  classDef leak stroke:#dc2626,stroke-width:2px;
  classDef deep fill:#0f172a,color:#e2e8f0,stroke:#0f172a;
  class C,D leak
```

In a half-width before/after column, a left-to-right chain of more than four nodes shrinks to unreadable text: use `flowchart TB` there, or split the chain.

Rules that keep Mermaid from breaking: node ids are plain words (`CC`, `WaveCtl`); labels with spaces, dots, slashes or parentheses go in quotes (`A["Game.gd _ready()"]`); line breaks inside a label are `<br/>`; no `end` as a node id.

## Mermaid flowchart: tickets and blocked-by edges

Waves as subgraphs, an edge from blocker to blocked, so the frontier reads left to right.

```
flowchart LR
  subgraph W0["Wave 0"]
    A["A: contract"]
    B["B: pure logic"]
  end
  subgraph W1["Wave 1"]
    C["C: entity"]
  end
  A --> C
  B --> C
```

## Mermaid sequence: round trips

For "before: six round trips; after: one".

## Mermaid xychart: a trend

```
xychart-beta
  title "Weekly users"
  x-axis ["W1", "W2", "W3", "W4"]
  y-axis "users" 0 --> 120
  bar [12, 40, 75, 96]
```

## Hand-built boxes

When Mermaid's layout fights the point, for example an "after" that should look like one heavy module with greyed internals: `<div>`s with borders, the `.deep` class for the heavy one, `.seam` for a dashed seam, inline SVG `<line>` for arrows.

## Cross-section: layers a call passes through

Stacked bands (`h-12 border-l-4`), one per layer. Before: many thin bands that do nothing. After: one thick band named after its responsibility.

## Mass diagram: interface as wide as implementation

Two rectangles per module, interface and implementation, heights proportional. Shallow: nearly equal. Deep: short interface, tall implementation.
