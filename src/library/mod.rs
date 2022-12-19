pub mod errors;
pub mod search;
pub mod song;
pub mod tag;

use crate::library::song::Song;
use bincode;
use errors::ImportError;
use lofty::read_from_path;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

pub struct Library {
    pub songs: Vec<Song>,
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

    // only supports wav, mp3, flac
    pub fn import_file(&mut self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let path = if Path::new(filepath).exists() {
            match fs::canonicalize(filepath) {
                Ok(path) => match path.to_str() {
                    Some(p) => p.to_string(),
                    None => return Err(Box::new(ImportError::Parsing)), // only if path contains
                                                                        // invalid unicode
                },
                Err(e) => return Err(Box::new(e)),
            }
        } else {
            return Err(Box::new(ImportError::FileNotFound));
        };

        match read_from_path(filepath, false) {
            Ok(file) => match file.primary_tag() {
                Some(tag) => {
                    let song = match Song::from_tag(tag, path) {
                        Ok(song) => song,
                        Err(err) => return Err(err),
                    };
                    self.songs.push(song);
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

    // recursive helper function for import_dir
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
        Ok(())
    }

    pub fn save_to_file(&self, path: String) -> Result<(), Box<dyn Error>> {
        let db_file = match fs::File::create(path) {
            Ok(f) => f,
            Err(err) => return Err(Box::new(err)), //TODO: replace this with library DB error type
        };
        for song in self.songs.iter() {
            match bincode::serialize_into(&db_file, song) {
                Err(err) => return Err(Box::new(err)), //TODO: replace this with library DB error type
                _ => (),
            }
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        let db_file = match std::fs::File::open("db") {
            Ok(f) => f,
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
}
