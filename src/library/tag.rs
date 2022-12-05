use lofty::{ItemKey, Tag};
use std::time::Duration;

use super::errors::ImportError;

pub const UNKNOWN_ARTIST: &str = "Unknown Artist";
pub const UNKNOWN_ALBUM: &str = "Unkwon Album";

pub fn get_title(tag: &Tag) -> Result<String, Box<ImportError>> {
    match tag.get_string(&ItemKey::TrackTitle) {
        Some(title) => Ok(title.to_string()),
        None => return Err(Box::new(ImportError::MissingData)),
    }
}

pub fn get_track_artist(tag: &Tag) -> String {
    tag.get_string(&ItemKey::TrackArtist)
        .unwrap_or(UNKNOWN_ARTIST)
        .to_string()
}

pub fn get_album_title(tag: &Tag) -> String {
    tag.get_string(&ItemKey::AlbumTitle)
        .unwrap_or(UNKNOWN_ALBUM)
        .to_string()
}

pub fn get_album_artist(tag: &Tag) -> String {
    tag.get_string(&ItemKey::AlbumArtist)
        .unwrap_or(UNKNOWN_ARTIST)
        .to_string()
}

pub fn get_year(tag: &Tag) -> Option<String> {
    match tag.get_string(&ItemKey::Year) {
        Some(year) => Some(year.to_string()),
        None => None,
    }
}

//TODO: use this in Song
pub fn get_lyrics(tag: &Tag) -> Option<String> {
    match tag.get_string(&ItemKey::Lyrics) {
        Some(lyrics) => Some(lyrics.to_string()),
        None => None,
    }
}

pub fn get_track_number(tag: &Tag) -> Option<String> {
    match tag.get_string(&ItemKey::TrackNumber) {
        Some(track_number) => Some(track_number.to_string()),
        None => None,
    }
}

pub fn get_genre(tag: &Tag) -> Option<String> {
    match tag.get_string(&ItemKey::Genre) {
        Some(genre) => Some(genre.to_string()),
        None => None,
    }
}

pub fn get_total_dur_sec(tag: &Tag) -> u64 {
    match tag.get_string(&ItemKey::Length) {
        Some(ms_str) => Duration::from_millis(ms_str.parse().unwrap_or_default()).as_secs(),
        None => 0,
    }
}
