pub mod helper;
pub mod input;
pub mod widgets;

use crate::library::Song;
use crate::player::symphonia_player::SymphoniaPlayer;
use crate::player::Player;
use crate::utils::constants::Requests::UIRequests::*;
use crate::{library::Library, utils::constants::Requests::UIRequests};
use std::{
    fmt::format,
    io::{self, Stdout},
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};
use tui::layout::Alignment;
use widgets::stateful_list::StatefulList;

use crossterm::{
    cursor, event,
    event::Event,
    event::KeyCode,
    execute, style,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

pub fn start<'a>(rx: Receiver<UIRequests>, songs: Vec<Song>) {
    info!("Starting up UI...");

    // initialize terminal state
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::Hide,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )
    .unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    debug!("Terminal started successfully");

    let app = App::with_songs(songs);
    app.run(&mut terminal, rx);

    info!("stopping now");

    // restore terminal
    info!("Starting to cleanup terminal ...");
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
    info!("Terminal cleaned successfully");
}

struct State {
    song_list: StatefulList<Song>,
}

pub struct App {
    state: State,
    tmp_show_popup: bool,
}

impl App {
    pub fn new() -> App {
        let state = State {
            song_list: StatefulList::with_items(vec![]),
        };
        App {
            state,
            tmp_show_popup: false,
        }
    }

    pub fn with_songs(songs: Vec<Song>) -> App {
        let state = State {
            song_list: StatefulList::with_items(songs),
        };
        App {
            state,
            tmp_show_popup: false,
        }
    }

    #[warn(unreachable_patterns)]
    pub fn run(
        mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        rx: Receiver<UIRequests>,
    ) -> () {
        self.state.song_list.next(); // select first element

        loop {
            terminal.draw(|f| self.get_ui(f)).unwrap();
            match rx.recv() {
                Ok(request) => match request {
                    Up => self.on_up(),
                    Down => self.on_down(),
                    Enter => self.on_enter(),
                    Quit => return,
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

    fn on_up(&mut self) {
        self.state.song_list.previous()
    }

    fn on_down(&mut self) {
        self.state.song_list.next();
    }

    fn on_enter(&mut self) {
        self.tmp_show_popup = !self.tmp_show_popup;
    }

    fn get_ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let size = frame.size();
        let block = Block::default().title("tarvrs").borders(Borders::ALL);
        frame.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let block = Block::default().title("Block").borders(Borders::ALL);
        frame.render_widget(block, chunks[0]);

        let block = Block::default().title("Block 3").borders(Borders::ALL);
        frame.render_widget(block, chunks[2]);

        let list: Vec<ListItem> = self
            .state
            .song_list
            .items
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(i.title.clone())]))
            .collect();

        let list = List::new(list)
            .block(Block::default().borders(Borders::ALL).title("Songs"))
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, chunks[1], &mut self.state.song_list.state);

        if self.tmp_show_popup {
            let block = Block::default().title("Popup").borders(Borders::ALL);
            let area = helper::centered_rect(60, 60, size);
            let selected_song = self
                .state
                .song_list
                .items
                .get(self.state.song_list.state.selected().unwrap());
            let paragraph = Paragraph::new(format!("{:#?}", selected_song.unwrap()))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);
            frame.render_widget(Clear, area);
            frame.render_widget(paragraph, block.inner(area));
            frame.render_widget(block, area);
        }
    }
}
