//! LiteTV OS demo backend — std-only HTTP server (no external crates).
//!
//! A real device never runs this. It exists so the Aurora UI launcher and the
//! `aurora-focus` / `aurora-catalog` crates can be shown live from a hosted Web
//! Service that must bind `$PORT` and stay running (e.g. Render).
//!
//! Static:  GET /  ·  /tokens.css  ·  /components/aurora-card.js  ·  /icons.js
//! API:     GET /api/home  /api/featured  /api/glance  /api/apps
//!          GET /api/channels  /api/inputs  /api/recordings
//!          GET /api/search?q=  /api/item?id=
//!          GET /api/navigate?row=&col=&dir=   /api/specs  /api/state  /healthz

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use aurora_catalog as cat;
use aurora_focus::{Direction, FocusGrid, Row};

const DEMO_HTML: &str = include_str!("../../frameworks/aurora-ui/demo.html");
const TOKENS_CSS: &str = include_str!("../../frameworks/aurora-ui/tokens.css");
const CARD_JS: &str = include_str!("../../frameworks/aurora-ui/components/aurora-card.js");
const ICONS_JS: &str = include_str!("../../frameworks/aurora-ui/icons.js");

/// Focus grid mirroring the catalog's home rows, so /api/navigate stays in sync
/// with /api/home.
fn home_grid() -> FocusGrid {
    let rows = cat::home()
        .into_iter()
        .map(|r| Row::new(r.title, r.items.len()))
        .collect();
    FocusGrid::new(rows)
}

pub fn run(port: u16) {
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| {
        eprintln!("FATAL: cannot bind {addr}: {e}");
        std::process::exit(1);
    });
    println!("LiteTV OS · Aurora UI backend listening on http://{addr}");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || handle(s));
            }
            Err(e) => eprintln!("connection error: {e}"),
        }
    }
}

fn handle(mut stream: TcpStream) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    });
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    let target = request_line.split_whitespace().nth(1).unwrap_or("/");
    let (status, ctype, body) = route(target);
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {len}\r\n\
         Access-Control-Allow-Origin: *\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",
        len = body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
    let _ = stream.flush();
}

fn route(target: &str) -> (&'static str, &'static str, String) {
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    match path {
        "/" | "/index.html" | "/demo.html" => {
            ("200 OK", "text/html; charset=utf-8", DEMO_HTML.to_string())
        }
        "/tokens.css" => ("200 OK", "text/css; charset=utf-8", TOKENS_CSS.to_string()),
        "/components/aurora-card.js" => (
            "200 OK",
            "application/javascript; charset=utf-8",
            CARD_JS.to_string(),
        ),
        "/icons.js" => (
            "200 OK",
            "application/javascript; charset=utf-8",
            ICONS_JS.to_string(),
        ),
        "/api/home" => json(home_json()),
        "/api/featured" => json(featured_json()),
        "/api/glance" => json(glance_json()),
        "/api/apps" => json(apps_json()),
        "/api/channels" => json(channels_json()),
        "/api/inputs" => json(inputs_json()),
        "/api/recordings" => json(recordings_json()),
        "/api/search" => json(search_json(&param(query, "q").unwrap_or_default())),
        "/api/item" => item(&param(query, "id").unwrap_or_default()),
        "/api/navigate" => navigate(query),
        "/api/specs" => json(specs_json()),
        "/api/state" => json(state_json()),
        "/healthz" => ("200 OK", "text/plain", "ok".to_string()),
        _ => (
            "404 Not Found",
            "application/json",
            "{\"error\":\"not found\"}".to_string(),
        ),
    }
}

fn json(body: String) -> (&'static str, &'static str, String) {
    ("200 OK", "application/json", body)
}

// --- JSON builders --------------------------------------------------------

fn media_json(m: &cat::MediaItem) -> String {
    format!(
        "{{\"type\":\"media\",\"id\":\"{}\",\"title\":\"{}\",\"subtitle\":\"{}\",\
          \"genre\":\"{}\",\"accent\":\"{}\",\"rating\":\"{}\",\"progress\":{}}}",
        esc(m.id), esc(m.title), esc(m.subtitle), esc(m.genre), esc(m.accent), esc(m.rating), m.progress
    )
}

