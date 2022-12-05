pub mod helper;
pub mod widgets;

use crate::library::song::Song;
use crate::player::symphonia_player::SymphoniaPlayer;
use crate::player::Player;
use crate::state::AppState;
use crate::utils::constants::requests::{AppRequests, PlayerRequests, UIRequests::*};
use crate::utils::constants::PlayerStates;
use crate::{library::Library, utils::constants::requests::UIRequests};
use std::sync::{mpsc, Arc, Mutex};
use std::{
    fmt::format,
    io::{self, Stdout},
    sync::mpsc::{Receiver, Sender},
    time::{Duration, Instant},
};
use std::{thread, time};
use tui::layout::Alignment;
use tui::widgets::Wrap;
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

pub fn start<'a>(
    app_state: Arc<Mutex<AppState>>,
    rx: Receiver<UIRequests>,
    main_tx: Sender<AppRequests>,
) {
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

    let songs = app_state.lock().unwrap().library.songs.to_owned();
    let app = App::with_songs(app_state, songs);
    app.run(&mut terminal, rx, main_tx);

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

pub struct App {
    state: Arc<Mutex<AppState>>,
    song_list: StatefulList<Song>,
}

impl App {
    pub fn new(state: Arc<Mutex<AppState>>) -> App {
        App {
            state,
            song_list: StatefulList::with_items(vec![]),
        }
    }

    pub fn with_songs(state: Arc<Mutex<AppState>>, songs: Vec<Song>) -> App {
        App {
            state,
            song_list: StatefulList::with_items(songs),
        }
    }

    #[warn(unreachable_patterns)]
    pub fn run(
        mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        rx: Receiver<UIRequests>,
        main_tx: Sender<AppRequests>,
    ) -> () {
        self.on_down(); //select first element

        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            terminal.draw(|f| self.get_ui(f, &main_tx)).unwrap();
            match rx.recv_timeout(timeout) {
                Ok(request) => match request {
                    Up => self.on_up(),
                    Down => self.on_down(),
                    Enter => self.on_enter(),
                    ShowSearch => self.state.lock().unwrap().search.searching = true,
                    SearchInput(ch) => self.state.lock().unwrap().search.term.push(ch),
                    GoBack => self.go_back(),
                    Quit => return,
                    _ => {
                        error!("This UI event is not implemented yet")
                    }
                },
                Err(err) => match err {
                    mpsc::RecvTimeoutError::Disconnected => error!(
                        "Could not receive UI event. \n \t Reason: {}",
                        err.to_string()
                    ),
                    _ => (),
                },
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn on_up(&mut self) {
        self.song_list.previous();
        self.state.lock().unwrap().ui.selected_song = Some(
            self.song_list
                .items
                .get(self.song_list.state.selected().unwrap())
                .unwrap()
                .clone(),
        );
    }

    fn on_down(&mut self) {
        self.song_list.next();
        self.state.lock().unwrap().ui.selected_song = Some(
            self.song_list
                .items
                .get(self.song_list.state.selected().unwrap())
                .unwrap()
                .clone(),
        );
    }

    fn on_enter(&mut self) {}

    fn go_back(&mut self) {
        if self.state.lock().unwrap().search.searching {
            self.state.lock().unwrap().search.searching = false;
            self.state.lock().unwrap().search.term.clear();
        }
    }

    fn get_ui<B: Backend>(&mut self, frame: &mut Frame<B>, main_tx: &Sender<AppRequests>) {
        let size = frame.size();
        let block = Block::default().title("tarvrs").borders(Borders::ALL);
        frame.render_widget(block, size);

        let vert_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(frame.size());

        let horiz_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(vert_chunks[1]);

        let song_list_vert_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(horiz_chunks[0]);

        // match &self.state.lock().unwrap().player.curr_state {
        //     PlayerStates::PLAYING => {
        //         let block = Block::default().title("Popup").borders(Borders::ALL);
        //         let area = helper::centered_rect(60, 60, size);
        //         let selected_song = self
        //             .song_list
        //             .items
        //             .get(self.song_list.state.selected().unwrap());
        //         let paragraph = Paragraph::new(format!("{:#?}", selected_song.unwrap()))
        //             .style(Style::default().fg(Color::White))
        //             .alignment(Alignment::Left);
        //         frame.render_widget(Clear, area);
        //         frame.render_widget(paragraph, block.inner(area));
        //         frame.render_widget(block, area);
        //     }
        //     x => info!("{:?}", x)
        // }

        let mut filtered_songs: Vec<Song> = Vec::new();
        if self.state.lock().unwrap().search.searching {
            let search = Paragraph::new(self.state.lock().unwrap().search.term.to_owned())
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false });
            frame.render_widget(Clear, song_list_vert_chunks[0]);
            frame.render_widget(search, song_list_vert_chunks[0]);

            let search_term = &self.state.lock().unwrap().search.term;
            for song in self.song_list.items.iter() {
                if song.title.contains(search_term) {
                    filtered_songs.push(song.clone());
                }
            }
        } else {
            filtered_songs = self.song_list.items.clone();
        }

        let filtered_stateful_list = StatefulList::with_items(filtered_songs);

        let list: Vec<ListItem> = filtered_stateful_list
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

        frame.render_stateful_widget(list, song_list_vert_chunks[1], &mut self.song_list.state);
        widgets::curr_playing_bar::render(frame, vert_chunks[0], &(self.state.lock().unwrap()));
    }
}
