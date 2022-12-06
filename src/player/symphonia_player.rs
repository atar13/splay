use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::player::PlayerRequests;
use crate::state::AppState;
use crate::utils::constants::PlayerStates;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvError;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use super::{output, Player};

pub struct SymphoniaPlayer {}

impl SymphoniaPlayer {
    pub fn new() -> SymphoniaPlayer {
        SymphoniaPlayer {}
    }
}

impl Player for SymphoniaPlayer {
    // listen for actions the player should take
    fn listen(&mut self, app_state: Arc<Mutex<AppState>>, rx: Receiver<PlayerRequests>) {
        let mut join_handle: Option<JoinHandle<()>> = None;

        let _result = loop {
            match rx.recv() {
                Ok(request) => match request {
                    PlayerRequests::Quit => return,
                    PlayerRequests::Stop => {
                        app_state.lock().unwrap().player.curr_state = PlayerStates::STOPPED;
                        join_handle.take().map(JoinHandle::join);
                        app_state.lock().unwrap().player.curr_song = None;
                    }
                    PlayerRequests::Pause => {
                        app_state.lock().unwrap().player.curr_state = PlayerStates::PAUSED;
                    }
                    PlayerRequests::Resume => {
                        app_state.lock().unwrap().player.curr_state = PlayerStates::PLAYING;
                    }
                    PlayerRequests::PlayPause => {
                        match app_state.lock().unwrap().player.curr_state {
                            PlayerStates::PLAYING => {
                                app_state.lock().unwrap().player.curr_state = PlayerStates::PAUSED
                            }
                            PlayerStates::PAUSED => {
                                app_state.lock().unwrap().player.curr_state = PlayerStates::PLAYING
                            }
                            _ => (),
                        }
                    }
                    PlayerRequests::Start => {
                        // stop player if previously playing
                        app_state.lock().unwrap().player.curr_state = PlayerStates::STOPPED;
                        join_handle.take().map(JoinHandle::join);
                        app_state.lock().unwrap().player.curr_state = PlayerStates::PLAYING;

                        // init setup for playing a song

                        // fetch which song is selected in the UI
                        // TODO: maybe just have other threads modify player.curr_song instead
                        let song = match app_state.lock().unwrap().ui.selected_song.to_owned() {
                            Some(song) => song,
                            None => continue,
                        };

                        app_state.lock().unwrap().player.curr_song = Some(song.to_owned());

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
                                //TODO: return Result instead of panic here
                            }
                        };

                        let media_source_stream =
                            MediaSourceStream::new(source, Default::default());

                        let format_opts = FormatOptions {
                            enable_gapless: true, // TODO: have this be a config option
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

                        // spin up another thread that will start playing audio
                        join_handle = Some(std::thread::spawn(move || {
                            player(cloned_state, &mut format, track_id, &mut decoder)
                        }));
                    }
                },
                Err(err) => match err {
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
}

fn player(
    app_state: Arc<Mutex<AppState>>,
    format: &mut Box<dyn FormatReader>,
    track_id: u32,
    decoder: &mut Box<dyn Decoder>,
) {
    let mut audio_output = None;
    app_state.lock().unwrap().player.progress = Duration::ZERO;

    loop {
        match app_state.lock().unwrap().player.curr_state {
            PlayerStates::STOPPED => break,
            PlayerStates::PAUSED => {
                continue;
            }
            _ => (),
        }

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

        let start_packet_time = Instant::now(); // record the time before a packet is played
        let _ = play_packet(&mut audio_output, decoder, packet);
        let mut guard = app_state.lock().unwrap(); //idk I just did this not to call lock() a bunch
                                                   //of times
                                                   // update player time with how long the last packet took to play
        guard.player.progress = guard.player.progress + start_packet_time.elapsed();
        drop(guard);
    }
}

fn play_packet(
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
