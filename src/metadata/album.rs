// use std::sync::{Arc, Mutex};
// use crate::metadata::artist::Artist;

// pub struct Album {
//     title: String,
//     songs: Vec<String>,
//     artist_name: String,
//     artist: Arc<Mutex<Artist>>,
//     year: i32,
//     genre: String,
//     play_count: u32,
// }

// impl Album {
//     pub fn new(
//         title: &String,
//         songs: Vec<String>,
//         artist_name: &String,
//         artist: Arc<Mutex<Artist>>,
//         year: i32,
//         genre: &String,
//     ) -> Album {
//         let mut play_count: u32 = 0;
//         Album {
//             title: title.to_owned(),
//             songs,
//             artist_name: artist_name.to_owned(),
//             artist: artist.clone(),
//             year,
//             genre: genre.to_owned(),
//             play_count,
//         }
//     }

//     pub fn get_artist_name(&self) -> &String {
//         return &self.artist_name;
//     }

//     pub fn get_play_count(&self) -> u32 {
//         return self.play_count;
//     }

// }

// // impl Clone for Album {
// //     fn clone(&self) -> Album {
// //         Album {
// //             title: self.title.to_string(),
// //             songs: self.songs.to_vec(),
// //             artist: self.artist,
// //             year: self.year,
// //             play_count: self.play_count,
// //             genre: self.genre.to_string(),
// //         }
// //     }
// // }
