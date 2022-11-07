use bincode;
use lofty::{read_from_path, ItemKey};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;
pub mod search;

const UNKNOWN_ARTIST: &str = "Unknown Artist";
const UNKNOWN_ALBUM: &str = "Unkwon Album";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
    pub album_titles: Vec<String>,
    pub play_count: u32,
}

pub struct Album {
    pub title: String,
    pub song_titles: Vec<String>,
    pub artist_name: String,
    pub play_count: u32,
}

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
    // artist: Box<Artist>
}

pub struct Library {
    pub songs: Vec<Song>,
}

pub enum ImportError {
    MissingData,
    FileNotFound,
    Parsing,
}

impl Error for ImportError {}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingData => {
                return write!(f, "Could not gather parts of metadata.");
            }
            Self::FileNotFound => {
                return write!(f, "file not found");
            }
            Self::Parsing => {
                return write!(f, "Parsing error");
            }
        }
    }
}

impl fmt::Debug for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingData => {
                return write!(f, "Could not gather parts of metadata.");
            }
            Self::FileNotFound => {
                return write!(f, "file not found");
            }
            Self::Parsing => {
                return write!(f, "Parsing error");
            }
        }
    }
}

impl Default for Library {
    fn default() -> Self {
        Library { songs: vec![] }
    }
}

impl Library {
    pub fn new() -> Library {
        Library { songs: Vec::new() }
    }

    // only support wav, mp3, flac
    pub fn import_file(&mut self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let path = if Path::new(filepath).exists() {
            fs::canonicalize(filepath)
        } else {
            return Err(Box::new(ImportError::FileNotFound));
        };

        match read_from_path(filepath, false) {
            Ok(file) => match file.primary_tag() {
                Some(tag) => {
                    let title = match tag.get_string(&ItemKey::TrackTitle) {
                        Some(val) => val,
                        None => return Err(Box::new(ImportError::MissingData)),
                    };

                    let track_artist = match tag.get_string(&ItemKey::TrackArtist) {
                        Some(val) => val,
                        None => UNKNOWN_ARTIST,
                    };

                    let album_title = match tag.get_string(&ItemKey::AlbumTitle) {
                        Some(val) => val,
                        None => UNKNOWN_ALBUM,
                    };

                    let album_artist = match tag.get_string(&ItemKey::AlbumArtist) {
                        Some(val) => val,
                        None => UNKNOWN_ARTIST,
                    };

                    let year = match tag.get_string(&ItemKey::Year) {
                        Some(val) => Some(val.to_string()),
                        None => None,
                    };

                    let _lyrics = match tag.get_string(&ItemKey::Lyrics) {
                        Some(val) => Some(val.to_string()),
                        None => None,
                    };

                    let track_number = match tag.get_string(&ItemKey::TrackNumber) {
                        Some(val) => Some(val.to_string()),
                        None => None,
                    };

                    let genre = match tag.get_string(&ItemKey::Genre) {
                        Some(val) => Some(val.to_string()),
                        None => None,
                    };

                    let total_dur_sec = match tag.get_string(&ItemKey::Length) {
                        Some(ms_str) => Duration::from_millis(ms_str.parse().unwrap()).as_secs(),
                        None => 0,
                    };

                    match path {
                        Ok(tmp) => {
                            let new_song = Song {
                                title: title.to_string(),
                                album_title: album_title.to_string(),
                                track_artist: track_artist.to_string(),
                                album_artist: album_artist.to_string(),
                                genre,
                                year,
                                duration_secs: total_dur_sec,
                                play_count: 0,
                                track_number,
                                path: tmp.to_str().unwrap().to_string(),
                            };

                            self.songs.push(new_song);
                        }
                        Err(e) => {
                            return Err(Box::new(e));
                        }
                    }
                }
                _ => {
                    return Err(Box::new(ImportError::Parsing));
                }
            },
            Err(_) => {
                return Err(Box::new(ImportError::Parsing));
            }
        }

        Ok(())
    }

