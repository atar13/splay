use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::utils::constants::Requests::{AppRequests, PlayerRequests, UIRequests};

pub struct Keybinds {
    pub lookup: HashMap<KeyEvent, AppRequests>,
}

impl Default for Keybinds {
    fn default() -> Self {
        let mut lookup: HashMap<KeyEvent, AppRequests> = HashMap::new();
        lookup.insert(
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::Quit,
        );
        lookup.insert(
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            },
            AppRequests::Quit,
        );
        lookup.insert(
            KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::UIRequests(UIRequests::Down),
        );
        lookup.insert(
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::UIRequests(UIRequests::Up),
        );
        // lookup.insert(
        //     KeyEvent {
        //         code: KeyCode::Enter,
        //         modifiers: KeyModifiers::NONE,
        //     },
        //     AppRequests::UIRequests(UIRequests::Enter),
        // );
        lookup.insert(
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::UIRequests(UIRequests::Down),
        );
        lookup.insert(
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::UIRequests(UIRequests::Up),
        );
        lookup.insert(
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::PlayerRequests(PlayerRequests::Start),
        );
        lookup.insert(
            KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::CONTROL,
            },
            AppRequests::UIRequests(UIRequests::ShowSearch),
        );

        lookup.insert(
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::UIRequests(UIRequests::GoBack),
        );

        lookup.insert(
            KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::PlayerRequests(PlayerRequests::Pause),
        );

        lookup.insert(
            KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::PlayerRequests(PlayerRequests::Resume),
        );
        lookup.insert(
            KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::NONE,
            },
            AppRequests::PlayerRequests(PlayerRequests::Stop),
        );

        return Keybinds { lookup };
    }
}
