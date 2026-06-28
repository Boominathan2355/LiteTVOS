# Aurora UI — Component Library

Reusable, framework-free Web Components for LiteTV OS, built strictly on the
design tokens in [`tokens.css`](tokens.css). Implements the visual identity,
focus contract, and motion rules from
[`docs/04-Design-System.md`](../../docs/04-Design-System.md).

## Try it

Open `demo.html` in a browser (no build step). On most systems:

```bash
# from frameworks/aurora-ui/
python3 -m http.server 8000   # then visit http://localhost:8000/demo.html
```

Use **← →** to move focus and **Enter** to select.

## Components

### `<aurora-card>`

The focusable media card — the centerpiece of remote/focus navigation. On focus
it **scales, glows, elevates, and reveals quick actions**, and it degrades to the
minimal motion tier under reduced-motion.

| Attribute | Type | Description |
|-----------|------|-------------|
| `title` | string | Primary label |
| `subtitle` | string | Secondary label (gray) |
| `image` | url | Poster image; falls back to an accent gradient |
| `progress` | 0..1 | Shows a Continue Watching bar |
| `accent` | css color | Per-card accent (defaults to `--au-primary`) |

**Slots:** `actions` — buttons with `data-action="..."` shown on focus.

**Events:**

- `au-select` — Enter/click. `detail: { id, title }`
- `au-action` — a slotted action activated. `detail: { action, id }`

```html
<link rel="stylesheet" href="tokens.css" />
<script type="module" src="components/aurora-card.js"></script>

<aurora-card title="Starfall" subtitle="2h 44m · Sci-Fi" progress="0.4">
  <button slot="actions" data-action="play" aria-label="Play">▶</button>
  <button slot="actions" data-action="info" aria-label="Info">ⓘ</button>
</aurora-card>
```

## Conventions

- **Tokens only.** Components consume CSS custom properties from `tokens.css`;
  never hard-code color, radius, or timing.
- **Focus-first.** Every interactive component is keyboard/remote focusable and
  honors the focus contract.
- **Motion budget.** No transition exceeds 400 ms; all transitions collapse to
  0 ms under the reduced-motion tier.
