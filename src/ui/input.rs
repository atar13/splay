use crossterm::{
    event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers},
    Result,
};
use std::sync::mpsc::Sender;

use crate::utils::constants::Requests::*;

pub fn listen_for_input(app_tx: Sender<AppRequests>, ui_tx: Sender<UIRequests>) {
    'main_input: loop {
        let quit = || {
            let _ = ui_tx.send(UIRequests::CleanupAndQuit);
            let _ = app_tx.send(AppRequests::Quit);
        };
        match read().unwrap() {
            Event::Key(event) => match event {
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                } => match ui_tx.send(UIRequests::Up) {
                    Ok(_) => (),
                    Err(err) => error!(
                        "Could not send request for up key. Reason {:?}",
                        err.to_string()
                    ),
                },
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                } => match ui_tx.send(UIRequests::Down) {
                    Ok(_) => info!("Down"),
                    Err(err) => error!(
                        "Could not send request for down key. Reason {:?}",
                        err.to_string()
                    ),
                },
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                } => {
                    quit();
                    break 'main_input;
                }
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                } => {
                    quit();
                    break 'main_input;
                }
                _ => (),
            },
            _ => (),
        }
    }
}
