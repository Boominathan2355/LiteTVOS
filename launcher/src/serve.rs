//! Minimal std-only HTTP server for hosted demos (e.g. Render).
//!
//! A LiteTV device never runs this — it exists so the Aurora UI web prototype
//! and the focus engine can be shown live from a cloud Web Service that must
//! bind `$PORT` and stay running. No external crates.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use aurora_focus::{Direction, FocusGrid, Row};

// Embedded at compile time so there is no runtime filesystem dependency.
const DEMO_HTML: &str = include_str!("../../frameworks/aurora-ui/demo.html");
const TOKENS_CSS: &str = include_str!("../../frameworks/aurora-ui/tokens.css");
const CARD_JS: &str = include_str!("../../frameworks/aurora-ui/components/aurora-card.js");

/// Bind `0.0.0.0:port` and serve until killed.
pub fn run(port: u16) {
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| {
        eprintln!("FATAL: cannot bind {addr}: {e}");
        std::process::exit(1);
    });
    println!("LiteTV OS · Aurora UI demo server listening on http://{addr}");

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
    // "GET /path HTTP/1.1"
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");

    let (status, ctype, body) = route(path);
    let response = format!(
        "HTTP/1.1 {status}\r\n\
         Content-Type: {ctype}\r\n\
         Content-Length: {len}\r\n\
         Cache-Control: no-cache\r\n\
         Connection: close\r\n\r\n",
        len = body.len()
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(body.as_bytes());
    let _ = stream.flush();
}

fn route(path: &str) -> (&'static str, &'static str, String) {
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
        "/api/state" => ("200 OK", "application/json", state_json()),
        "/healthz" => ("200 OK", "text/plain", "ok".to_string()),
        _ => ("404 Not Found", "text/plain", "not found".to_string()),
    }
}

/// JSON trace of the focus engine driving the default home screen — the same
/// model the native shell uses, exposed for the live demo.
fn state_json() -> String {
    let mut grid = FocusGrid::new(vec![
        Row::new("Continue Watching", 6),
        Row::new("Pinned Apps", 8),
        Row::new("Media Recommendations", 12),
        Row::new("Recently Opened", 5),
    ]);
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
