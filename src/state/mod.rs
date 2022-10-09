use crate::{library::{Song, Library, self}, utils::constants::PlayerStates};

pub struct AppState {
    pub library: Library,
    pub ui: UIState,
    pub player: PlayerState,
    pub search: SearchState,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            library: Library::default(), 
            ui: UIState::default(),
            player: PlayerState::default(),
            search: SearchState::default(),
        }
    }
}


pub struct UIState {
    curr_tab: u8,
    selected_pane: u8,
    selected_row: u8,
    pub selected_song: Option<Song>
}

impl Default for UIState {
    fn default() -> Self {
        Self { curr_tab: 0, selected_pane: 0, selected_row: 0, selected_song: None }
    }
}

pub struct PlayerState {
    pub curr_state: PlayerStates,
    pub curr_secs: u32,
    pub curr_song: Option<Song>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self { curr_state: PlayerStates::STOPPED, curr_secs: 0, curr_song: None }
    }
}

pub struct SearchState {
    pub searching: bool,
    pub term: String,
}

impl Default for SearchState {
    fn default() -> Self {
        Self { searching: false, term: String::default() }
    }
}
