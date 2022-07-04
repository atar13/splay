pub mod input;
use crate::utils::constants::Requests::UIRequests;
use std::{io, sync::mpsc::Receiver};

use crossterm::{
    cursor,
    event::DisableMouseCapture,
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    text::Text,
    widgets::{Block, Borders},
    Terminal,
};

pub struct UI {
    term: Terminal<CrosstermBackend<io::Stdout>>,
}

impl UI {
    pub fn new() -> UI {
        enable_raw_mode().unwrap();
        let backend = CrosstermBackend::new(io::stdout());
        let mut term = Terminal::new(backend).unwrap();
        UI { term }
    }
    pub fn start_ui(mut self, rx: Receiver<UIRequests>) {
        info!("Starting up UI...");
        self.clear();
        self.term
            .draw(|frame| {
                let size = frame.size();
                let block = Block::default().title("tarvrs").borders(Borders::ALL);
                frame.render_widget(block, size);
            })
            .unwrap();
        'main_ui: loop {
            match rx.recv() {
                #[warn(unreachable_patterns)]
                Ok(request) => match request {
                    UIRequests::Cleanup => self.cleanup(),
                    UIRequests::CleanupAndQuit => {
                        self.cleanup();
                        self.clear();
                        break 'main_ui;
                    }
                    _ => {
                        error!("This UI event is not implemented yet")
                    }
                },
                Err(err) => {
                    error!(
                        "Could not receive UI event. \n \t Reason: {}",
                        err.to_string()
                    )
                }
            }
        }
    }

    fn clear(&mut self) {
        match self.term.clear() {
            Err(err) => {
                error!(
                    "Could not clear terminal screen. \n \t Reason: {}",
                    err.to_string()
                )
            }
            _ => {}
        }
    }

    fn cleanup(&mut self) {
        execute!(
            io::stdout(),
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )
        .unwrap();

        disable_raw_mode().unwrap();
    }
}
