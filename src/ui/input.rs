use crossterm::{
    event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers},
    Result,
};
use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc, Mutex},
    time::{Duration, Instant},
};

use crate::{utils::constants::Requests::*, state::AppState};

pub fn listen(state: Arc<Mutex<AppState>>, main_tx: Sender<AppRequests>) {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let key_lookup = get_key_lookup();

    'input: loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                info!("{}", state.lock().unwrap().searching);
                if state.lock().unwrap().searching {
                    if let KeyCode::Char(ch) = key.code {
                        main_tx.send(AppRequests::UIRequests(UIRequests::SearchInput(ch)));
                        continue 'input;
                    }
                }
                match key_lookup.get(&key) {
                    Some(request) => {
                        let _ = main_tx.send(request.to_owned());
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
    // 'main_input: loop {
    //     let quit = || {
    //         let _ = ui_tx.send(UIRequests::CleanupAndQuit);
    //         let _ = app_tx.send(AppRequests::Quit);
    //     };
    //     match read().unwrap() {
    //         Event::Key(event) => match event {
    //             KeyEvent {
    //                 code: KeyCode::Up,
    //                 modifiers: KeyModifiers::NONE,
    //             } => match ui_tx.send(UIRequests::Up) {
    //                 Ok(_) => (),
    //                 Err(err) => error!(
    //                     "Could not send request for up key. Reason {:?}",
    //                     err.to_string()
    //                 ),
    //             },
    //             KeyEvent {
    //                 code: KeyCode::Down,
    //                 modifiers: KeyModifiers::NONE,
    //             } => match ui_tx.send(UIRequests::Down) {
    //                 Ok(_) => info!("Down"),
    //                 Err(err) => error!(
    //                     "Could not send request for down key. Reason {:?}",
    //                     err.to_string()
    //                 ),
    //             },
    //             KeyEvent {
    //                 code: KeyCode::Char('q'),
    //                 modifiers: KeyModifiers::NONE,
    //             } => {
    //                 quit();
    //                 break 'main_input;
    //             }
    //             KeyEvent {
    //                 code: KeyCode::Char('c'),
    //                 modifiers: KeyModifiers::CONTROL,
    //             } => {
    //                 quit();
    //                 break 'main_input;
    //             }
    //             _ => (),
    //         },
    //         _ => (),
    //     }
    // }
}

fn get_key_lookup<'a>() -> HashMap<KeyEvent, AppRequests> {
    let mut key_lookup: HashMap<KeyEvent, AppRequests> = HashMap::new();
    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
        },
        AppRequests::Quit,
    );
    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        },
        AppRequests::Quit,
    );
    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
        },
        AppRequests::UIRequests(UIRequests::Down),
    );
    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
        },
        AppRequests::UIRequests(UIRequests::Up),
    );
    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        },
        AppRequests::UIRequests(UIRequests::Enter),
    );
    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        },
        AppRequests::UIRequests(UIRequests::Down),
    );
    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        },
        AppRequests::UIRequests(UIRequests::Up),
    );
    // key_lookup.insert(
    //     KeyEvent {
    //         code: KeyCode::Enter,
    //         modifiers: KeyModifiers::NONE,
    //     },
    //     AppRequests::PlayerRequests(PlayerRequests::Start(())),
    // );
    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::CONTROL,
        },
        AppRequests::UIRequests(UIRequests::ShowSearch),
    );

    key_lookup.insert(
        KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
        },
        AppRequests::UIRequests(UIRequests::GoBack),
    );

    return key_lookup;
}
