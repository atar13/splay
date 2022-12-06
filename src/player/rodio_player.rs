use crate::player::{Player, PlayerRequests};
use crate::state::AppState;
use crate::utils::constants::PlayerStates;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct RodioPlayer {
}

impl RodioPlayer {
    pub fn new() -> RodioPlayer {
        RodioPlayer { }
    }
}

impl Player for RodioPlayer {
    // create a player thread with a loop that receives requests for player functions
    fn listen(&mut self, app_state: Arc<Mutex<AppState>>, rx: Receiver<PlayerRequests>) {
        let mut join_handle: Option<JoinHandle<()>> = None;

        loop {
            match rx.recv() {
                Ok(request) => {
                    match request {
                        PlayerRequests::Quit => return,
                        PlayerRequests::Resume => {
                            app_state.lock().unwrap().player.curr_state = PlayerStates::PLAYING
                        }
                        PlayerRequests::Pause => {
                            app_state.lock().unwrap().player.curr_state = PlayerStates::PAUSED
                        }
                        PlayerRequests::Stop => {
                            app_state.lock().unwrap().player.curr_state = PlayerStates::STOPPED;
                            join_handle.take().map(JoinHandle::join);
                            app_state.lock().unwrap().player.curr_song = None;
                        }
                        PlayerRequests::Start => {
                            // stop player if previously playing
                            app_state.lock().unwrap().player.curr_state = PlayerStates::STOPPED;
                            join_handle.take().map(JoinHandle::join);
                            app_state.lock().unwrap().player.curr_state = PlayerStates::PLAYING;

                            // fetch which song is selected in the UI
                            // TODO: maybe just have other threads modify player.curr_song instead
                            let song = match app_state.lock().unwrap().ui.selected_song.to_owned() {
                                Some(song) => song,
                                None => continue,
                            };

                            app_state.lock().unwrap().player.curr_song = Some(song.to_owned());

                            let cloned_state = app_state.clone();
                            join_handle =
                                Some(thread::spawn(move || player(song.path, cloned_state)));
                        }
                        PlayerRequests::PlayPause => {
                            match app_state.lock().unwrap().player.curr_state {
                                PlayerStates::PLAYING => {
                                    app_state.lock().unwrap().player.curr_state =
                                        PlayerStates::PAUSED
                                }
                                PlayerStates::PAUSED => {
                                    app_state.lock().unwrap().player.curr_state =
                                        PlayerStates::PLAYING
                                }
                                _ => (),
                            }
                        }
                        // PlayerRequests::SEEK(seconds) => {
                        //     let sink: &Sink;
                        //     match &self.curr_sink {
                        //         Some(s) => {
                        //             sink = s;
                        //         }
                        //         None => {
                        //             let (_stream, stream_handle) =
                        //                 OutputStream::try_default().unwrap();
                        //             let s = Sink::try_new(&stream_handle).unwrap();
                        //             // self.curr_sink = Some(s);
                        //         }
                        //     }
                        //     match &self.curr_song {
                        //         Some(song) => {
                        //             let file = BufReader::new(
                        //                 File::open(song.path.to_owned()).unwrap(),
                        //             );
                        //             let source = Decoder::new(file).unwrap();
                        //             // .skip_duration(std::time::Duration::from_secs(seconds));
                        //             // sink.append(source);
                        //             info!("seeking");
                        //             // info!("{}", sink.empty());
                        //             // sink.play();
                        //             // sink.sleep_until_end();
                        //         }
                        //         None => {
                        //             error!("No song found")
                        //         }
                        //     }
                        // }
                        // PlayerRequests::CHANGE_VOLUME(vol_diff) => match &self.curr_sink {
                        //     Some(sink) => {
                        //         self.curr_volume = sink.volume();
                        //         sink.set_volume(self.curr_volume + vol_diff);
                        //         self.curr_volume = sink.volume();
                        //     }
                        //     None => error!("No sink found"),
                        // },
                        _ => (),
                    }
                }
                Err(e) => {
                    error!("{:?}", e);
                }
            }
        }
    }
}

fn player(path: String, app_state: Arc<Mutex<AppState>>) {
    let tick_rate = 250;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    sink.append(Decoder::new(BufReader::new(file)).unwrap());
    app_state.lock().unwrap().player.progress = Duration::ZERO;
    loop {
        match app_state.lock().unwrap().player.curr_state {
            PlayerStates::STOPPED => {
                sink.stop();
                break;
            }
            PlayerStates::PAUSED => {
                sink.pause();
                continue;
            }
            PlayerStates::PLAYING => {
                sink.play();
            }
        }
        if sink.empty() {
            break;
        }
        thread::sleep(Duration::from_millis(tick_rate));
        let mut guard = app_state.lock().unwrap(); //idk I just did this not to call lock() a bunch
                                                   //of times
                                                   // update player time with how long the last packet took to play
        guard.player.progress = guard.player.progress + Duration::from_millis(tick_rate);
        drop(guard);
    }
}
