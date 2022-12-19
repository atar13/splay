mod input;
mod library;
mod player;
mod queue;
mod state;
mod ui;
mod utils;

use crate::library::Library;
use crate::player::rodio_player::RodioPlayer;
use crate::player::symphonia_player::SymphoniaPlayer;
use crate::player::Player;
use crate::state::AppState;
use crate::utils::constants::requests::*;

#[macro_use]
extern crate log;
use simplelog::*;
use std::fs::File;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::io::Read;
use serde::Deserialize;

use toml::Table;

#[derive(Deserialize)]
struct SplayConfig {
    media: Media
}

#[derive(Deserialize)]
struct Media {
    directories: Vec<String>
}


fn main() {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("splay.log").unwrap(),
    );
    info!("Starting splay...");

    // let args: Vec<String> = env::args().collect();

    let state = Arc::new(Mutex::new(AppState::default()));

    // let mut player = SymphoniaPlayer::new();
    let mut player = RodioPlayer::new();
    let mut lib = Library::new();


    // TODO: change to other path on system in XDG_CONFIG_HOME 
    match File::open("config.toml") {
        Ok(mut in_file) => {
            let mut in_contents = String::new();
            in_file.read_to_string(&mut in_contents).unwrap();
            let config: SplayConfig = toml::from_str(in_contents.as_str()).unwrap();
            
            for dir in config.media.directories {
                // TODO: allow to use ~
                match lib.import_dir(dir.as_str()) {
                    Ok(_) => {
                        let _ = lib.save_to_file("db".to_string());
                    }
                    Err(e) => error!("{}", e),
                }
            }
        }
        Err(_) => (),
    }


    state.lock().unwrap().library = lib;

    let mut join_handlers = vec![];

    let (main_tx, main_rx): (Sender<AppRequests>, Receiver<AppRequests>) = mpsc::channel();
    let (ui_tx, ui_rx): (Sender<UIRequests>, Receiver<UIRequests>) = mpsc::channel();
    let (player_tx, player_rx): (Sender<PlayerRequests>, Receiver<PlayerRequests>) =
        mpsc::channel();

    let cloned_state = state.clone();
    let cloned_main_tx = main_tx.clone();
    join_handlers.push(thread::spawn(move || {
        ui::start(cloned_state, ui_rx, cloned_main_tx)
    }));

    let cloned_state = state.clone();
    let cloned_main_tx = main_tx.clone();
    join_handlers.push(thread::spawn(move || {
        input::listen(cloned_state, cloned_main_tx)
    }));

    let cloned_state = state.clone();
    // let cloned_main_tx = main_tx.clone();
    join_handlers.push(thread::spawn(move || {
        player.listen(cloned_state, player_rx)
    }));

    loop {
        match main_rx.recv() {
            Err(err) => {
                error!(
                    "Could not receive request to modify app state. Reason: {}",
                    err.to_string()
                );
            }
            Ok(request) => match request {
                AppRequests::Quit => {
                    let _ = ui_tx.send(UIRequests::Quit);
                    let _ = player_tx.send(PlayerRequests::Stop);
                    let _ = player_tx.send(PlayerRequests::Quit);
                    for handler in join_handlers {
                        let _ = handler.join();
                    }
                    info!("Gracefully shutting down");
                    std::process::exit(0);
                }
                AppRequests::UIRequests(request) => {
                    let _ = ui_tx.send(request);
                }
                AppRequests::PlayerRequests(request) => {
                    let _ = player_tx.send(request);
                }
            },
        }
    }
}