fn featured_json() -> String {
    let slides: Vec<String> = cat::featured()
        .iter()
        .filter_map(|f| {
            let m = cat::find_media(f.id)?;
            Some(format!(
                "{{\"headline\":\"{}\",\"blurb\":\"{}\",\"accent\":\"{}\",\"media\":{}}}",
                esc(f.headline), esc(f.blurb), esc(f.accent), media_json(m)
            ))
        })
        .collect();
    format!("{{\"slides\":[{}]}}", slides.join(","))
}

/// Demo "glance" widgets (storage / system / weather / reminder / profile).
/// Static sample values — a real device would source these from the platform.
fn glance_json() -> String {
    "{\"profile\":{\"name\":\"Boomi\",\"tier\":\"Premium\"},\
      \"storage\":{\"used_pct\":52,\"used_gb\":\"32\",\"total_gb\":\"64\"},\
      \"system\":{\"cpu_c\":45,\"ram_pct\":36},\
      \"weather\":{\"place\":\"Coimbatore, IN\",\"temp_c\":29,\"cond\":\"Clear\",\"hi_c\":33,\"lo_c\":24},\
      \"reminder\":{\"title\":\"Team Standup\",\"when\":\"Tomorrow · 10:00 AM\"},\
      \"tips\":\"Press and hold OK for more options.\"}"
        .to_string()
}

fn app_json(a: &cat::App) -> String {
    format!(
        "{{\"type\":\"app\",\"id\":\"{}\",\"name\":\"{}\",\"icon\":\"{}\",\
          \"accent\":\"{}\",\"category\":\"{}\"}}",
        esc(a.id), esc(a.name), esc(a.icon), esc(a.accent), esc(a.category)
    )
}

fn home_json() -> String {
    let rows: Vec<String> = cat::home()
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let items: Vec<String> = r.items.iter().map(|m| media_json(m)).collect();
            format!(
                "{{\"index\":{i},\"title\":\"{}\",\"items\":[{}]}}",
                esc(r.title),
                items.join(",")
            )
        })
        .collect();
    format!("{{\"shell\":\"Aurora UI\",\"rows\":[{}]}}", rows.join(","))
}

fn apps_json() -> String {
    let apps: Vec<String> = cat::APPS.iter().map(|a| app_json(a)).collect();
    format!("{{\"apps\":[{}]}}", apps.join(","))
}

fn channel_json(c: &cat::Channel) -> String {
    format!(
        "{{\"type\":\"channel\",\"number\":\"{}\",\"name\":\"{}\",\"category\":\"{}\",\
          \"accent\":\"{}\",\"now\":\"{}\",\"next\":\"{}\",\"source\":\"{}\"}}",
        esc(c.number), esc(c.name), esc(c.category), esc(c.accent),
        esc(c.now), esc(c.next), esc(c.source)
    )
}

fn input_json(i: &cat::Input) -> String {
    format!(
        "{{\"id\":\"{}\",\"name\":\"{}\",\"kind\":\"{}\"}}",
        esc(i.id), esc(i.name), esc(i.kind)
    )
}

fn channels_json() -> String {
    let ch: Vec<String> = cat::channels().iter().map(|c| channel_json(c)).collect();
    format!("{{\"channels\":[{}]}}", ch.join(","))
}

fn inputs_json() -> String {
    let inp: Vec<String> = cat::inputs().iter().map(|i| input_json(i)).collect();
    format!("{{\"inputs\":[{}]}}", inp.join(","))
}

fn recording_json(r: &cat::Recording) -> String {
    format!(
        "{{\"type\":\"recording\",\"id\":\"{}\",\"title\":\"{}\",\"channel\":\"{}\",\
          \"accent\":\"{}\",\"when\":\"{}\",\"duration\":\"{}\",\"status\":\"{}\"}}",
        esc(r.id), esc(r.title), esc(r.channel), esc(r.accent),
        esc(r.when), esc(r.duration), esc(r.status)
    )
}

