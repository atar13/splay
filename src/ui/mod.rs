pub mod input;
pub mod widgets;

use crate::utils::constants::Requests::UIRequests;
use crate::utils::constants::Requests::UIRequests::*;
use std::{
    fmt::format,
    io,
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};
use widgets::stateful_list::StatefulList;

use crossterm::{
    cursor, event,
    event::Event,
    event::KeyCode,
    execute, style,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

struct UIState<'a> {
    songListState: StatefulList<&'a str>,
}

pub struct UI<'a> {
    term: Box<Terminal<CrosstermBackend<io::Stdout>>>,
    state: UIState<'a>,
}
pub fn start(rx: Receiver<UIRequests>) {
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

    // 'main_ui: loop {
    //     match rx.recv() {
    //         #[warn(unreachable_patterns)]
    //         Ok(request) => match request {
    //             Up => self.on_up(),
    //             Down => self.on_down(),
    //             Cleanup => self.cleanup(),
    //             CleanupAndQuit => {
    //                 self.cleanup();
    //                 // self.clear();
    //                 break 'main_ui;
    //             }
    //             _ => {
    //                 error!("This UI event is not implemented yet")
    //             }
    //         },
    //         Err(err) => {
    //             error!(
    //                 "Could not receive UI event. \n \t Reason: {}",
    //                 err.to_string()
    //             )
    //         }
    //     }
    // }

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    'main: loop {
        // terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break 'main,
                    // KeyCode::Left => app.items.unselect(),
                    // KeyCode::Down => app.items.next(),
                    // KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
    }

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();

    // //  draw borders
    // self.term
    //     .draw(|frame| {
    //         let size = frame.size();
    //         let block = Block::default().title("tarvrs").borders(Borders::ALL);
    //         frame.render_widget(block, size);
    //     })
    //     .unwrap();

    // self.term
    //     .draw(|frame| {
    //         let chunks = Layout::default()
    //             .direction(Direction::Vertical)
    //             .margin(1)
    //             .constraints(
    //                 [
    //                     Constraint::Percentage(10),
    //                     Constraint::Percentage(80),
    //                     Constraint::Percentage(10),
    //                 ]
    //                 .as_ref(),
    //             )
    //             .split(frame.size());
    //         let block = Block::default().title("Block").borders(Borders::ALL);
    //         frame.render_widget(block, chunks[0]);
    //         // let block = Block::default().title("Block 2").borders(Borders::ALL);
    //         // frame.render_widget(block, chunks[1]);
    //         let block = Block::default().title("Block 3").borders(Borders::ALL);
    //         frame.render_widget(block, chunks[2]);

    //         let items: Vec<&'a str> = vec!["Item 1", "Item 2", "Item 3"];
    //         self.state.songListState = StatefulList::with_items(items);
    //         // self.state.songListState.state.select(Some(2));
    //         let list: Vec<ListItem> = self
    //             .state
    //             .songListState
    //             .items
    //             .iter()
    //             .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
    //             .collect();
    //         let list = List::new(list)
    //             .block(Block::default().borders(Borders::ALL).title("Songs"))
    //             .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    //             .highlight_symbol(">> ");

    //         self.state.songListState.next();
    //         self.state.songListState.next();
    //         frame.render_stateful_widget(list, chunks[1], &mut self.state.songListState.state);
    //         self.state.songListState.next();
    //     })
    //     .unwrap();
}

impl<'a> UI<'a> {
    // pub fn new() -> UI<'a> {
    //     UI {
    //         // term: Terminal::new(backend).unwrap(),
    //         term: Box::new(terminal),
    //         state: UIState {
    //             songListState: StatefulList::with_items(Vec::new()),
    //         },
    //     }
    // }

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
        // restore terminal
        disable_raw_mode().unwrap();
        execute!(
            self.term.backend_mut(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )
        .unwrap();
        self.term.show_cursor().unwrap();
        // execute!(
        //     io::stdout(),
        //     style::ResetColor,
        //     cursor::Show,
        //     terminal::LeaveAlternateScreen,
        // )
        // .unwrap();

        // disable_raw_mode().unwrap();
    }

    fn on_up(&mut self) {
        match self.state.songListState.state.selected() {
            None => info!("could not get selected item"),
            Some(idx) => info!("{}", idx),
        }
        self.state.songListState.previous()
    }

    fn on_down(&mut self) {
        match self.state.songListState.state.selected() {
            None => info!("could not get selected item"),
            Some(idx) => info!("{}", idx),
        }
        self.state.songListState.next();
    }

    fn refresh(&mut self) {}
}
