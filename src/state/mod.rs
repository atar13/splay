use crate::{library::Song, utils::constants::PlayerStates};

pub struct AppState {
    pub searching: bool,
    pub search_term: String,
    pub curr_state: PlayerStates,
    pub curr_secs: u32,
    pub curr_song: Option<Song>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            searching: false,
            search_term: String::new(),
            curr_state: PlayerStates::STOPPED,
            curr_secs: 0,
            curr_song: None,
        }
    }
}
