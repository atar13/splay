use std::{
    fmt::format,
    sync::{Arc, Mutex},
};

use tui::{
    layout::{Alignment::Left, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Gauge, Paragraph},
    Frame,
};

use crate::{queue::SongQueue, state::AppState, utils::constants::PlayerStates};

pub fn render(frame: &mut Frame<impl tui::backend::Backend>, area: Rect, state: &AppState) {
    let song_title = match &state.player.curr_song {
        None => Span::raw(""),
        Some(song) => Span::raw(song.title.to_owned()),
    };

    let song_artist = match &state.player.curr_song {
        None => Span::raw(""),
        Some(song) => Span::raw(song.track_artist.to_owned()),
    };

    let curr_time_secs = state.player.progress.as_secs();

    let total_time_secs = match &state.player.curr_song {
        None => 0,
        Some(song) => song.duration_secs,
    };

    let curr_time_span = Span::raw(readable_time(curr_time_secs));

    let total_time_span = Span::raw(readable_time(total_time_secs));

    let percentage_played: f64 = match total_time_secs {
        0 => 0.,
        _ => (curr_time_secs as f64) / (total_time_secs as f64),
    };

    let time_gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::ITALIC),
        )
        .ratio(percentage_played);

    let play_status = match &state.player.curr_state {
        PlayerStates::PLAYING => Span::raw("Playing"),
        PlayerStates::STOPPED => Span::raw("Stopped"),
        PlayerStates::PAUSED => Span::raw("Paused"),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(area);

    let player_info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let player_status_text = vec![
        Spans::from(vec![curr_time_span, Span::raw("/"), total_time_span]),
        Spans::from(play_status),
    ];
    let song_status_text = vec![Spans::from(song_title), Spans::from(song_artist)];

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(player_status_text).alignment(Left),
        player_info_chunks[0],
    );
    frame.render_widget(
        Paragraph::new(song_status_text).alignment(Left),
        player_info_chunks[1],
    );
    frame.render_widget(time_gauge, chunks[1]);
}

fn readable_time(secs: u64) -> String {
    let mins = secs / 60;
    let secs = secs % 60;

    return format!("{:02}:{:02}", mins, secs);
}
