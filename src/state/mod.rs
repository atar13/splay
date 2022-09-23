
pub struct AppState {
    pub searching: bool,
    pub search_term: String,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { searching: false, search_term: String::new() }
    }
}
