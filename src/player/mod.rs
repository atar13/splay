pub mod rodio_player;
pub mod symphonia_player;
use crate::library::Song;
use std::sync::mpsc::{Receiver, Sender};

pub enum PlayerRequests {
    STOP,
    START(Song),
    RESUME,
    PAUSE,
    NEXT,
    PREV,
    SEEK(u64),
    CHANGE_VOLUME(f32),
}

pub trait Player {
    fn init() -> Self;
    fn listen(self, rx: Receiver<PlayerRequests>);
    fn start(self, song: Song);
    fn resume();
    fn stop();
    fn next();
    fn prev();
    fn seek(seconds: u64);
    fn set_volume(level: f32);
}
