/*
 * Aurora UI — <aurora-card>
 *
 * The focusable media card: the centerpiece of remote/focus navigation.
 * Implements the focus contract from docs/01-Vision.md + docs/04-Design-System.md:
 *   on focus  -> scale slightly, glow softly, elevate, reveal quick actions.
 * Built only from design tokens (frameworks/aurora-ui/tokens.css) and degrades
 * gracefully under the reduced-motion tier.
 *
 * Usage:
 *   <aurora-card
 *     title="Blade Runner 2049"
 *     subtitle="2h 44m · Sci-Fi"
 *     image="poster.jpg"
 *     progress="0.42"            // 0..1, shows a Continue Watching bar
 *     accent="#5C6BC0">          // optional per-card accent
 *     <button slot="actions" data-action="play">▶</button>
 *     <button slot="actions" data-action="info">ⓘ</button>
 *   </aurora-card>
 *
 * Events:
 *   au-select  -> fired on Enter / click. detail: { id, title }
 *   au-action  -> fired when a slotted [data-action] is activated. detail: { action, id }
 */
const TEMPLATE = document.createElement("template");
TEMPLATE.innerHTML = `
  <style>
    :host {
      --_radius: var(--au-radius-md, 16px);
      --_dur: var(--au-dur-3, 250ms);
      display: inline-block;
      outline: none;
      cursor: pointer;
      -webkit-tap-highlight-color: transparent;
    }
    /* Reduced-motion tier: drop the transition entirely. */
    :host([reduce-motion]),
    :host-context(:root[data-au-motion="reduced"]) { --_dur: 0ms; }

    .card {
      position: relative;
      width: var(--au-card-w, 240px);
      background: var(--au-surface, #1B1D22);
      border-radius: var(--_radius);
      overflow: hidden;
      transform: translateZ(0);
      transition:
        transform var(--_dur) var(--au-ease-spring, ease),
        box-shadow var(--_dur) var(--au-ease-out, ease),
        background-color var(--_dur) var(--au-ease-out, ease);
      will-change: transform;
    }

    /* --- Focus state: scale + glow + elevate (the focus contract) --- */
    :host(:focus-visible) .card,
    :host(:focus) .card,
    :host([focused]) .card {
      transform: scale(var(--au-focus-scale, 1.06));
      box-shadow: var(--au-focus-glow);
      background: var(--au-surface-2, #23262D);
    }

    .poster {
      position: relative;
      aspect-ratio: 2 / 3;
      background: linear-gradient(135deg,
        color-mix(in srgb, var(--au-accent) 35%, var(--au-surface)),
        var(--au-surface));
      background-size: cover;
      background-position: center;
    }

    .meta {
      padding: var(--au-space-3, 12px) var(--au-space-4, 16px) var(--au-space-4, 16px);
    }
    .title {
      font: var(--au-heading, 600 24px/1.3 sans-serif);
      font-size: 18px;
      color: var(--au-text, #fff);
      margin: 0;
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }
    .subtitle {
      font: var(--au-caption, 500 14px/1.4 sans-serif);
      color: var(--au-text-2, #9AA0A6);
      margin: 4px 0 0;
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }

    /* Continue Watching progress bar */
    .progress {
      position: absolute; left: 0; right: 0; bottom: 0;
      height: 4px; background: var(--au-track, rgba(255,255,255,0.18));
    }
    .progress > i {
      display: block; height: 100%; width: var(--_pct, 0%);
      background: var(--au-accent);
      transition: width var(--au-dur-4, 350ms) var(--au-ease-out, ease);
    }
    :host(:not([progress])) .progress { display: none; }

    /* Quick actions — hidden until focus, then slide/fade in */
    .actions {
      position: absolute; inset-inline: var(--au-space-3, 12px);
      bottom: var(--au-space-3, 12px);
      display: flex; gap: var(--au-space-2, 8px);
      opacity: 0; transform: translateY(8px);
      transition: opacity var(--_dur) var(--au-ease-out, ease),
                  transform var(--_dur) var(--au-ease-out, ease);
      pointer-events: none;
    }
    :host(:focus-visible) .actions,
    :host(:focus) .actions,
    :host([focused]) .actions { opacity: 1; transform: none; pointer-events: auto; }

    ::slotted([slot="actions"]) {
      all: unset;
      display: grid; place-items: center;
      width: 40px; height: 40px;
      border-radius: var(--au-radius-sm, 12px);
      background: rgba(16,17,20,0.72);
      backdrop-filter: blur(6px);
      color: var(--au-text, #fff);
      font-size: 18px; cursor: pointer;
      transition: background var(--au-dur-1, 100ms) var(--au-ease-out, ease);
    }
    ::slotted([slot="actions"]:hover),
    ::slotted([slot="actions"]:focus-visible) { background: var(--au-accent); }
  </style>

  <div class="card" part="card">
    <div class="poster" part="poster">
      <div class="actions"><slot name="actions"></slot></div>
      <div class="progress"><i></i></div>
    </div>
    <div class="meta">
      <p class="title" part="title"></p>
      <p class="subtitle" part="subtitle"></p>
    </div>
  </div>
`;

