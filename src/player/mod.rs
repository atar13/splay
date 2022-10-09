pub mod output;
pub mod rodio_player;
pub mod symphonia_player;
use crate::utils::constants::Requests::*;
use std::sync::mpsc::Receiver;

pub trait Player {
    fn init() -> Self;
    fn listen(self, rx: Receiver<PlayerRequests>);
    fn start(self, path: &str);
    fn resume(&mut self);
    fn stop(&mut self);
    fn next(self);
    fn prev(self);
    fn seek(seconds: u64);
    fn set_volume(level: f32);
}
