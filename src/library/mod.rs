use lofty::{read_from_path, ItemKey, Probe, Tag};
use multimap::MultiMap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
// use std::fs::File;
use bincode;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
// use std::io::BufReader;
pub mod search;

const UNKNOWN_ARTIST: &str = "Unknown Artist";
const UNKNOWN_ALBUM: &str = "Unkwon Album";

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
    pub duration: String,
    pub play_count: u32,
    pub track_number: Option<String>,
    pub path: String,
}

pub struct Library {
    pub artist_map: HashMap<String, Artist>,
    pub album_map: MultiMap<String, Album>,
    pub song_map: MultiMap<String, Song>,
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
        // write!(f, "{{ file: {}, line: {} }}", file!(), line!())
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

impl Library {
    pub fn new() -> Library {
        Library {
            artist_map: HashMap::new(),
            album_map: MultiMap::new(),
            song_map: MultiMap::new(),
        }
    }

    // only support wav, mp3, flac
    pub fn import_file(&mut self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let path;
        if Path::new(filepath).exists() {
            path = fs::canonicalize(filepath);
            // match fs::canonicalize(filepath) {
            //     Ok(tmp) => {
            //         path = Some(tmp.into_os_string().to_str().unwrap().as_bytes());
            //     },
            //     _ => {
            //         path = None;
            //     }
            // }
        } else {
            return Err(Box::new(ImportError::FileNotFound));
        }

        match read_from_path(filepath, false) {
            Ok(file) => {
                let duration = file.properties().duration();
                match file.primary_tag() {
                    Some(tag) => {
                        let title: &str;
                        match tag.get_string(&ItemKey::TrackTitle) {
                            Some(val) => title = val,
                            _ => return Err(Box::new(ImportError::MissingData)),
                        };
                        let track_artist: &str;
                        match tag.get_string(&ItemKey::TrackArtist) {
                            Some(val) => track_artist = val,
                            _ => track_artist = UNKNOWN_ARTIST,
                        };
                        let album_title: &str;
                        match tag.get_string(&ItemKey::AlbumTitle) {
                            Some(val) => album_title = val,
                            _ => album_title = UNKNOWN_ALBUM,
                        };
                        let album_artist: &str;
                        match tag.get_string(&ItemKey::AlbumArtist) {
                            Some(val) => album_artist = val,
                            _ => album_artist = UNKNOWN_ARTIST,
                        };
                        let year: Option<String>;
                        match tag.get_string(&ItemKey::Year) {
                            Some(val) => year = Some(val.to_string()),
                            _ => year = None,
                        };
                        let lyrics: Option<String>;
                        match tag.get_string(&ItemKey::Lyrics) {
                            Some(val) => lyrics = Some(val.to_string()),
                            _ => lyrics = None,
                        };
                        let track_number: Option<String>;
                        match tag.get_string(&ItemKey::TrackNumber) {
                            Some(val) => track_number = Some(val.to_string()),
                            _ => track_number = None,
                        };
                        let genre: Option<String>;
                        match tag.get_string(&ItemKey::Genre) {
                            Some(val) => genre = Some(val.to_string()),
                            _ => genre = None,
                        };

                        // let album_artist_arc;

                        // match self.artist_map.get(track_artist) {
                        //     Some(artist_arc) => {
                        //         album_artist_arc = artist_arc.clone();
                        //     }
                        //     None => {
                        //         let new_artist = Artist {
                        //             name: album_artist.to_string(),
                        //             album_titles: vec![album_title.to_string()],
                        //             play_count: 0,
                        //         };
                        //         album_artist_arc = Arc::new(Mutex::new(new_artist));
                        //     }
                        // }

                        // let new_album = Album {
                        //     title: album_title.to_string(),
                        //     song_titles: vec![title.to_owned()],
                        //     artist_name: album_artist.to_owned(),
                        //     year,
                        //     genre,
                        //     play_count: 0,
                        // };
                        // let new_album_arc = Arc::new(Mutex::new(new_album));
                        match path {
                            Ok(tmp) => {
                                let duration_str = format!(
                                    "{}:{}",
                                    duration.as_secs() / 60,
                                    duration.as_secs() % 60
                                );
                                let new_song = Song {
                                    title: title.to_string(),
                                    album_title: album_title.to_string(),
                                    track_artist: track_artist.to_string(),
                                    album_artist: album_artist.to_string(),
                                    genre,
                                    year,
                                    duration: duration_str,
                                    play_count: 0,
                                    track_number,
                                    path: tmp.to_str().unwrap().to_string(),
                                };
                                self.song_map.insert(title.to_string(), new_song);

                                match self.album_map.get_vec_mut(album_title) {
                                    Some(albums) => {
                                        for album in albums {
                                            if album.artist_name == album_artist {
                                                album.song_titles.push(title.to_string());
                                            }
                                        }
                                    }
                                    None => {
                                        let new_album = Album {
                                            title: album_title.to_string(),
                                            song_titles: vec![title.to_owned()],
                                            artist_name: album_artist.to_owned(),
                                            play_count: 0,
                                        };
                                        self.album_map.insert(album_title.to_string(), new_album);
                                    }
                                }
                                match self.artist_map.get_mut(album_artist) {
                                    Some(artist) => {
                                        artist.album_titles.push(album_title.to_string());
                                    }
                                    None => {
                                        let new_artist = Artist {
                                            name: album_artist.to_string(),
                                            album_titles: vec![album_title.to_string()],
                                            play_count: 0,
                                        };
                                        self.artist_map
                                            .insert(album_artist.to_string(), new_artist);
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(Box::new(e));
                            }
                        }
                    }
                    _ => {
                        return Err(Box::new(ImportError::Parsing));
                    }
                }
            }
            Err(_) => {
                return Err(Box::new(ImportError::Parsing));
            }
        }

        // self.save_to_db()?;

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
            self.song_map.len(),
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
        for (_, song) in self.song_map.iter() {
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
        for (_, song) in self.song_map.iter() {
            bincode::serialize_into(&db_file, song);
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
                    // println!("{}", song.title);
                    self.song_map.insert(song.title.to_string(), song);
                }
                Err(e) => {
                    error!("{:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn read_from_csv(&mut self) -> Result<(), Box<dyn Error>> {
        let db_file = fs::File::open("db.csv")?;
        let mut reader = csv::Reader::from_reader(db_file);
        for result in reader.deserialize() {
            let song: Song = result?;
            match self.album_map.get_vec_mut(song.album_title.as_str()) {
                Some(albums) => {
                    for album in albums {
                        if album.artist_name == song.album_artist {
                            album.play_count += song.play_count;
                            album.song_titles.push(song.title.to_string());
                        }
                    }
                }
                None => {
                    let album = Album {
                        title: song.album_title.to_string(),
                        artist_name: song.album_artist.to_string(),
                        play_count: song.play_count,
                        song_titles: vec![song.title.to_string()],
                    };
                    self.album_map.insert(album.title.to_string(), album);
                }
            }
            match self.artist_map.get_mut(song.album_artist.as_str()) {
                Some(artist) => {
                    artist.play_count += song.play_count;
                    artist.album_titles.push(song.album_title.to_string());
                }
                None => {
                    let artist = Artist {
                        name: song.album_artist.to_string(),
                        album_titles: vec![song.album_title.to_string()],
                        play_count: song.play_count,
                    };
                    self.artist_map.insert(artist.name.to_string(), artist);
                }
            }
            self.song_map.insert(song.title.to_owned(), song);
        }
        Ok(())
    }
}