export class AuroraCard extends HTMLElement {
  static get observedAttributes() {
    return ["title", "subtitle", "image", "progress", "accent"];
  }

  constructor() {
    super();
    this.attachShadow({ mode: "open" }).appendChild(
      TEMPLATE.content.cloneNode(true)
    );
    this._onKeyDown = this._onKeyDown.bind(this);
    this._onActionClick = this._onActionClick.bind(this);
  }

  connectedCallback() {
    // Focusable by remote/keyboard.
    if (!this.hasAttribute("tabindex")) this.setAttribute("tabindex", "0");
    this.setAttribute("role", "button");
    this.addEventListener("keydown", this._onKeyDown);
    this.addEventListener("click", () => this._select());
    this.shadowRoot
      .querySelector('slot[name="actions"]')
      .addEventListener("slotchange", () => this._wireActions());
    this._wireActions();
    this._render();
  }

  disconnectedCallback() {
    this.removeEventListener("keydown", this._onKeyDown);
  }

  attributeChangedCallback() {
    if (this.isConnected) this._render();
  }

  _render() {
    const sr = this.shadowRoot;
    sr.querySelector(".title").textContent = this.getAttribute("title") || "";
    sr.querySelector(".subtitle").textContent =
      this.getAttribute("subtitle") || "";

    const img = this.getAttribute("image");
    sr.querySelector(".poster").style.backgroundImage = img
      ? `url("${img}")`
      : "";

    const accent = this.getAttribute("accent");
    this.style.setProperty("--au-accent", accent || "var(--au-primary)");

    const pct = Math.max(0, Math.min(1, parseFloat(this.getAttribute("progress"))));
    this.style.setProperty("--_pct", Number.isFinite(pct) ? `${pct * 100}%` : "0%");

    this.setAttribute(
      "aria-label",
      [this.getAttribute("title"), this.getAttribute("subtitle")]
        .filter(Boolean)
        .join(", ")
    );
  }

  _wireActions() {
    this.querySelectorAll('[slot="actions"][data-action]').forEach((el) => {
      el.removeEventListener("click", this._onActionClick);
      el.addEventListener("click", this._onActionClick);
    });
  }

  _onActionClick(e) {
    e.stopPropagation(); // don't also fire au-select
    this.dispatchEvent(
      new CustomEvent("au-action", {
        bubbles: true,
        composed: true,
        detail: { action: e.currentTarget.dataset.action, id: this.id || null },
      })
    );
  }

  _onKeyDown(e) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      this._select();
    }
  }

  _select() {
    this.dispatchEvent(
      new CustomEvent("au-select", {
        bubbles: true,
        composed: true,
        detail: { id: this.id || null, title: this.getAttribute("title") },
      })
    );
  }
}

customElements.define("aurora-card", AuroraCard);
