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
    /// Critic-style score, 0.0..=10.0 (shown as a rating badge).
    pub rating: &'static str,
    /// Watch progress, 0..=100 (0 = not started).
    pub progress: u8,
}

/// A featured hero slide for the home banner.
#[derive(Clone, Copy, Debug)]
pub struct Featured {
    /// The media this slide promotes (resolves via [`find_media`]).
    pub id: &'static str,
    pub headline: &'static str,
    pub blurb: &'static str,
    pub accent: &'static str,
}

/// An installed application.
#[derive(Clone, Copy, Debug)]
pub struct App {
    pub id: &'static str,
    pub name: &'static str,
    /// Name of an Aurora UI SVG icon (see `frameworks/aurora-ui/icons.js`).
    pub icon: &'static str,
    pub accent: &'static str,
    pub category: &'static str,
}

/// A live TV channel (cable / antenna tuner or an external input).
#[derive(Clone, Copy, Debug)]
pub struct Channel {
    pub number: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub accent: &'static str,
    /// Now-playing program (simple EPG: now / next).
    pub now: &'static str,
    pub next: &'static str,
    /// Where the channel comes from: "Cable", "Antenna", or an input name.
    pub source: &'static str,
}

/// A signal input / source.
#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub id: &'static str,
    pub name: &'static str,
    /// "Tuner", "HDMI", "Component", "VGA", or "Composite".
    pub kind: &'static str,
}

/// A DVR recording — completed, in-progress, or scheduled.
#[derive(Clone, Copy, Debug)]
pub struct Recording {
    pub id: &'static str,
    pub title: &'static str,
    /// Source channel number (matches a [`Channel::number`]).
    pub channel: &'static str,
    pub accent: &'static str,
    /// Human-readable start time.
    pub when: &'static str,
    pub duration: &'static str,
    /// "Recorded", "Recording", or "Scheduled".
    pub status: &'static str,
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
    Channel(&'static Channel),
}

pub static MEDIA: &[MediaItem] = &[
    MediaItem { id: "aurora-origins", title: "Aurora Origins",  subtitle: "S1 · E4", genre: "Documentary", accent: "#3A7AFE", rating: "8.4", progress: 42 },
    MediaItem { id: "neon-drift",     title: "Neon Drift",      subtitle: "2h 08m", genre: "Action",      accent: "#EA4335", rating: "8.8", progress: 71 },
    MediaItem { id: "quiet-tides",    title: "Quiet Tides",     subtitle: "1h 36m", genre: "Drama",       accent: "#34C759", rating: "7.9", progress: 15 },
    MediaItem { id: "starfall",       title: "Starfall",        subtitle: "S2 · E8", genre: "Sci-Fi",      accent: "#5C6BC0", rating: "9.1", progress: 90 },
    MediaItem { id: "golden-hour",    title: "Golden Hour",     subtitle: "1h 21m", genre: "Romance",     accent: "#F4B400", rating: "7.6", progress: 33 },
    MediaItem { id: "the-long-road",  title: "The Long Road",   subtitle: "2h 02m", genre: "Adventure",   accent: "#8E7CFF", rating: "8.0", progress: 0 },
    MediaItem { id: "skyline",        title: "Skyline",         subtitle: "1h 47m", genre: "Thriller",    accent: "#3A7AFE", rating: "8.3", progress: 0 },
    MediaItem { id: "echoes",         title: "Echoes",          subtitle: "1h 58m", genre: "Mystery",     accent: "#EA4335", rating: "7.7", progress: 0 },
    MediaItem { id: "midnight-sun",   title: "Midnight Sun",    subtitle: "S3 · E4", genre: "Sci-Fi",      accent: "#34C759", rating: "8.6", progress: 0 },
    MediaItem { id: "paper-boats",    title: "Paper Boats",     subtitle: "1h 29m", genre: "Drama",       accent: "#5C6BC0", rating: "7.4", progress: 0 },
    MediaItem { id: "lantern",        title: "Lantern",         subtitle: "1h 40m", genre: "Family",      accent: "#F4B400", rating: "8.1", progress: 0 },
    MediaItem { id: "driftwood",      title: "Driftwood",       subtitle: "1h 33m", genre: "Documentary", accent: "#8E7CFF", rating: "8.2", progress: 0 },
];

/// Hero slides for the home banner carousel.
pub static FEATURED: &[Featured] = &[
    Featured { id: "starfall",      headline: "Discover Your Next Adventure", blurb: "Explore unlimited movies, shows and more.", accent: "#5C6BC0" },
    Featured { id: "neon-drift",    headline: "Tonight, Drive Into the Neon", blurb: "High-octane action, mastered for 4K HDR.",   accent: "#EA4335" },
    Featured { id: "midnight-sun",  headline: "Worlds Beyond the Horizon",    blurb: "New sci-fi originals, only on LiteTV.",      accent: "#34C759" },
    Featured { id: "aurora-origins",headline: "The Story of the Light",       blurb: "A documentary series in stunning detail.",   accent: "#3A7AFE" },
];

pub fn featured() -> &'static [Featured] {
    FEATURED
}

