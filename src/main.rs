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
    let mut lib = Library::new();
    // let filename = &args[1];
    // let result = lib.import_file(filename);
    let result = lib.import_dir("/home/atarbinian/Desktop/sample"); // TODO: allow to use ~

    // lib.save_to_csv();
    match result {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    };
    match lib.read_from_csv() {
        Ok(_) => (),
        Err(e) => error!("{:?}", e),
    }
    // error!("{}", filename);
    // match lib.read_from_bin() {
    //     Ok(_) => (),
    //     Err(e) => error!("{:?}", e),
    // }

    let mut children = vec![];

    let (main_tx, main_rx): (Sender<AppRequests>, Receiver<AppRequests>) = mpsc::channel();
    let (ui_tx, ui_rx): (Sender<UIRequests>, Receiver<UIRequests>) = mpsc::channel();

    let songs = match lib.artist_map.get("Daft Punk") {
        Some(artist) => {
            let mut songs = Vec::new();
            for album in &artist.album_titles {
                for song_title in &lib.album_map.get(album).unwrap().song_titles {
                    songs.push(lib.song_map.get(song_title).unwrap().clone())
                }
            }
            songs
        }
        None => Vec::new(),
    };

    children.push(thread::spawn(move || ui::start(ui_rx, songs)));
    children.push(thread::spawn(move || ui::input::listen_for_input(main_tx)));

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
                    for child in children {
                        let _ = child.join();
                    }
                    info!("Gracefully shutting down");
                    std::process::exit(0);
                }
                AppRequests::UIUp => {
                    let _ = ui_tx.send(UIRequests::Up);
                }
                AppRequests::UIDown => {
                    let _ = ui_tx.send(UIRequests::Down);
                }
                AppRequests::UIEnter => {
                    let _ = ui_tx.send(UIRequests::Enter);
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
    // // test
}
