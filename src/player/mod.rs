pub mod output;
pub mod rodio_player;
pub mod symphonia_player;
use crate::{state::AppState, utils::constants::requests::*};
use std::sync::{mpsc::Receiver, Arc, Mutex};

pub trait Player {
    fn listen(&mut self, app_state: Arc<Mutex<AppState>>, rx: Receiver<PlayerRequests>);
}
