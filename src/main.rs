mod library;
mod metadata;
mod player;
mod queue;
mod ui;
mod utils;

use crate::library::search::SearchDB;
use crate::library::Library;
// use crate::player::rodio_player::RodioPlayer;
// use crate::player::symphonia_player::SymphoniaPlayer;
use crate::player::{Player, PlayerRequests};
use crate::ui::input::listen_for_input;
use crate::ui::UI;
use crate::utils::constants::Requests::*;
use core::time;
use std::env;
use std::process::exit;

#[macro_use]
extern crate log;
use simplelog::*;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
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

    let mut children = vec![];

    let (app_tx, app_rx): (Sender<AppRequests>, Receiver<AppRequests>) = mpsc::channel();
    let (ui_tx, ui_rx): (Sender<UIRequests>, Receiver<UIRequests>) = mpsc::channel();

    // children.push(thread::spawn(move || ui::start(ui_rx)));
    ui::start(ui_rx);
    exit(0);
    children.push(thread::spawn(move || listen_for_input(app_tx, ui_tx)));
    // ui.cleanup();
    loop {
        match app_rx.recv() {
            Err(err) => {
                error!(
                    "Could not receive request to modify app state. Reason: {}",
                    err.to_string()
                );
            }
            Ok(request) => match request {
                AppRequests::Quit => {
                    for child in children {
                        let _ = child.join();
                    }
                    std::process::exit(0);
                }
            },
        }
    }

    // let mut lib = Library::new();
    // let filename = &args[1];
    // // let result = lib.import_file(filename);
    // // let result = lib.import_dir(filename);
    // // lib.save_to_csv();
    // // match result {
    // //     Ok(_) => (),
    // //     Err(e) => println!("{}", e),
    // // };
    // match lib.read_from_csv() {
    //     Ok(_) => (),
    //     Err(e) => error!("{:?}", e),
    // }
    // // error!("{}", filename);
    // // match lib.read_from_bin() {
    // //     Ok(_) => (),
    // //     Err(e) => error!("{:?}", e),
    // // }

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
    // // test
}
