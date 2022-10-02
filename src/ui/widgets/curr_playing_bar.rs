use std::{
    fmt::format,
    sync::{Arc, Mutex},
};

use tui::{
    layout::{Rect, Layout, Direction, Constraint},
    style::{Style, Color, Modifier},
    text::{Span, Spans},
    widgets::{Paragraph, Gauge, Block, Borders},
    Frame,
};

use crate::{queue::SongQueue, state::AppState, utils::constants::PlayerStates};

pub fn render(frame: &mut Frame<impl tui::backend::Backend>, area: Rect, state: &AppState) {
    match &state.curr_song {
        None => (),
        Some(song) => error!("{}", song.title.to_owned()),
    };


    let song_title = match &state.curr_song {
        None => Span::raw("N/A"),
        Some(song) => Span::raw(song.title.to_owned()),
    };

    let curr_time = Span::raw(readable_time(state.curr_secs));
    let total_time = match &state.curr_song {
        None => Span::raw("00:00"),
        Some(song) => Span::raw(song.duration.to_owned()),
    };

    let time_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Song Progress"))
        .gauge_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(20);

    let play_status = match &state.curr_state {
        PlayerStates::PLAYING => Span::raw("Playing"),
        PlayerStates::STOPPED => Span::raw("Stopped"),
        PlayerStates::PAUSED => Span::raw("Paused"),
    };

    let text = vec![
        Spans::from(song_title),
        Spans::from(vec![
            curr_time,
            Span::raw("/"),
            total_time,
        ]),
        Spans::from(play_status),
    ];

    let chunks = Layout::default().direction(Direction::Vertical).constraints(
        [
        Constraint::Percentage(60),
        Constraint::Percentage(40),
        ].as_ref()
        ).split(area);

    frame.render_widget(Paragraph::new(text), chunks[0]);
    frame.render_widget(time_gauge, chunks[1]);
}

fn readable_time(secs: u32) -> String {
    let mins = secs / 60;
    let secs = secs % 60;

    return format!("{:02}:{:02}", mins, secs);
}