    pub fn import_dir(&mut self, dir_path: &str) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        match self._import_dir(dir_path) {
            Err(e) => return Err(e),
            _ => (),
        }
        let elapsed = now.elapsed();
        info!(
            "Took {:.3?} to import {} files from {}",
            elapsed,
            self.songs.len(),
            dir_path
        );
        Ok(())
    }

    fn _import_dir(&mut self, dir_path: &str) -> Result<(), Box<dyn Error>> {
        let entries = fs::read_dir(dir_path);
        for entry in entries {
            for tmp in entry {
                match tmp {
                    Ok(file) => {
                        if file.path().is_dir() {
                            match self._import_dir(file.path().to_str().unwrap()) {
                                Err(e) => error!("{:?}", e),
                                _ => (),
                            }
                        } else {
                            let _ = self.import_file(file.path().to_str().unwrap());
                        }
                    }
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                }
            }
        }
        return Ok(());
    }

    pub fn save_to_csv(&self) -> Result<(), Box<dyn Error>> {
        let db_file = fs::File::create("db.csv")?;
        let mut wtr = csv::Writer::from_writer(db_file);
        // wtr.write_record(&["Title", "AlbumTitle", "ArtistName", "Duration", "PlayCount", "TrackNumber", "Path"])?;
        // let mut sorted : Vec<_> = self.song_map.iter().collect();
        // sorted.sort_by_key(|a| a.0);
        for song in self.songs.iter() {
            match wtr.serialize(song) {
                Ok(_) => (),
                Err(e) => error!("{:?}", e),
            }
        }
        match wtr.flush() {
            Ok(_) => (),
            Err(e) => error!("{:?}", e),
        }
        Ok(())
    }

    pub fn save_to_bin(&self) -> Result<(), Box<dyn Error>> {
        let db_file = fs::File::create("db").unwrap();
        for song in self.songs.iter() {
            _ = bincode::serialize_into(&db_file, song);
        }
        Ok(())
    }

    pub fn read_from_bin(&mut self) -> Result<(), Box<dyn Error>> {
        let db_file;
        match std::fs::File::open("db") {
            Ok(f) => db_file = f,
            Err(e) => return Err(Box::new(e)),
        };
        let mut buf_reader = std::io::BufReader::new(db_file);
        loop {
            let result: Result<Song, Box<bincode::ErrorKind>> =
                bincode::deserialize_from(&mut buf_reader);
            match result {
                Ok(song) => {
                    self.songs.push(song);
                }
                Err(e) => {
                    error!("{:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    //     pub fn read_from_csv(&mut self) -> Result<(), Box<dyn Error>> {
    //         let db_file = fs::File::open("db.csv")?;
    //         let mut reader = csv::Reader::from_reader(db_file);
    //         for result in reader.deserialize() {
    //             let song: Song = result?;
    //             match self.album_map.get_vec_mut(song.album_title.as_str()) {
    //                 Some(albums) => {
    //                     for album in albums {
    //                         if album.artist_name == song.album_artist {
    //                             album.play_count += song.play_count;
    //                             album.song_titles.push(song.title.to_string());
    //                         }
    //                     }
    //                 }
    //                 None => {
    //                     let album = Album {
    //                         title: song.album_title.to_string(),
    //                         artist_name: song.album_artist.to_string(),
    //                         play_count: song.play_count,
    //                         song_titles: vec![song.title.to_string()],
    //                     };
    //                     self.album_map.insert(album.title.to_string(), album);
    //                 }
    //             }
    //             match self.artist_map.get_mut(song.album_artist.as_str()) {
    //                 Some(artist) => {
    //                     artist.play_count += song.play_count;
    //                     artist.album_titles.push(song.album_title.to_string());
    //                 }
    //                 None => {
    //                     let artist = Artist {
    //                         name: song.album_artist.to_string(),
    //                         album_titles: vec![song.album_title.to_string()],
    //                         play_count: song.play_count,
    //                     };
    //                     self.artist_map.insert(artist.name.to_string(), artist);
    //                 }
    //             }
    //             self.song_map.insert(song.title.to_owned(), song);
    //         }
    //         Ok(())
    //     }
}
