// TODO: make SearchDB a trait and have different variations that implement it (songs, artists, albums, playlists, genres ...)

struct node {
    is_word: bool,
    left_child: Box<node>,
    middle_child: Box<node>,
    right_child: Box<node>,
}

pub struct SearchDB {
    pathToDB: String,
    words: Vec<String>,
}

impl SearchDB {
    pub fn new() -> SearchDB {
        SearchDB {
            pathToDB: String::new(),
            words: Vec::new(),
        }
    }

    pub fn find_matches(self, input: String) -> (Vec<String>, Vec<String>, Vec<String>) {
        return (Vec::new(), Vec::new(), Vec::new());
    }

    fn find_match(self, input: String) {}

    pub fn insert(self, input: String) -> Result<(), String> {
        return Ok(());
    }

    fn remove(self, input: String) -> Result<(), String> {
        return Ok(());
    }

    fn save(self) -> Result<(), String> {
        unimplemented!()
    }

    fn load() -> Result<(), String> {
        unimplemented!()
    }
}
