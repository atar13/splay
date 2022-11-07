mod input;
mod library;
mod metadata;
mod player;
mod queue;
mod state;
mod ui;
mod utils;

use crate::library::Library;
use crate::player::symphonia_player::SymphoniaPlayer;
use crate::state::AppState;
use crate::utils::constants::Requests::*;

#[macro_use]
extern crate log;
use simplelog::*;
use std::fs::File;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("tarvrs.log").unwrap(),
    );
    info!("Starting tarvrs...");

    // let args: Vec<String> = env::args().collect();

    let state = Arc::new(Mutex::new(AppState::default()));

    let player = SymphoniaPlayer::init();
    let mut lib = Library::new();

    match lib.import_dir("/home/atarbinian/Desktop/media") {
        // TODO: allow to use ~
        Ok(_) => {
            lib.save_to_bin();
        }
        Err(e) => error!("{}", e),
    }

    // match lib.read_from_bin() {
    //     Ok(_) => (),
    //     Err(e) => error!("{:?}", e),
    // }

    state.lock().unwrap().library = lib;

    let mut children = vec![];

    let (main_tx, main_rx): (Sender<AppRequests>, Receiver<AppRequests>) = mpsc::channel();
    let (ui_tx, ui_rx): (Sender<UIRequests>, Receiver<UIRequests>) = mpsc::channel();
    let (player_tx, player_rx): (Sender<PlayerRequests>, Receiver<PlayerRequests>) =
        mpsc::channel();

    // let control Cont
    // All threads should only take their own receiver and the transmitter to the control thread
    let cloned_state = state.clone();
    let cloned_main_tx = main_tx.clone();
    children.push(thread::spawn(move || {
        ui::start(cloned_state, ui_rx, cloned_main_tx)
    }));
    let cloned_state = state.clone();
    let cloned_main_tx = main_tx.clone();
    children.push(thread::spawn(move || {
        input::listen(cloned_state, cloned_main_tx)
    }));
    // TODO: investigate high cpu on player thread
    let cloned_state = state.clone();
    let cloned_main_tx = main_tx.clone();
    children.push(thread::spawn(move || {
        player.listen(cloned_state, player_rx, cloned_main_tx)
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
                    //TODO: this can go in control
                    let _ = ui_tx.send(UIRequests::Quit);
                    let _ = player_tx.send(PlayerRequests::Stop);
                    let _ = player_tx.send(PlayerRequests::Quit);
                    //
                    let _ = for child in children {
                        let _ = child.join();
                    };
                    info!("Gracefully shutting down");
                    std::process::exit(0);
                }
                AppRequests::UIRequests(request) => {
                    let _ = ui_tx.send(request);
                }
                AppRequests::PlayerRequests(request) => {
                    let _ = player_tx.send(request);
                }
                _ => (),
            },
        }
    }
}
