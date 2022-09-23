mod library;
mod metadata;
mod player;
mod queue;
mod ui;
mod utils;
mod state;

use crate::library::search::SearchDB;
use crate::library::Library;
use crate::player::symphonia_player::SymphoniaPlayer;
use crate::player::Player;
use crate::state::AppState;
use crate::ui::input;
use crate::utils::constants::Requests::*;
use core::time;
use std::env;
use std::process::exit;

#[macro_use]
extern crate log;
use simplelog::*;
use std::fs::File;
use std::io::BufReader;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("tarvrs.log").unwrap(),
    );
    info!("Starting tarvrs...");

    // let args: Vec<String> = env::args().collect();
    
    let player = SymphoniaPlayer::init();
    let mut lib = Library::new();
    
    match lib.import_dir("/home/atarbinian/Desktop/media") { // TODO: allow to use ~
        Ok(_) => { lib.save_to_bin(); } ,
        Err(e) => error!("{}", e),
    }
    // info!("{} songs added", len())

    // match lib.read_from_bin() {
    //     Ok(_) => (),
    //     Err(e) => error!("{:?}", e),
    // }

    let mut children = vec![];

    let (main_tx, main_rx): (Sender<AppRequests>, Receiver<AppRequests>) = mpsc::channel();
    let (ui_tx, ui_rx): (Sender<UIRequests>, Receiver<UIRequests>) = mpsc::channel();
    let (player_tx, player_rx): (Sender<PlayerRequests>, Receiver<PlayerRequests>) =
        mpsc::channel();

    let app_state = Arc::new(Mutex::new(AppState::new()));
    let ui_app_state = app_state.clone();
    let input_app_state = app_state.clone();
    
    // All threads should only take their own receiver and the transmitter to the main thread
    let ui_to_player_tx = player_tx.clone();
    children.push(thread::spawn(move || ui::start(ui_app_state, ui_rx, lib.songs, ui_to_player_tx)));
    children.push(thread::spawn(move || ui::input::listen(input_app_state, main_tx)));
    children.push(thread::spawn(move || player.listen(player_rx)));

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
                    let _ = for child in children {
                        let _ = child.join();
                    };
                    info!("Gracefully shutting down");
                    std::process::exit(0);
                }
                AppRequests::UIRequests(request) => { 
                    let _ = ui_tx.send(request);
                }
                _ => (),
            },
        }
    }

    // let (tx, rx): (Sender<PlayerRequests>, Receiver<PlayerRequests>) = mpsc::channel();
    // // let mut children = vec![];
    // let player = SymphoniaPlayer::init();
    // // player.listen(rx);
    // // children.push(thread::spawn(move || {
    // //     player.start();
    // // }));
    // // for child in children {
    // //     let _ = child.join();
    // // }
    // // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // match lib.song_map.get_vec_mut(filename) {
    //     Some(songs) => {
    //         player.start(songs[0].clone());
    //         let result = tx.send(PlayerRequests::START(songs[0].clone()));
    //         match result {
    //             Ok(_) => (),
    //             Err(e) => error!("{:?}", e),
    //         }
    //         thread::sleep(std::time::Duration::from_secs(2));
    //         // let result = tx.send(PlayerRequests::STOP);
    //         // match result {
    //         //     Ok(_) => (),
    //         //     Err(e) => error!("{:?}", e)
    //         // }
    //         // let result = tx.send(PlayerRequests::START(songs[0].clone()));
    //         // match result {
    //         //     Ok(_) => (),
    //         //     Err(e) => error!("{:?}", e)
    //         // }
    //         // let result = tx.send(PlayerRequests::SEEK(50));
    //         // match result {
    //         //     Ok(_) => (),
    //         //     Err(e) => error!("{:?}", e)
    //         // }
    //         // thread::sleep(std::time::Duration::from_secs(2));
    //         // let result = tx.send(PlayerRequests::RESUME);
    //         // match result {
    //         //     Ok(_) => (),
    //         //     Err(e) => error!("{:?}", e)
    //         // }
    //     }
    //     _ => {
    //         error!("no song {}", filename)
    //     }
    // }
    // loop {}

    // // lib.save_to_csv();
    // // lib.save_to_bin();
    // // tes
}
