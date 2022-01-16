mod library;
mod metadata;
mod queue;
mod utils;
use crate::library::Library;
use lofty::{read_from_path, ItemKey, Probe, Tag};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lib = Library::new();
    let filename = &args[1];
    // let tagged_file = read_from_path(filename, false).unwrap();
    // let id3v2 = tagged_file.primary_tag().unwrap();
    // println!("{}", id3v2.get_string(&ItemKey::TrackTitle).unwrap());
    // let result = lib.import_file(filename);
    let result = lib.import_dir(filename);
    match result {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };

    for artist in lib.artistMap.iter() {
        match lib.artistMap.get(artist.0) {
            Some(tmp) => {
                for album in &tmp.lock().unwrap().album_titles {
                    match lib.albumMap.get_vec(album) {
                        Some(a) => {
                            for al in a {
                                println!("{:?}", al.lock().unwrap().song_titles);
                            }
                        }
                        None => {
                            println!("No album of name {}", album);
                        }
                    }
                }
            }
            None => {}
        }
    }
}
