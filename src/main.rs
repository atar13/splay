mod library;
mod metadata;
mod player;
mod queue;
mod utils;
use crate::library::Library;
use crate::player::{Player, PlayerRequests};
use std::env;
#[macro_use]
extern crate log;
use rodio::{source::Source, Decoder, OutputStream};
use simplelog::*;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;


fn main() {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("tarvrs.log").unwrap(),
    );
    info!("Starting tarvrs...");

    let args: Vec<String> = env::args().collect();
    let mut lib = Library::new();
    let filename = &args[1];
    // let result = lib.import_file(filename);
    // let result = lib.import_dir(filename);
    // match result {
    //     Ok(_) => (),
    //     Err(e) => println!("{}", e),
    // };
    match lib.read_from_csv() {
        Ok(_) => (),
        Err(e) => error!("{:?}", e),
    }
    // error!("{}", filename);
    // match lib.read_from_bin() {
    //     Ok(_) => (),
    //     Err(e) => error!("{:?}", e),
    // }

    let (tx, rx): (Sender<PlayerRequests>, Receiver<PlayerRequests>) = mpsc::channel();
    // let mut children = vec![];
    let player = Player::init();
    player.start(rx);
    // children.push(thread::spawn(move || {
    //     player.start();
    // }));
    // for child in children {
    //     let _ = child.join();
    // }
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    match lib.song_map.get_vec_mut(filename) {
        Some(songs) => {
            let result = tx.send(PlayerRequests::START(songs[0].clone()));
            match result {
                Ok(_) => (),
                Err(e) => error!("{:?}", e)
            }
            thread::sleep(std::time::Duration::from_secs(2));
            let result = tx.send(PlayerRequests::PAUSE);
            match result {
                Ok(_) => (),
                Err(e) => error!("{:?}", e)
            }
            thread::sleep(std::time::Duration::from_secs(2));
            let result = tx.send(PlayerRequests::RESUME);
            match result {
                Ok(_) => (),
                Err(e) => error!("{:?}", e)
            }
            thread::sleep(std::time::Duration::from_secs(2));
            let result = tx.send(PlayerRequests::SEEK(5));
            match result {
                Ok(_) => (),
                Err(e) => error!("{:?}", e)
            }
        },
        _ =>  {error!("no song {}", filename)}
    }
    loop {}

    // lib.save_to_csv();
    // lib.save_to_bin();
}
