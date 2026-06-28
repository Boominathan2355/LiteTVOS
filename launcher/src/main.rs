//! LiteTV OS launcher — Aurora UI shell entry point.
//!
//! Today: a headless run that builds the home screen, drives the focus engine
//! from a scripted remote sequence, and "presents" each frame through the
//! [`compositor`] backend. The Smithay+Skia backend slots in behind the
//! `render` feature without touching this control flow.

mod compositor;
mod serve;

use aurora_focus::{Direction, FocusGrid, Row};
use compositor::{Backend, Frame, HeadlessBackend};

/// Build the default home screen (`docs/03-UI-Design.md` § Home Screen Layout).
fn home_screen() -> FocusGrid {
    FocusGrid::new(vec![
        Row::new("Continue Watching", 6),
        Row::new("Pinned Apps", 8),
        Row::new("Media Recommendations", 12),
        Row::new("Recently Opened", 5),
    ])
}

fn main() {
    // Hosted Web Service (Render et al.) sets PORT and needs a long-running
    // process bound to it — serve the Aurora UI demo. Otherwise run headless.
    if let Ok(port) = std::env::var("PORT") {
        let port: u16 = port.parse().unwrap_or(10000);
        serve::run(port);
        return;
    }

    println!("LiteTV OS · Aurora UI shell (headless)");
    println!("Surface: {:?}\n", aurora_tokens::palette::BG.rgba());

    let mut grid = home_screen();
    let mut backend = HeadlessBackend;

    // A scripted "remote" walk to exercise navigation end-to-end.
    let script = [
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];

    let titles: Vec<String> = grid.rows().iter().map(|r| r.title.clone()).collect();
    let present = |grid: &FocusGrid, backend: &mut HeadlessBackend| {
        backend.present(&Frame { row_titles: &titles, focus: grid.focus(), fps: 60 });
    };

    println!("Initial:");
    present(&grid, &mut backend);

    for dir in script {
        let moved = grid.navigate(dir);
        println!("\nremote: {dir:?}{}", if moved { "" } else { " (edge — no move)" });
        present(&grid, &mut backend);
    }
}
