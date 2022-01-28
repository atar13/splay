use crate::library::Library;
use crate::library::Song;
use crate::queue::SongQueue;
use rodio::{source::Source, Decoder, OutputStream, Sink, OutputStreamHandle};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;


pub struct Player {
    curr_song: Option<Song>,
    curr_sink: Option<Sink>,
    queue: SongQueue,
    curr_volume: f32,
    curr_state: PlayerStates,
    curr_buf: Option<BufReader<File>>
}

enum PlayerStates {
    STOPPED,
    STARTED,
    PAUSED
}

pub enum PlayerRequests {
    STOP,
    START(Song),
    RESUME,
    PAUSE,
    NEXT,
    PREV,
    SEEK(u64),
    CHANGE_VOLUME(f32)
}

impl Player {
    pub fn init() -> Player {
        Player {
            curr_song: None,
            queue: SongQueue::new(),
            curr_volume: 0.,
            curr_state: PlayerStates::STOPPED,
            curr_sink: None,
            curr_buf: None
        }
    }

    // create a player thread with a loop that receives requuests for player functions
    pub fn start(mut self, rx: Receiver<PlayerRequests>) {
        let player_process = thread::spawn(move || loop { 
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            self.curr_sink = Some(sink);
            info!("Started player thread...");
            loop {
                match rx.recv() {
                    Ok(request) => {
                        match request {
                            PlayerRequests::START(song) => {
                                self.curr_song = Some(song.clone());
                                info!("Playing {}", song.title);
                                let file = BufReader::new(File::open(song.path).unwrap());
                                let source = Decoder::new(file).unwrap();
                                match &self.curr_sink {
                                    Some(sink) => sink.append(source),
                                    None => error!("No sink found")
                                }
                            }
                            PlayerRequests::RESUME => {
                                match &self.curr_sink {
                                    Some(sink) => sink.play(),
                                    None => error!("No sink found")
                                }
                            },
                            PlayerRequests::PAUSE => {
                                info!("Request to pause");
                                match &self.curr_sink {
                                    Some(sink) => sink.pause(),
                                    None => error!("No sink found")
                                }
                            }, 
                            PlayerRequests::STOP => {
                                match &self.curr_sink {
                                    Some(sink) => sink.stop(),
                                    None => error!("No sink found")
                                }
                            },
                            PlayerRequests::SEEK(seconds) => {
                                match &mut self.curr_sink {
                                    Some(sink) => {
                                        match &self.curr_song {
                                            Some(song) => {
                                                sink.stop();
                                                let file = BufReader::new(File::open(song.path.to_owned()).unwrap());
                                                let source = Decoder::new(file).unwrap().skip_duration(std::time::Duration::from_secs(seconds));
                                                sink.append(source);
                                                info!("seeking");
                                                sink.sleep_until_end();
                                            },
                                            None => {
                                            }
                                        }
                                    },
                                    None => {}
                                }

                            },
                            PlayerRequests::CHANGE_VOLUME(vol_diff) => {
                                match &self.curr_sink {
                                    Some(sink) => {
                                        self.curr_volume = sink.volume();
                                        sink.set_volume(self.curr_volume + vol_diff);
                                        self.curr_volume = sink.volume();
                                    },
                                    None => error!("No sink found")
                                }
                            },
                            _ => {

                            }
                        }
                    },
                    Err(e) => {
                        error!("{:?}", e);
                    }
                }
            }
        });
    }

    pub fn stop(&self, song: Song) {

    }

    pub fn resume(&self) {}

    pub fn pause(&self) {}
}
