//! LiteTV OS demo backend — std-only HTTP server (no external crates).
//!
//! A real device never runs this. It exists so the Aurora UI prototype and the
//! `aurora-focus` engine can be shown live from a hosted Web Service that must
//! bind `$PORT` and stay running (e.g. Render).
//!
//! Static:
//!   GET /                              the Aurora UI web demo
//!   GET /tokens.css, /components/aurora-card.js
//! Backend API (JSON):
//!   GET /api/home                      home-screen rows
//!   GET /api/navigate?row=&col=&dir=   stateless focus move (dir=up|down|left|right)
//!   GET /api/state                     scripted navigation trace
//!   GET /api/specs                     engineering targets
//!   GET /healthz                       health check

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use aurora_focus::{Direction, FocusGrid, Row};

const DEMO_HTML: &str = include_str!("../../frameworks/aurora-ui/demo.html");
const TOKENS_CSS: &str = include_str!("../../frameworks/aurora-ui/tokens.css");
const CARD_JS: &str = include_str!("../../frameworks/aurora-ui/components/aurora-card.js");

/// The default home screen, single source of truth for the backend.
fn home_grid() -> FocusGrid {
    FocusGrid::new(vec![
        Row::new("Continue Watching", 6),
        Row::new("Pinned Apps", 8),
        Row::new("Media Recommendations", 12),
        Row::new("Recently Opened", 5),
    ])
}

/// Bind `0.0.0.0:port` and serve until killed.
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
    let response = format!(
        "HTTP/1.1 {status}\r\n\
         Content-Type: {ctype}\r\n\
         Content-Length: {len}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Cache-Control: no-cache\r\n\
         Connection: close\r\n\r\n",
        len = body.len()
    );
    let _ = stream.write_all(response.as_bytes());
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
        "/api/home" => ("200 OK", "application/json", home_json()),
        "/api/navigate" => navigate(query),
        "/api/state" => ("200 OK", "application/json", state_json()),
        "/api/specs" => ("200 OK", "application/json", specs_json()),
        "/healthz" => ("200 OK", "text/plain", "ok".to_string()),
        _ => (
            "404 Not Found",
            "application/json",
            "{\"error\":\"not found\"}".to_string(),
        ),
    }
}

// --- API handlers ---------------------------------------------------------

fn home_json() -> String {
    let grid = home_grid();
    let rows: Vec<String> = grid
        .rows()
        .iter()
        .enumerate()
        .map(|(i, r)| format!("{{\"index\":{i},\"title\":\"{}\",\"items\":{}}}", r.title, r.len))
        .collect();
    format!("{{\"shell\":\"Aurora UI\",\"rows\":[{}]}}", rows.join(","))
}

/// Stateless focus move: client passes current `row`/`col` and a `dir`.
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
    (
        "200 OK",
        "application/json",
        format!(
            "{{\"dir\":\"{dir:?}\",\"moved\":{moved},\"row\":{},\"col\":{},\"title\":\"{title}\"}}",
            f.row, f.col
        ),
    )
}

/// Engineering targets (docs/13-Roadmap.md), served as JSON.
fn specs_json() -> String {
    "{\"total_ram_mb_max\":512,\"cpu_idle_pct_max\":2,\"boot_s_max\":20,\
      \"launcher_start_ms_max\":500,\"app_launch_s_max\":1,\"animation_fps\":60,\
      \"system_image_gb_max\":2.5}"
        .to_string()
}

/// Scripted navigation trace — same model the native shell runs.
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
    let title = grid.focused_row_title().unwrap_or("");
    format!(
        "{{\"action\":\"{action}\",\"row\":{},\"col\":{},\"title\":\"{title}\"}}",
        f.row, f.col
    )
}

// --- tiny query helpers ---------------------------------------------------

fn param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|kv| {
        let (k, v) = kv.split_once('=')?;
        (k == key).then_some(v)
    })
}

fn parse_dir(s: &str) -> Option<Direction> {
    match s.to_ascii_lowercase().as_str() {
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        "left" => Some(Direction::Left),
        "right" => Some(Direction::Right),
        _ => None,
    }
}
