// use crate::library::Library;
// use crate::library::Song;
// use crate::player::{Player, PlayerRequests};
// use crate::queue::SongQueue;
// use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle, Sink};
// use std::fs::File;
// use std::io::BufReader;
// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};
// use std::thread;

// pub struct RodioPlayer {
//     curr_song: Option<Song>,
//     curr_sink: Option<Sink>,
//     queue: SongQueue,
//     curr_volume: f32,
//     curr_state: PlayerStates,
//     curr_buf: Option<BufReader<File>>,
// }

// enum PlayerStates {
//     STOPPED,
//     STARTED,
//     PAUSED,
// }

// impl Player for RodioPlayer {
//     fn init() -> RodioPlayer {
//         let (sink, _) = Sink::new_idle();
//         return RodioPlayer {
//             curr_song: None,
//             queue: SongQueue::new(),
//             curr_volume: 0.,
//             curr_state: PlayerStates::STOPPED,
//             curr_sink: Some(sink),
//             curr_buf: None,
//         };
//     }
//     fn start(self, song: Song) {
//         unimplemented!();
//     }
//     fn resume() {
//         unimplemented!();
//     }
//     fn stop() {
//         unimplemented!();
//     }
//     fn next() {
//         unimplemented!();
//     }
//     fn prev() {
//         unimplemented!();
//     }
//     fn seek(seconds: u64) {
//         unimplemented!();
//     }
//     fn set_volume(level: f32) {
//         unimplemented!();
//     }

//     // create a player thread with a loop that receives requuests for player functions
//     fn listen(mut self, rx: Receiver<PlayerRequests>) {
//         let _player_process = thread::spawn(move || loop {
//             let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//             let sink = Sink::try_new(&stream_handle).unwrap();
//             self.curr_sink = Some(sink);
//             info!("Started player thread...");
//             loop {
//                 match rx.recv() {
//                     Ok(request) => {
//                         match request {
//                             PlayerRequests::START(song) => {
//                                 self.curr_song = Some(song.clone());
//                                 info!("Playing {}", song.title);
//                                 let file = BufReader::new(File::open(song.path).unwrap());
//                                 let source = Decoder::new(file).unwrap();
//                                 match &self.curr_sink {
//                                     Some(sink) => sink.append(source),
//                                     None => error!("No sink found"),
//                                 }
//                             }
//                             PlayerRequests::RESUME => match &self.curr_sink {
//                                 Some(sink) => sink.play(),
//                                 None => error!("No sink found"),
//                             },
//                             PlayerRequests::PAUSE => {
//                                 info!("Request to pause");
//                                 match &self.curr_sink {
//                                     Some(sink) => sink.pause(),
//                                     None => error!("No sink found"),
//                                 }
//                             }
//                             PlayerRequests::STOP => match &self.curr_sink {
//                                 Some(sink) => {
//                                     sink.stop();
//                                     self.curr_sink = None;
//                                 }
//                                 None => error!("No sink found"),
//                             },
//                             PlayerRequests::SEEK(seconds) => {
//                                 let sink: &Sink;
//                                 match &self.curr_sink {
//                                     Some(s) => {
//                                         sink = s;
//                                     }
//                                     None => {
//                                         let (_stream, stream_handle) =
//                                             OutputStream::try_default().unwrap();
//                                         let s = Sink::try_new(&stream_handle).unwrap();
//                                         // self.curr_sink = Some(s);
//                                     }
//                                 }
//                                 match &self.curr_song {
//                                     Some(song) => {
//                                         let file = BufReader::new(
//                                             File::open(song.path.to_owned()).unwrap(),
//                                         );
//                                         let source = Decoder::new(file).unwrap();
//                                         // .skip_duration(std::time::Duration::from_secs(seconds));
//                                         // sink.append(source);
//                                         info!("seeking");
//                                         // info!("{}", sink.empty());
//                                         // sink.play();
//                                         // sink.sleep_until_end();
//                                     }
//                                     None => {
//                                         error!("No song found")
//                                     }
//                                 }
//                             }
//                             PlayerRequests::CHANGE_VOLUME(vol_diff) => match &self.curr_sink {
//                                 Some(sink) => {
//                                     self.curr_volume = sink.volume();
//                                     sink.set_volume(self.curr_volume + vol_diff);
//                                     self.curr_volume = sink.volume();
//                                 }
//                                 None => error!("No sink found"),
//                             },
//                             _ => {}
//                         }
//                     }
//                     Err(e) => {
//                         error!("{:?}", e);
//                     }
//                 }
//             }
//         });
//     }
// }
