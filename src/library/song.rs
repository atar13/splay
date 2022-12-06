use lofty::Tag;
use serde::{Deserialize, Serialize};

use super::{errors::ImportError, tag};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Song {
    pub title: String,
    pub album_title: String,
    pub track_artist: String,
    pub album_artist: String,
    pub genre: Option<String>,
    pub year: Option<String>,
    pub duration_secs: u64,
    pub play_count: u32,
    pub track_number: Option<String>,
    pub path: String,
}

impl Song {
    pub fn new(title: String, path: String) -> Self {
        Song {
            title,
            album_title: tag::UNKNOWN_ALBUM.to_string(),
            track_artist: tag::UNKNOWN_ARTIST.to_string(),
            album_artist: tag::UNKNOWN_ARTIST.to_string(),
            genre: None,
            year: None,
            duration_secs: 0,
            play_count: 0,
            track_number: None,
            path,
        }
    }

    pub fn from_tag(tag: &Tag, path: String) -> Result<Self, Box<ImportError>> {
        let title = match tag::get_title(tag) {
            Ok(title) => title,
            Err(err) => return Err(err),
        };
        let mut s = Self::new(title, path);
        s.track_artist = tag::get_track_artist(tag);
        s.album_title = tag::get_album_title(tag);
        s.album_artist = tag::get_album_artist(tag);
        s.year = tag::get_year(tag);
        s.track_number = tag::get_track_number(tag);
        s.genre = tag::get_genre(tag);
        s.duration_secs = tag::get_total_dur_sec(tag);
        Ok(s)
    }
}
