//! Aurora UI content catalog — the data behind the launcher.
//!
//! Static, std-only sample content (media + apps) plus search. The backend
//! ([`litetv-launcher`]) serves this as JSON; the native shell will read the
//! same model directly. No I/O, no deps — fully testable.

/// A piece of playable media (movie / show / episode).
#[derive(Clone, Copy, Debug)]
pub struct MediaItem {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub genre: &'static str,
    pub accent: &'static str,
    /// Watch progress, 0..=100 (0 = not started).
    pub progress: u8,
}

/// An installed application.
#[derive(Clone, Copy, Debug)]
pub struct App {
    pub id: &'static str,
    pub name: &'static str,
    pub glyph: &'static str,
    pub accent: &'static str,
    pub category: &'static str,
}

/// A titled row of media for the home screen.
pub struct Row {
    pub title: &'static str,
    pub items: Vec<&'static MediaItem>,
}

/// A search hit across content types.
pub enum Hit {
    Media(&'static MediaItem),
    App(&'static App),
}

pub static MEDIA: &[MediaItem] = &[
    MediaItem { id: "aurora-origins", title: "Aurora Origins",  subtitle: "1h 52m", genre: "Documentary", accent: "#3A7AFE", progress: 42 },
    MediaItem { id: "neon-drift",     title: "Neon Drift",      subtitle: "2h 08m", genre: "Action",      accent: "#EA4335", progress: 71 },
    MediaItem { id: "quiet-tides",    title: "Quiet Tides",     subtitle: "1h 36m", genre: "Drama",       accent: "#34C759", progress: 15 },
    MediaItem { id: "starfall",       title: "Starfall",        subtitle: "2h 44m", genre: "Sci-Fi",      accent: "#5C6BC0", progress: 90 },
    MediaItem { id: "golden-hour",    title: "Golden Hour",     subtitle: "1h 21m", genre: "Romance",     accent: "#F4B400", progress: 33 },
    MediaItem { id: "the-long-road",  title: "The Long Road",   subtitle: "2h 02m", genre: "Adventure",   accent: "#8E7CFF", progress: 0 },
    MediaItem { id: "skyline",        title: "Skyline",         subtitle: "1h 47m", genre: "Thriller",    accent: "#3A7AFE", progress: 0 },
    MediaItem { id: "echoes",         title: "Echoes",          subtitle: "1h 58m", genre: "Mystery",     accent: "#EA4335", progress: 0 },
    MediaItem { id: "midnight-sun",   title: "Midnight Sun",    subtitle: "2h 12m", genre: "Sci-Fi",      accent: "#34C759", progress: 0 },
    MediaItem { id: "paper-boats",    title: "Paper Boats",     subtitle: "1h 29m", genre: "Drama",       accent: "#5C6BC0", progress: 0 },
    MediaItem { id: "lantern",        title: "Lantern",         subtitle: "1h 40m", genre: "Family",      accent: "#F4B400", progress: 0 },
    MediaItem { id: "driftwood",      title: "Driftwood",       subtitle: "1h 33m", genre: "Documentary", accent: "#8E7CFF", progress: 0 },
];

pub static APPS: &[App] = &[
    App { id: "streamly",  name: "Streamly",  glyph: "S",  accent: "#EA4335", category: "Streaming" },
    App { id: "tunebox",   name: "TuneBox",   glyph: "♪",  accent: "#34C759", category: "Music" },
    App { id: "playcube",  name: "PlayCube",  glyph: "▶",  accent: "#3A7AFE", category: "Streaming" },
    App { id: "newsnow",   name: "NewsNow",   glyph: "N",  accent: "#F4B400", category: "News" },
    App { id: "kidzone",   name: "KidZone",   glyph: "★",  accent: "#8E7CFF", category: "Kids" },
    App { id: "fittv",     name: "FitTV",     glyph: "✛",  accent: "#34C759", category: "Fitness" },
    App { id: "browser",   name: "Browser",   glyph: "◎",  accent: "#5C6BC0", category: "System" },
    App { id: "photos",    name: "Photos",    glyph: "❖",  accent: "#3A7AFE", category: "System" },
    App { id: "podcasts",  name: "Podcasts",  glyph: "◖",  accent: "#EA4335", category: "Audio" },
    App { id: "store",     name: "LiteStore", glyph: "⬡",  accent: "#F4B400", category: "System" },
    App { id: "settings",  name: "Settings",  glyph: "⚙",  accent: "#9AA0A6", category: "System" },
    App { id: "live-tv",   name: "Live TV",   glyph: "📺", accent: "#5C6BC0", category: "Live" },
];

fn by_id(id: &str) -> Option<&'static MediaItem> {
    MEDIA.iter().find(|m| m.id == id)
}

/// Home-screen rows (`docs/03-UI-Design.md` § Home Screen Layout).
pub fn home() -> Vec<Row> {
    let cont: Vec<&MediaItem> = MEDIA.iter().filter(|m| m.progress > 0).collect();
    let recommended: Vec<&MediaItem> = MEDIA.iter().collect();
    let new_releases: Vec<&MediaItem> = MEDIA.iter().rev().take(6).collect();
    vec![
        Row { title: "Continue Watching", items: cont },
        Row { title: "Recommended For You", items: recommended },
        Row { title: "New Releases", items: new_releases },
    ]
}

pub fn find_media(id: &str) -> Option<&'static MediaItem> {
    by_id(id)
}

pub fn find_app(id: &str) -> Option<&'static App> {
    APPS.iter().find(|a| a.id == id)
}

/// Universal search over media (title/genre) and apps (name/category).
pub fn search(query: &str) -> Vec<Hit> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    let mut hits = Vec::new();
    for m in MEDIA {
        if m.title.to_lowercase().contains(&q) || m.genre.to_lowercase().contains(&q) {
            hits.push(Hit::Media(m));
        }
    }
    for a in APPS {
        if a.name.to_lowercase().contains(&q) || a.category.to_lowercase().contains(&q) {
            hits.push(Hit::App(a));
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_rows_are_populated() {
        let h = home();
        assert_eq!(h.len(), 3);
        assert!(h.iter().all(|r| !r.items.is_empty()));
        // Continue Watching only holds started items.
        assert!(h[0].items.iter().all(|m| m.progress > 0));
    }

    #[test]
    fn search_matches_title_genre_and_apps() {
        assert!(search("drama").iter().any(|h| matches!(h, Hit::Media(m) if m.genre == "Drama")));
        assert!(search("neon").iter().any(|h| matches!(h, Hit::Media(m) if m.id == "neon-drift")));
        assert!(search("music").iter().any(|h| matches!(h, Hit::App(a) if a.id == "tunebox")));
        assert!(search("").is_empty());
        assert!(search("zzzznomatch").is_empty());
    }

    #[test]
    fn lookups_resolve() {
        assert_eq!(find_media("starfall").unwrap().genre, "Sci-Fi");
        assert!(find_media("nope").is_none());
        assert_eq!(find_app("settings").unwrap().name, "Settings");
    }
}