pub static APPS: &[App] = &[
    App { id: "streamly",  name: "Streamly",  icon: "film",    accent: "#EA4335", category: "Streaming" },
    App { id: "tunebox",   name: "TuneBox",   icon: "music",   accent: "#34C759", category: "Music" },
    App { id: "playcube",  name: "PlayCube",  icon: "playbox", accent: "#3A7AFE", category: "Streaming" },
    App { id: "newsnow",   name: "NewsNow",   icon: "news",    accent: "#F4B400", category: "News" },
    App { id: "kidzone",   name: "KidZone",   icon: "star",    accent: "#8E7CFF", category: "Kids" },
    App { id: "fittv",     name: "FitTV",     icon: "pulse",   accent: "#34C759", category: "Fitness" },
    App { id: "browser",   name: "Browser",   icon: "globe",   accent: "#5C6BC0", category: "System" },
    App { id: "photos",    name: "Photos",    icon: "image",   accent: "#3A7AFE", category: "System" },
    App { id: "podcasts",  name: "Podcasts",  icon: "mic",     accent: "#EA4335", category: "Audio" },
    App { id: "camera",    name: "Camera",    icon: "camera",  accent: "#34C759", category: "System" },
    App { id: "store",     name: "LiteStore", icon: "bag",     accent: "#F4B400", category: "System" },
    App { id: "settings",  name: "Settings",  icon: "sliders", accent: "#9AA0A6", category: "System" },
    App { id: "live-tv",   name: "Live TV",   icon: "tv",      accent: "#5C6BC0", category: "Live" },
];

/// Live TV channels — a mix of cable (QAM) and antenna (ATSC) sources.
pub static CHANNELS: &[Channel] = &[
    Channel { number: "2.1",  name: "Aurora News 24", category: "News",        accent: "#EA4335", now: "World Tonight",        next: "Market Watch", source: "Antenna" },
    Channel { number: "4.1",  name: "Skyline Sports",  category: "Sports",      accent: "#34C759", now: "Live: City vs United", next: "Postgame",     source: "Cable" },
    Channel { number: "5.1",  name: "Cinephile",       category: "Movies",      accent: "#5C6BC0", now: "Neon Drift",           next: "Starfall",     source: "Cable" },
    Channel { number: "7.1",  name: "KidZone TV",      category: "Kids",        accent: "#8E7CFF", now: "Lantern Tales",        next: "Paper Boats",  source: "Antenna" },
    Channel { number: "9.1",  name: "TuneBox Live",    category: "Music",       accent: "#F4B400", now: "Indie Hour",           next: "Top 40",       source: "Cable" },
    Channel { number: "11.1", name: "Discovery+",      category: "Documentary", accent: "#3A7AFE", now: "Aurora Origins",       next: "Driftwood",    source: "Cable" },
    Channel { number: "13.1", name: "Local 13",        category: "Lifestyle",   accent: "#34C759", now: "Cooking Coast",        next: "Home Fix",     source: "Antenna" },
    Channel { number: "22.1", name: "Cine Action",     category: "Movies",      accent: "#EA4335", now: "The Long Road",        next: "Echoes",       source: "Cable" },
];

/// Signal inputs: tuners and external sources.
pub static INPUTS: &[Input] = &[
    Input { id: "antenna", name: "Antenna (ATSC)",    kind: "Tuner" },
    Input { id: "cable",   name: "Cable (QAM)",       kind: "Tuner" },
    Input { id: "hdmi1",   name: "HDMI 1",            kind: "HDMI" },
    Input { id: "hdmi2",   name: "HDMI 2",            kind: "HDMI" },
    Input { id: "hdmi3",   name: "HDMI 3 (eARC)",     kind: "HDMI" },
    Input { id: "comp",    name: "Component (YPbPr)", kind: "Component" },
    Input { id: "vga",     name: "VGA (PC)",          kind: "VGA" },
    Input { id: "av1",     name: "AV 1",              kind: "Composite" },
    Input { id: "av2",     name: "AV 2",              kind: "Composite" },
];