fn recordings_json() -> String {
    let recs: Vec<String> = cat::recordings().iter().map(|r| recording_json(r)).collect();
    format!("{{\"recordings\":[{}]}}", recs.join(","))
}

fn search_json(q: &str) -> String {
    let hits: Vec<String> = cat::search(q)
        .iter()
        .map(|h| match h {
            cat::Hit::Media(m) => media_json(m),
            cat::Hit::App(a) => app_json(a),
            cat::Hit::Channel(c) => channel_json(c),
        })
        .collect();
    format!("{{\"query\":\"{}\",\"results\":[{}]}}", esc(q), hits.join(","))
}

fn item(id: &str) -> (&'static str, &'static str, String) {
    if let Some(m) = cat::find_media(id) {
        json(media_json(m))
    } else if let Some(a) = cat::find_app(id) {
        json(app_json(a))
    } else if let Some(c) = cat::find_channel(id) {
        json(channel_json(c))
    } else if let Some(r) = cat::find_recording(id) {
        json(recording_json(r))
    } else {
        (
            "404 Not Found",
            "application/json",
            "{\"error\":\"unknown id\"}".to_string(),
        )
    }
}

fn specs_json() -> String {
    "{\"total_ram_mb_max\":512,\"cpu_idle_pct_max\":2,\"boot_s_max\":20,\
      \"launcher_start_ms_max\":500,\"app_launch_s_max\":1,\"animation_fps\":60,\
      \"system_image_gb_max\":2.5}"
        .to_string()
}

fn navigate(query: &str) -> (&'static str, &'static str, String) {
    let dir = match param(query, "dir").and_then(parse_dir) {
        Some(d) => d,
        None => {
            return (
                "400 Bad Request",
                "application/json",
                "{\"error\":\"missing or invalid dir (up|down|left|right)\"}".to_string(),
            )
        }
    };
    let row = param(query, "row").and_then(|s| s.parse().ok()).unwrap_or(0);
    let col = param(query, "col").and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut grid = home_grid();
    grid.set_focus(row, col);
    let moved = grid.navigate(dir);
    let f = grid.focus();
    let title = grid.focused_row_title().unwrap_or("");
    json(format!(
        "{{\"dir\":\"{dir:?}\",\"moved\":{moved},\"row\":{},\"col\":{},\"title\":\"{}\"}}",
        f.row,
        f.col,
        esc(title)
    ))
}

fn state_json() -> String {
    let mut grid = home_grid();
    let script = [
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];
    let mut steps = vec![step_obj("initial", &grid)];
    for dir in script {
        grid.navigate(dir);
        steps.push(step_obj(&format!("{dir:?}"), &grid));
    }
    format!("{{\"shell\":\"Aurora UI\",\"trace\":[{}]}}", steps.join(","))
}

fn step_obj(action: &str, grid: &FocusGrid) -> String {
    let f = grid.focus();
    format!(
        "{{\"action\":\"{action}\",\"row\":{},\"col\":{},\"title\":\"{}\"}}",
        f.row,
        f.col,
        esc(grid.focused_row_title().unwrap_or(""))
    )
}

// --- helpers --------------------------------------------------------------

/// Minimal URL-decode (handles %XX and '+') for query values.
fn url_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'+' => {
                out.push(' ');
                i += 1;
            }
            b'%' if i + 2 < b.len() => {
                let h = std::str::from_utf8(&b[i + 1..i + 3]).ok();
                if let Some(v) = h.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                    out.push(v as char);
                    i += 3;
                } else {
                    out.push('%');
                    i += 1;
                }
            }
            c => {
                out.push(c as char);
                i += 1;
            }
        }
    }
    out
}

fn param(query: &str, key: &str) -> Option<String> {
    query.split('&').find_map(|kv| {
        let (k, v) = kv.split_once('=')?;
        (k == key).then(|| url_decode(v))
    })
}

/// Escape a string for embedding in JSON.
fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn parse_dir(s: String) -> Option<Direction> {
    match s.to_ascii_lowercase().as_str() {
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        "left" => Some(Direction::Left),
        "right" => Some(Direction::Right),
        _ => None,
    }
}
