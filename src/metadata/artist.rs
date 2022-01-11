// use std::sync::{Arc, Mutex};
// // use crate::metadata::album::Album;

// pub struct Artist {
//     name: String,
//     albums_titles: Vec<String>,
//     albums: Vec<Arc<Mutex<Album>>>,
//     play_count: u32,
// }

// impl Artist {
//     pub fn new(name: &String, albums_titles: Vec<String>, albums: Vec<Arc<Mutex<Album>>>) -> Artist {
//         let mut play_count: u32 = 0;
//         // let mut albums_copy: Vec<String> = Vec::new();
//         for album in albums {
//             let album = album.lock().unwrap();
//             play_count += album.get_play_count();
//         }
//         return Artist {
//             name: name.to_owned(),
//             albums_titles,
//             albums,
//             play_count,
//         };
//     }

//     pub fn get_name(&self) -> &String {
//         return &self.name;
//     }

//     pub fn get_album_titles(&self) -> &Vec<String> {
//         return &self.albums_titles;
//     }

//     pub fn add_album_title(&mut self, album_title: &String) {
//         self.albums_titles.push(album_title.to_owned());
//     }
// }

// // impl Copy for Artist {
// // }

// // impl Clone for Artist {
// //     fn clone(&self) -> Artist {
// //         Artist {
// //             name: self.name.to_string(),
// //             albums: self.albums,
// //             play_count: self.play_count,
// //         }
// //     }
// // }
