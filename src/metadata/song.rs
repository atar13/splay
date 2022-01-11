
// pub struct Song {
//     title: String,
//     artist_name: String,
//     album_title: String,
//     duration: u32,
//     play_count: u32,
//     track_number: u16,
//     path: Option<String>,
// }

// impl Song {
//     pub fn new(
//         title: &String,
//         artist_name: &String,
//         album_title: &String,
//         duration: u32,
//         track_number: u16,
//         path: Option<String>,
//     ) -> Song {
//         Song {
//             title: title.to_owned(),
//             artist_name: artist_name.to_owned(),
//             album_title: album_title.to_owned(),
//             duration,
//             play_count: 0,
//             track_number,
//             path,
//         }
//     }


//     pub fn get_title(&self) -> &String {
//         return &self.title;
//     }

//     pub fn get_path(&self) -> &Option<String> {
//         return &self.path;
//     }

//     pub fn get_fmt_duration(&self) -> (u32, u32) {
//         let min: u32 = self.duration / 60;
//         let sec: u32 = self.duration % 60;
//         return (min, sec);
//     }
// }

// // impl Clone for Song {
// //     fn clone(&self) -> Song {
// //         let new_artist = Box::new(Artist::new(&self.artist.get_name(), self.artist.get_albums_copy()));
// //         let new_album = Album::new(&self.album.get_title(), self.album.get_songs_copy(), new_artist, self.album.get_year(), &self.album.get_genre());
// //         return Song {
// //             title: self.title.to_string(),
// //             artist: Box::from(new_artist),
// //             album: self.album,
// //             duration: self.duration,
// //             play_count: self.play_count,
// //             track_number: self.track_number,
// //             path: self.path.to_string()
// //         };
// //     }
// // }
