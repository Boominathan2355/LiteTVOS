/*
 * Aurora UI — SVG icon set.
 *
 * Line icons on a 24×24 grid, drawn with `stroke="currentColor"` so they inherit
 * text color and re-theme automatically (no emoji, no external assets, no font).
 * Filled icons set their own `fill="currentColor" stroke="none"`.
 *
 * Usage:  el.innerHTML = icon("play");
 *         el.innerHTML = icon("camera", { size: 26 });
 */

const ICONS = {
  // --- actions ---
  play:    '<path d="M8 5v14l11-7z" fill="currentColor" stroke="none"/>',
  pause:   '<rect x="7" y="5" width="3.5" height="14" rx="1" fill="currentColor" stroke="none"/><rect x="13.5" y="5" width="3.5" height="14" rx="1" fill="currentColor" stroke="none"/>',
  stop:    '<rect x="6" y="6" width="12" height="12" rx="2" fill="currentColor" stroke="none"/>',
  record:  '<circle cx="12" cy="12" r="6" fill="currentColor" stroke="none"/>',
  close:   '<path d="M6 6l12 12M18 6L6 18"/>',
  check:   '<path d="M5 13l4 4L19 7"/>',
  info:    '<circle cx="12" cy="12" r="9"/><path d="M12 11v5"/><path d="M12 7.5h.01"/>',
  search:  '<circle cx="11" cy="11" r="7"/><path d="M21 21l-4.3-4.3"/>',

  // --- apps ---
  film:    '<rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 9.5h18M8 5v14M16 5v14"/>',
  music:   '<path d="M9 17V5l10-2v12"/><circle cx="6.5" cy="17" r="2.5"/><circle cx="15.5" cy="15" r="2.5"/>',
  playbox: '<rect x="3" y="3" width="18" height="18" rx="4"/><path d="M10 8.5l6 3.5-6 3.5z" fill="currentColor" stroke="none"/>',
  news:    '<path d="M4 5h13v14H4z"/><path d="M17 8h3v9a2 2 0 0 1-2 2"/><path d="M7 9h7M7 12h7M7 15h4"/>',
  star:    '<path d="M12 3.5l2.6 5.3 5.9.9-4.3 4.1 1 5.8L12 17l-5.2 2.6 1-5.8-4.3-4.1 5.9-.9z" fill="currentColor" stroke="none"/>',
  pulse:   '<path d="M3 12h4l3 8 4-16 3 8h4"/>',
  globe:   '<circle cx="12" cy="12" r="9"/><path d="M3 12h18"/><path d="M12 3c2.6 2.6 2.6 15.4 0 18M12 3c-2.6 2.6-2.6 15.4 0 18"/>',
  image:   '<rect x="3" y="4" width="18" height="16" rx="2"/><circle cx="8.5" cy="9.5" r="1.5"/><path d="M21 16l-5-5L5 20"/>',
  mic:     '<rect x="9" y="3" width="6" height="11" rx="3"/><path d="M6 11a6 6 0 0 0 12 0M12 17v4M9 21h6"/>',
  camera:  '<path d="M4 8h3l1.5-2h7L17 8h3a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1z"/><circle cx="12" cy="13" r="3.5"/>',
  bag:     '<path d="M6 8h12l-1 12H7z"/><path d="M9 8a3 3 0 0 1 6 0"/>',
  sliders: '<path d="M4 8h9M17 8h3M4 16h3M11 16h9"/><circle cx="15" cy="8" r="2"/><circle cx="9" cy="16" r="2"/>',
  tv:      '<rect x="3" y="7" width="18" height="12" rx="2"/><path d="M8 22h8M12 4l3 3M12 4L9 7"/>',

  // --- inputs / sources ---
  antenna: '<circle cx="12" cy="15" r="2" fill="currentColor" stroke="none"/><path d="M8.1 10.6a5.5 5.5 0 0 1 7.8 0M5.4 7.9a9.3 9.3 0 0 1 13.2 0"/>',
  hdmi:    '<path d="M6.5 8h11l-2 3.5v4.5h-7v-4.5z"/><path d="M9 7v1.5M12 7v1.5M15 7v1.5"/>',
  rca:     '<circle cx="6.5" cy="12" r="2.5"/><circle cx="12" cy="12" r="2.5"/><circle cx="17.5" cy="12" r="2.5"/>',
  monitor: '<rect x="3" y="4" width="18" height="13" rx="2"/><path d="M8 21h8M12 17v4"/>',
  jack:    '<circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="1.8" fill="currentColor" stroke="none"/>',

  // --- shell / chrome ---
  home:    '<path d="M4 11l8-7 8 7"/><path d="M6 10v9h5v-6h2v6h5v-9"/>',
  grid:    '<rect x="4" y="4" width="6" height="6" rx="1.5"/><rect x="14" y="4" width="6" height="6" rx="1.5"/><rect x="4" y="14" width="6" height="6" rx="1.5"/><rect x="14" y="14" width="6" height="6" rx="1.5"/>',
  media:   '<circle cx="12" cy="12" r="9"/><path d="M10 8.5l6 3.5-6 3.5z" fill="currentColor" stroke="none"/>',
  gamepad: '<rect x="2.5" y="8" width="19" height="9" rx="4"/><path d="M8 12.5H5M6.5 11v3"/><path d="M15.5 11.5h.01M18 13.5h.01"/>',
  cog:     '<circle cx="12" cy="12" r="3.2"/><path d="M12 2.5v3M12 18.5v3M21.5 12h-3M5.5 12h-3M18.7 5.3l-2.1 2.1M7.4 16.6l-2.1 2.1M18.7 18.7l-2.1-2.1M7.4 7.4L5.3 5.3"/>',
  user:    '<circle cx="12" cy="8" r="4"/><path d="M5 20a7 7 0 0 1 14 0"/>',
  wifi:    '<path d="M4.5 12.5a10 10 0 0 1 15 0M7.5 15.5a6 6 0 0 1 9 0"/><circle cx="12" cy="18.5" r="1" fill="currentColor" stroke="none"/>',
  folder:  '<path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>',
  bluetooth:'<path d="M7 7.5l10 9-5 4V3.5l5 4-10 9"/>',
  chevron: '<path d="M9 6l6 6-6 6"/>',
  sun:     '<circle cx="12" cy="12" r="4"/><path d="M12 2v2.5M12 19.5V22M2 12h2.5M19.5 12H22M5 5l1.7 1.7M17.3 17.3L19 19M5 19l1.7-1.7M17.3 6.7L19 5"/>',
  moon:    '<path d="M20 14.5A8 8 0 1 1 9.5 4 6.5 6.5 0 0 0 20 14.5z"/>',
  bell:    '<path d="M6 9a6 6 0 0 1 12 0c0 5 2 6 2 6H4s2-1 2-6z"/><path d="M10 19a2 2 0 0 0 4 0"/>',
  bulb:    '<path d="M9 18h6M10 21h4M8.5 14a5.5 5.5 0 1 1 7 0c-.8.7-1.5 1.4-1.5 2.5h-4c0-1.1-.7-1.8-1.5-2.5z"/>',
  cloud:   '<path d="M7 18a4 4 0 0 1 .4-8A6 6 0 0 1 19 11.5 3.5 3.5 0 0 1 18 18z"/>',
  calendar:'<rect x="3.5" y="5" width="17" height="16" rx="2"/><path d="M3.5 9.5h17M8 3v4M16 3v4"/>',
  plus:    '<path d="M12 5v14M5 12h14"/>',
  picture: '<rect x="3" y="5" width="18" height="13" rx="2"/><path d="M8 21h8"/><path d="M8 13l2.5-2.5L14 14l2-1.5L19 16"/>',
  volume:  '<path d="M4 9v6h4l5 4V5L8 9z"/><path d="M16.5 9a4 4 0 0 1 0 6"/>',

  dot:     '<circle cx="12" cy="12" r="3" fill="currentColor" stroke="none"/>',
};

/** Return inline SVG markup for a named icon (falls back to a dot). */
export function icon(name, opts) {
  const { size = 24, cls = "" } = opts || {};
  const inner = ICONS[name] || ICONS.dot;
  return (
    `<svg class="icn ${cls}" viewBox="0 0 24 24" width="${size}" height="${size}" ` +
    `fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" ` +
    `stroke-linejoin="round" aria-hidden="true" focusable="false">${inner}</svg>`
  );
}

export const hasIcon = (name) => Object.prototype.hasOwnProperty.call(ICONS, name);
