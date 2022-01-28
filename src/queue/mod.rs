use crate::library::Song;

use std::collections::VecDeque;

pub struct SongQueue {
    previous_queue: VecDeque<Song>,
    immediate_queue: VecDeque<Song>,
    upcoming_queue: VecDeque<Song>,
}

impl SongQueue {
    pub fn new() -> SongQueue {
        SongQueue {
            previous_queue: VecDeque::new(),
            immediate_queue: VecDeque::new(),
            upcoming_queue: VecDeque::new(),
        }
    }

    pub fn add_first_immediate(&mut self, song: Song) {
        self.immediate_queue.push_front(song);
    }

    pub fn add_last_immediate(&mut self, song: Song) {
        self.immediate_queue.push_back(song);
    }

    pub fn add_upcoming(&mut self, song: Song) {
        self.upcoming_queue.push_back(song);
    }

    pub fn add_to_previous(&mut self, song: Song) {
        self.previous_queue.push_back(song);
    }

    pub fn next(&mut self) -> Option<Song> {
        if self.immediate_queue.is_empty() {
            return self.upcoming_queue.pop_front();
        } else {
            return self.immediate_queue.pop_front();
        }
    }
}