/// DVR library — recorded, recording, and scheduled programs.
pub static RECORDINGS: &[Recording] = &[
    Recording { id: "rec-1", title: "World Tonight",        channel: "2.1",  accent: "#EA4335", when: "Today · 8:00 PM",  duration: "1h 00m", status: "Recorded"  },
    Recording { id: "rec-2", title: "Live: City vs United", channel: "4.1",  accent: "#34C759", when: "Now",             duration: "2h 30m", status: "Recording" },
    Recording { id: "rec-3", title: "Aurora Origins",       channel: "11.1", accent: "#3A7AFE", when: "Today · 9:00 PM",  duration: "1h 52m", status: "Scheduled" },
    Recording { id: "rec-4", title: "Cooking Coast",        channel: "13.1", accent: "#34C759", when: "Yesterday",       duration: "0h 30m", status: "Recorded"  },
    Recording { id: "rec-5", title: "Top 40",               channel: "9.1",  accent: "#F4B400", when: "Tomorrow · 6:00 PM", duration: "1h 00m", status: "Scheduled" },
];

pub fn recordings() -> &'static [Recording] {
    RECORDINGS
}

pub fn find_recording(id: &str) -> Option<&'static Recording> {
    RECORDINGS.iter().find(|r| r.id == id)
}

pub fn channels() -> &'static [Channel] {
    CHANNELS
}

pub fn find_channel(number: &str) -> Option<&'static Channel> {
    CHANNELS.iter().find(|c| c.number == number)
}

pub fn inputs() -> &'static [Input] {
    INPUTS
}

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
    for c in CHANNELS {
        if c.name.to_lowercase().contains(&q)
            || c.category.to_lowercase().contains(&q)
            || c.now.to_lowercase().contains(&q)
        {
            hits.push(Hit::Channel(c));
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

    #[test]
    fn live_tv_channels_and_inputs() {
        assert!(!channels().is_empty());
        assert_eq!(find_channel("4.1").unwrap().category, "Sports");
        assert!(find_channel("99.9").is_none());
        // Both cable and antenna sources are present.
        assert!(channels().iter().any(|c| c.source == "Cable"));
        assert!(channels().iter().any(|c| c.source == "Antenna"));
        // Tuner, HDMI, Component, VGA, and Composite inputs all exist.
        for k in ["Tuner", "HDMI", "Component", "VGA", "Composite"] {
            assert!(inputs().iter().any(|i| i.kind == k), "missing input kind {k}");
        }
        // Two composite AV inputs.
        assert_eq!(inputs().iter().filter(|i| i.kind == "Composite").count(), 2);
    }

    #[test]
    fn camera_app_present() {
        assert_eq!(find_app("camera").unwrap().name, "Camera");
    }

    #[test]
    fn featured_slides_reference_real_media() {
        assert!(!featured().is_empty());
        assert!(featured().iter().all(|f| find_media(f.id).is_some()));
        // Every media item carries a rating badge value.
        assert!(MEDIA.iter().all(|m| !m.rating.is_empty()));
    }

    #[test]
    fn dvr_recordings_resolve_and_reference_channels() {
        assert!(!recordings().is_empty());
        let r = find_recording("rec-2").unwrap();
        assert_eq!(r.status, "Recording");
        assert!(find_recording("rec-nope").is_none());
        // Every recording points at a real channel.
        assert!(recordings().iter().all(|r| find_channel(r.channel).is_some()));
        // All three lifecycle states are represented.
        for s in ["Recorded", "Recording", "Scheduled"] {
            assert!(recordings().iter().any(|r| r.status == s), "missing status {s}");
        }
    }

    #[test]
    fn search_includes_channels() {
        assert!(search("sports").iter().any(|h| matches!(h, Hit::Channel(c) if c.number == "4.1")));
        // matches a now-playing program too
        assert!(search("aurora origins").iter().any(|h| matches!(h, Hit::Channel(_))));
    }
}
