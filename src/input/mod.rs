mod keybinds;

use crossterm::event::{self, Event, KeyCode};
use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    time::{Duration, Instant},
};

use keybinds::Keybinds;

use crate::{state::AppState, utils::constants::requests::*};

pub fn listen(app_state: Arc<Mutex<AppState>>, main_tx: Sender<AppRequests>) {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let binds = Keybinds::default();

    'input: loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                //Handle search input
                //TODO: move to another function "handle_search_input" and have it return a bool
                //(should continue searching or not)
                if app_state.lock().unwrap().search.searching {
                    // TODO: refactor this so that it modifies the search variables
                    if let KeyCode::Char(ch) = key.code {
                        // TODO: if enter is pressed then stop reading keyboard input and display
                        // results
                        // TODO: backspace to remove last character from search term
                        _ = main_tx.send(AppRequests::UIRequests(UIRequests::SearchInput(ch)));
                        continue 'input;
                    }
                }

                //Handle all other keyboard input and check if a kebind exists for them
                match binds.lookup.get(&key) {
                    Some(request) => {
                        let _ = main_tx.send(request.to_owned());

                        //TODO: maybe refactor this so that this thread listens for a Quit signal
                        //from a transmitter instead
                        match request {
                            AppRequests::Quit => break,
                            _ => (),
                        }
                    }
                    None => (),
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
