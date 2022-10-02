use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, SupportedStreamConfig};
use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::library::Song;
use crate::player::{Player, PlayerRequests};
use crate::queue::SongQueue;
use crate::state::AppState;
use crate::utils::constants::PlayerStates;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{self, RecvError, TryRecvError};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use super::output;

pub struct SymphoniaPlayer {
    // curr_song: Option<Song>,
    // queue: SongQueue,
    // curr_volume: f32,
    // curr_state: PlayerStates,
}

// one "listen" or run method that reads from the receiver
// listen may also store device info for playback
// start method will start a given song and store
// pause/resume/stop will modify the current stream based on info stored by listen
// impl Player for SymphoniaPlayer {
impl SymphoniaPlayer {
    pub fn init() -> SymphoniaPlayer {
        SymphoniaPlayer {
            // curr_song: None,
            // queue: SongQueue::new(),
            // curr_volume: 0.,
            // curr_state: PlayerStates::STOPPED,
        }
    }

    pub fn listen(mut self, rx: std::sync::mpsc::Receiver<PlayerRequests>, app_state: Arc<Mutex<AppState>>) {
        //listen for some events
        // if we are asked to play do some song setup
        // then go into a playing mode
        // still (or start inside this loop) to listen for requests but plays a packet on each iteration
        // we are only living inside this "playing loop"
        // anytime we get a stop inside this loop then we break
        // anytime we get a pause then we continue (but don't leave this loop back to the song setup)
        let _ = loop {
            match rx.recv() {
                Ok(request) => match request {
                    PlayerRequests::Start(path) => {
                        //init setup for a song
                        app_state.lock().unwrap().curr_state = PlayerStates::PLAYING;

                        let song_path = Path::new(&path);

                        let mut hint = Hint::new();

                        if let Some(extension) = song_path.extension() {
                            if let Some(extension_str) = extension.to_str() {
                                hint.with_extension(extension_str);
                            }
                        }

                        let source = match File::open(song_path) {
                            Ok(f) => Box::new(f),
                            Err(err) => {
                                panic!("Could not open song at path {}. Reason: {}", path, err)
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
                        let mut audio_output: Option<Box<dyn output::AudioOutput>> = None;

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

                        loop {
                            match rx.try_recv() {
                                Ok(request) => match request {
                                    PlayerRequests::Stop => app_state.lock().unwrap().curr_state = PlayerStates::STOPPED,
                                    PlayerRequests::Pause => app_state.lock().unwrap().curr_state = PlayerStates::PAUSED,
                                    PlayerRequests::Resume => {
                                        app_state.lock().unwrap().curr_state = PlayerStates::PLAYING
                                    }
                                    _ => (),
                                },
                                Err(err) => match err {
                                    TryRecvError::Empty => (),
                                    TryRecvError::Disconnected => {
                                        error!(
                                        "Could not receive request to player app state. Reason: {}",
                                        err.to_string()
                                    );
                                    }
                                },
                            }

                            match app_state.lock().unwrap().curr_state {
                                PlayerStates::PLAYING => {
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
                                        decode_and_play(&mut audio_output, &mut decoder, packet);
                                }
                                PlayerStates::STOPPED => break,
                                PlayerStates::PAUSED => continue,
                            }
                        }
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
