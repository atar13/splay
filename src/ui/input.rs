use crossterm::{
    event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers},
    Result,
};
use std::sync::mpsc::Sender;

use crate::utils::constants::Requests::*;

pub fn listen_for_input(app_tx: Sender<AppRequests>, ui_tx: Sender<UIRequests>) {
    let ctrlC = KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    };
    'main_input: loop {
        match read().unwrap() {
            Event::Key(event) => match event {
                // ctrlC => {
                //     let _ = app_tx.send(AppRequests::QUIT);
                // }
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                } => {
                    let _ = ui_tx.send(UIRequests::CleanupAndQuit);
                    let _ = app_tx.send(AppRequests::Quit);
                    break 'main_input;
                }
                _ => {
                    println!("{:?}", event)
                }
            },
            _ => (),
        }
    }
}
