use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, SupportedStreamConfig};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::library::Song;
use crate::player::{Player, PlayerRequests};
use crate::queue::SongQueue;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Instant;

use super::output;

pub struct SymphoniaPlayer {
    curr_song: Option<Song>,
    queue: SongQueue,
    curr_volume: f32,
    curr_state: PlayerStates,
}

enum PlayerStates {
    STOPPED,
    PLAYING,
    PAUSED,
}

// one "listen" or run method that reads from the receiver
// listen may also store device info for playback
// start method will start a given song and store
// pause/resume/stop will modify the current stream based on info stored by listen
impl Player for SymphoniaPlayer {
    fn init() -> SymphoniaPlayer {
        SymphoniaPlayer {
            curr_song: None,
            queue: SongQueue::new(),
            curr_volume: 0.,
            curr_state: PlayerStates::STOPPED,
        }
    }

    fn listen(mut self, rx: std::sync::mpsc::Receiver<PlayerRequests>) {
        let (player_tx, player_rx): (Sender<PlayerStates>, Receiver<PlayerStates>) =
            mpsc::channel();
        std::thread::spawn(move || run(player_rx));

        loop {
            match rx.recv() {
                Ok(request) => match request {
                    PlayerRequests::Stop => self.curr_state = PlayerStates::STOPPED,
                    PlayerRequests::Start(path) => {
                        match self.curr_state {
                            PlayerStates::PAUSED | PlayerStates::PLAYING => {
                                self.curr_state = PlayerStates::STOPPED;
                            }
                            _ => (),
                        }
                        self.curr_state = PlayerStates::PLAYING;
                        // self.player_thread =
                    }
                    PlayerRequests::Pause => self.curr_state = PlayerStates::PAUSED,
                    _ => (),
                },
                Err(err) => {
                    error!(
                        "Could not receive request to player app state. Reason: {}",
                        err.to_string()
                    );
                }
            }
        }
    }

    // TODO: change song to 'str path
    fn start(self, path: &str) {
        let song_path = Path::new(&path);

        let mut hint = Hint::new();

        if let Some(extension) = song_path.extension() {
            if let Some(extension_str) = extension.to_str() {
                hint.with_extension(extension_str);
            }
        }

        let source = match File::open(path) {
            Ok(f) => Box::new(f),
            Err(err) => panic!("Could not open song at path {}. Reason: {}", path, err),
        };

        let media_source_stream = MediaSourceStream::new(source, Default::default());

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

        let mut track_id = track.id;

        let mut audio_output: Option<Box<dyn output::AudioOutput>> = None;

        let dec_opts: DecoderOptions = Default::default();

        let _ = loop {
            match self.play_track(&mut format, &mut audio_output, track_id, &dec_opts) {
                Err(symphonia::core::errors::Error::ResetRequired) => {
                    track_id = format
                        .tracks()
                        .iter()
                        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
                        .unwrap()
                        .id
                }
                res => break res,
            }
        };
        if let Some(audio_output) = audio_output.as_mut() {
            audio_output.flush()
        }
    }
    fn resume(&mut self) {
        match self.curr_state {
            PlayerStates::PAUSED => self.curr_state = PlayerStates::PLAYING,
            _ => (),
        }
    }
    fn stop(&mut self) {
        self.curr_state = PlayerStates::STOPPED;
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
impl SymphoniaPlayer {
    fn play_track(
        &self,
        format: &mut Box<dyn FormatReader>,
        audio_output: &mut Option<Box<dyn output::AudioOutput>>,
        track_id: u32,
        dec_opts: &DecoderOptions,
    ) -> Result<(), symphonia::core::errors::Error> {
        let track = match format.tracks().iter().find(|track| track.id == track_id) {
            Some(track) => track,
            _ => return Ok(()),
        };

        // TODO: have these .expects be errors that are returned and displayed
        // TODO: user configurable option for how errors are displayed (popup or printed at the bottom)
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, dec_opts)
            .expect("unsupported codec");

        let time_base = track.codec_params.time_base;
        let duration = track
            .codec_params
            .n_frames
            .map(|frames| track.codec_params.start_ts + frames);

        let _: Result<(), symphonia::core::errors::Error> = loop {
            match self.curr_state {
                PlayerStates::STOPPED => return Ok(()),
                PlayerStates::PAUSED => continue,
                PlayerStates::PLAYING => (),
            }
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(err) => break Err(err),
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
                Err(err) => break Err(err),
            }
            // Regardless of result, finalize the decoder to get the verification result.
            let finalize_result = decoder.finalize();

            if let Some(verify_ok) = finalize_result.verify_ok {
                if verify_ok {
                    info!("verification passed");
                } else {
                    info!("verification failed");
                }
            }
        };
        Ok(())
    }
}

fn run(rx: Receiver<PlayerStates>) {
    let mut curr_state = PlayerStates::STOPPED;
    loop {
        match rx.try_recv() {
            Ok(new_state) => curr_state = new_state,
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

        match curr_state {
            PlayerStates::STOPPED => break,
            PlayerStates::PAUSED => continue,
            // need a decoder, packet, audio_output ane error return type
            PlayerStates::PLAYING => match decoder.decode(&packet) {
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
                Err(err) => break Err(err),
            },
        }
    }
}
