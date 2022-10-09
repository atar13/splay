use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::player::PlayerRequests;
use crate::state::AppState;
use crate::utils::constants::PlayerStates;
use crate::utils::constants::Requests::AppRequests;
use std::fs::File;
use std::path::Path;
use std::sync::{Mutex, Arc, mpsc};
use std::sync::mpsc::{RecvError, TryRecvError, Sender};
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use super::output;

pub struct SymphoniaPlayer {
}

impl SymphoniaPlayer {
    pub fn init() -> SymphoniaPlayer {
        SymphoniaPlayer {
        }
    }

    pub fn listen(mut self, app_state: Arc<Mutex<AppState>>, rx: Receiver<PlayerRequests>, main_tx: Sender<AppRequests>) {

        let mut player_join_handle : Option<JoinHandle<()>> = None;


        // this loop listens for requests
        let _ = loop {
            match rx.recv() {
                Ok(request) => match request {
                    PlayerRequests::Stop => {
                        app_state.lock().unwrap().player.curr_state = PlayerStates::STOPPED;
                        player_join_handle.take().map(JoinHandle::join);
                    },
                    PlayerRequests::Pause => {
                        app_state.lock().unwrap().player.curr_state = PlayerStates::PAUSED;
                    },
                    PlayerRequests::Resume => {
                        app_state.lock().unwrap().player.curr_state = PlayerStates::PLAYING;
                    },
                    PlayerRequests::Start => {
                        // info!("hi");
                        app_state.lock().unwrap().player.curr_state = PlayerStates::STOPPED;
                        player_join_handle.take().map(JoinHandle::join);
                        app_state.lock().unwrap().player.curr_state = PlayerStates::PLAYING;
                        //TODO: this blocks curr thread // 
                        // info!("hi there");
                        
                        //init setup for a song
                        let x = app_state.lock().unwrap(); 
                        let song = match &x.ui.selected_song {
                            Some(song) => song,
                            None => continue,
                        };
                        let song_path = Path::new(&song.path);

                        let mut hint = Hint::new();

                        if let Some(extension) = song_path.extension() {
                            if let Some(extension_str) = extension.to_str() {
                                hint.with_extension(extension_str);
                            }
                        }

                        let source = match File::open(song_path) {
                            Ok(f) => Box::new(f),
                            Err(err) => {
                                panic!("Could not open song at path {}. Reason: {}", song.path, err)
                            }
                        };

                        let media_source_stream =
                            MediaSourceStream::new(source, Default::default());

                        let format_opts = FormatOptions {
                            enable_gapless: true,
                            ..Default::default()
                        };

                        let metadata_opts: MetadataOptions = Default::default();

                        let probed = symphonia::default::get_probe()
                            .format(&hint, media_source_stream, &format_opts, &metadata_opts)
                            .expect("unsupported media format");

                        let mut format = probed.format;

                        // Finds the first decodable track
                        let track = format
                            .tracks()
                            .iter()
                            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
                            .expect("No supported audio track");

                        let track_id = track.id;

                        let dec_opts: DecoderOptions = Default::default();
                        let track = match format.tracks().iter().find(|track| track.id == track_id)
                        {
                            Some(track) => track,
                            _ => return,
                        };

                        // TODO: have these .expects be errors that are returned and displayed
                        // TODO: user configurable option for how errors are displayed (popup or printed at the bottom)
                        let mut decoder = symphonia::default::get_codecs()
                            .make(&track.codec_params, &dec_opts)
                            .expect("unsupported codec");

                        let cloned_state = app_state.clone();
                        player_join_handle = Some(std::thread::spawn(move || player(cloned_state, &mut format, track_id, &mut decoder)));
                    }
                    PlayerRequests::Quit => return,
                    _ => (),
                },
                Err(err) => match err {
                    // TryRecvError::Empty => (),
                    RecvError => {
                        error!(
                            "Could not receive request to player app state. Reason: {}",
                            err.to_string()
                        );
                    }
                },
            }
        };
    }

    // TODO: change song to 'str path
    fn resume(&mut self) {
        // match self.curr_state {
        //     PlayerStates::PAUSED => self.curr_state = PlayerStates::PLAYING,
        //     _ => (),
        // }
    }
    fn stop(&mut self) {
        // self.curr_state = PlayerStates::STOPPED;
    }
    fn next(self) {
        unimplemented!()
    }
    fn prev(self) {
        unimplemented!()
    }
    fn seek(seconds: u64) {
        unimplemented!()
    }
    fn set_volume(level: f32) {
        unimplemented!()
    }
}

fn player(
    app_state: Arc<Mutex<AppState>>,
    format: &mut Box<dyn FormatReader>,
    track_id: u32,
    decoder: &mut Box<dyn Decoder>
    ) {
    let mut audio_output = None;
    loop {
        match app_state.lock().unwrap().player.curr_state {
            PlayerStates::STOPPED => break,
            _ => (),
        }
        // match player_rx.try_recv() {
        //     Ok(request) => match request {
        //             PlayerRequests::Stop => break,
        //             PlayerRequests::Pause => continue,
        //             PlayerRequests::Resume => {}
        //         _ => (),
        //     },
        //     Err(err) => match err {
        //         TryRecvError::Empty => (),
        //         TryRecvError::Disconnected => {
        //             error!(
        //             "Could not receive request to player app state. Reason: {}",
        //             err.to_string()
        //         );
        //         }
        //     },
        // }

        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(..) => return,
        };

        if packet.track_id() != track_id {
            continue;
        }

        while !format.metadata().is_latest() {
            format.metadata().pop();

            if let Some(rev) = format.metadata().current() {
                info!("{:?}", rev);
            }
        }

        let _ =
            decode_and_play(&mut audio_output, decoder, packet);
    }
}

fn decode_and_play(
    audio_output: &mut Option<Box<dyn output::AudioOutput>>,
    decoder: &mut Box<dyn Decoder>,
    packet: symphonia::core::formats::Packet,
) -> Result<(), symphonia::core::errors::Error> {
    match decoder.decode(&packet) {
        Ok(decoded) => {
            if audio_output.is_none() {
                let spec = *decoded.spec();

                let duration = decoded.capacity() as u64;
                audio_output.replace(output::try_open(spec, duration).unwrap());
            }

            if let Some(audio_output) = audio_output {
                audio_output.write(decoded).unwrap()
            }
        }
        Err(symphonia::core::errors::Error::DecodeError(err)) => {
            warn!("Decode error: {}", err);
        }
        Err(err) => return Err(err),
    }
    Ok(())
}

