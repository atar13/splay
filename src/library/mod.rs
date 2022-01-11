use lofty::{read_from_path, Probe, Tag, ItemKey};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

const UNKNOWN_ARTIST: &str = "Unknown Artist";
const UNKNOWN_ALBUM: &str = "Unkwon Album";

pub struct Artist {
    pub name: String,
    pub albums: Vec<Arc<Mutex<Album>>>,
    pub play_count: u32,
}

pub struct Album {
    pub title: String,
    pub songs: Vec<Arc<Mutex<Song>>>,
    pub artist: Option<Arc<Mutex<Artist>>>,
    pub year: i32,
    pub genre: String,
    pub play_count: u32,
}

pub struct Song {
    pub title: String,
    pub album: Option<Arc<Mutex<Album>>>,
    pub artist: Option<Arc<Mutex<Artist>>>,
    pub duration: u32,
    pub play_count: u32,
    pub track_number: u16,
    pub path: Option<String>,
}

pub struct Library {
    pub artistMap: HashMap<String, Arc<Mutex<Artist>>>,
    pub albumMap: HashMap<String, Arc<Mutex<Album>>>,
    pub songMap: HashMap<String, Arc<Mutex<Song>>>,
}

pub enum ImportError {
    MissingData,
    FileNotFound,
    Parsing,
}

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
            albumMap: HashMap::new(),
            songMap: HashMap::new(),
        }
    }

    // only support wav, mp3, flac
    pub fn import_file(&mut self, filepath: &str) -> Result<(), ImportError> {
        let title: String;
        let artist_name: String;
        let year: i32;
        let album_title: String;
        let album_artist: String;
        let track_num: u16 = 0;
        let _total_track_num: u16;
        let path: Option<String>;

        if Path::new(filepath).exists() {
            match fs::canonicalize(filepath) {
                Ok(abs_path) => {
                    match abs_path.to_str() {
                        Some(tmp) => {
                            path = Some(tmp.to_string());
                        }
                        None => {
                            path = None;
                        }
                    };
                }
                Err(error) => {
                    println!("{}", error);
                    path = None;
                }
            };
        } else {
            return Err(ImportError::FileNotFound);
        }

        match read_from_path(filepath, false) {
            Ok(file) => {
                match file.primary_tag() {
                    Some(tag) =>  {
                        title = tag.get_string(&ItemKey::TrackTitle).unwrap().to_string();
                        println!("{}", tag.get_string(&ItemKey::Lyrics).unwrap())
                    },
                    _ =>  {
                        return Err(ImportError::Parsing);
                    }
                }
            },
            Err(e) => {
                return Err(ImportError::Parsing);
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

    pub fn import_dir(&mut self, dir_path: &str) {
        let entries = fs::read_dir(dir_path);
        for entry in entries {
            for tmp in entry {
                match tmp {
                    Ok(file) => {
                        self.import_file(file.path().to_str().unwrap());
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
    }

    pub fn save_to_file() {}
}
