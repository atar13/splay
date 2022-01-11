mod library;
mod metadata;
mod queue;
mod utils;
use crate::library::Library;
use std::env;
use lofty::{read_from_path, Probe, Tag, ItemKey};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lib = Library::new();
    let filename = &args[1];
    let tagged_file = read_from_path(filename, false).unwrap();
    let id3v2 = tagged_file.primary_tag().unwrap();
    // println!("{}", id3v2.get_string(&ItemKey::TrackTitle).unwrap());
    lib.import_file(filename);

    // for artist in lib.artistMap.iter() {
    //     println!("{}", artist.1.lock().unwrap().name);
    // }
}
