use lofty::{read_from_path, ItemKey, Probe, Tag};
use multimap::MultiMap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

const UNKNOWN_ARTIST: &str = "Unknown Artist";
const UNKNOWN_ALBUM: &str = "Unkwon Album";

pub struct Artist {
    pub name: String,
    pub album_titles: Vec<String>,
    pub play_count: u32,
}

pub struct Album {
    pub title: String,
    // pub songs: Option<Vec<Arc<Mutex<Song>>>>,
    // pub artist: Option<Arc<Mutex<Artist>>>,
    pub song_titles: Vec<String>,
    pub artist_name: String,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub play_count: u32,
}

pub struct Song {
    pub title: String,
    // pub album: Option<Arc<Mutex<Album>>>,
    // pub artist: Option<Arc<Mutex<Artist>>>,
    pub album_title: String,
    pub artist_name: String,
    pub duration: std::time::Duration,
    pub play_count: u32,
    pub track_number: Option<String>,
    pub path: Option<String>,
    pub lyrics: Option<String>,
}

pub struct Library {
    pub artistMap: HashMap<String, Arc<Mutex<Artist>>>,
    pub albumMap: MultiMap<String, Arc<Mutex<Album>>>,
    pub songMap: MultiMap<String, Arc<Mutex<Song>>>, // make these multimaps
                                                     // pub albumMap: HashMap<String, Arc<Mutex<Album>>>,
                                                     // pub songMap: HashMap<String, Arc<Mutex<Song>>>
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
            artistMap: HashMap::new(),
            albumMap: MultiMap::new(),
            songMap: MultiMap::new(),
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

                        // let new_artist = Artist {
                        //     name: track_artist.to_owned().to_string(),
                        //     album_titles: vec![album_title.to_string()],
                        //     play_count: 0,
                        // };

                        // let track_artist_arc = Arc::new(Mutex::new(new_artist));
                        let album_artist_arc;

                        // if track_artist != album_artist {
                        match self.artistMap.get(track_artist) {
                            Some(artist_arc) => {
                                album_artist_arc = artist_arc.clone();
                            }
                            None => {
                                let new_artist = Artist {
                                    name: album_artist.to_string(),
                                    album_titles: vec![album_title.to_string()],
                                    play_count: 0,
                                };
                                album_artist_arc = Arc::new(Mutex::new(new_artist));
                            }
                        }
                        // } else {
                        //     album_artist_arc = track_artist_arc.clone();
                        // }

                        let new_album = Album {
                            title: album_title.to_string(),
                            song_titles: vec![title.to_owned()],
                            artist_name: album_artist.to_owned(),
                            year,
                            genre,
                            play_count: 0,
                        };
                        let new_album_arc = Arc::new(Mutex::new(new_album));
                        match path {
                            Ok(tmp) => {
                                let new_song = Song {
                                    title: title.to_string(),
                                    // album: Some(new_album_arc.clone()),
                                    // artist: Some(track_artist_arc.clone()),
                                    album_title: album_title.to_string(),
                                    artist_name: album_artist.to_string(),
                                    duration,
                                    play_count: 0,
                                    track_number,
                                    path: Some(tmp.to_str().unwrap().to_string()),
                                    lyrics,
                                };
                                let new_song_arc = Arc::new(Mutex::new(new_song));
                                // new_album.addSong(new_song_arc);
                                self.artistMap
                                    .insert(album_artist.to_string(), album_artist_arc);
                                // self.artistMap.insert(track_artist.to_string(), track_artist_arc);
                                self.albumMap.insert(album_title.to_string(), new_album_arc);
                                self.songMap.insert(title.to_string(), new_song_arc);
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

        // // let fetched_artist =
        // self.artistMap.get(&artist_name)
        //     .map(|a| artist = Box::new(a))
        //     .map(|_| {
        //         let new_artist = Artist::new(&artist_name, Vec::new());
        //         self.artistMap.insert(artist_name, new_artist);
        //         artist = Box::new(&new_artist);
        //     });
        // make new song

        //commented after adding lofty code

        //         match self.artistMap.get(&artist_name) {
        //             Some(artist) => {
        //                 match self.albumMap.get(&album_title) {
        //                     Some(album) => {
        //                         artist.lock().unwrap().albums.push(album.clone());
        //                         let new_song = Song {
        //                             title: title.to_owned(),
        //                             album: Some(album.clone()),
        //                             artist: Some(artist.clone()),
        //                             play_count: 0,
        //                             track_number: track_num,
        //                             duration: 0,
        //                             path: path
        //                         };
        //                         let new_song_arc = Arc::new(Mutex::new(new_song));
        //                         album.lock().unwrap().songs.push(new_song_arc.clone());
        //                         self.songMap.insert(title.to_owned(), new_song_arc);
        //                     },
        //                     _ => {
        //                         let new_song = Song {
        //                             title: title.to_owned(),
        //                             album: None,
        //                             artist: Some(artist.clone()),
        //                             play_count: 0,
        //                             track_number: track_num,
        //                             duration: 0,
        //                             path: path
        //                         };
        //                         let new_song_arc = Arc::new(Mutex::new(new_song));
        //                         let new_album: Album = Album {
        //                             title: album_title.to_string(),
        //                             songs: vec![new_song_arc.clone()],
        //                             artist: Some(artist.clone()),
        //                             year,
        //                             genre: String::from(""),
        //                             play_count: 0,
        //                         };
        //                         let new_album_arc = Arc::new(Mutex::new(new_album));
        //                         new_song_arc.lock().unwrap().album = Some(new_album_arc.clone());
        //                         self.albumMap.insert(album_title.to_string(), new_album_arc);
        //                         self.songMap.insert(title.to_owned(), new_song_arc);
        //                     }
        //                 }
        //             }
        //             None => {
        //                 let new_song = Song {
        //                     title: title.to_owned(),
        //                     album: None,
        //                     artist: None,
        //                     play_count: 0,
        //                     track_number: track_num,
        //                     duration: 0,
        //                     path: path
        //                 };
        //                 let new_song_arc = Arc::new(Mutex::new(new_song));
        //                 let new_album: Album = Album {
        //                     title: album_title.to_string(),
        //                     songs: vec![new_song_arc.clone()],
        //                     artist: None,
        //                     year,
        //                     genre: String::from(""),
        //                     play_count: 0,
        //                 };
        //                 let new_album_arc = Arc::new(Mutex::new(new_album));
        //                 let new_artist = Artist {
        //                     name: artist_name.to_owned(),
        //                     albums: Vec::new(),
        //                     play_count: 0,
        //                 };
        //                 let new_artist_arc = Arc::new(Mutex::new(new_artist));

        //                 new_song_arc.lock().unwrap().album = Some(new_album_arc.clone());
        //                 new_song_arc.lock().unwrap().artist = Some(new_artist_arc.clone());

        //                 new_album_arc.lock().unwrap().artist = Some(new_artist_arc.clone());

        //                 self.artistMap.insert(artist_name.to_owned(), new_artist_arc);
        //                 self.albumMap.insert(album_title.to_owned(), new_album_arc);
        //                 self.songMap.insert(title.to_owned(), new_song_arc);
        //             }
        //         }

        Ok(())
    }

    pub fn import_dir(&mut self, dir_path: &str) -> Result<(), Box<dyn Error>> {
        let entries = fs::read_dir(dir_path);
        for entry in entries {
            for tmp in entry {
                match tmp {
                    Ok(file) => {
                        let _ = self.import_file(file.path().to_str().unwrap());
                    }
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                }
            }
        }
        return Ok(());
    }

    pub fn save_to_file() {}
}
