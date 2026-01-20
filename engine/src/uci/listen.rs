use super::go::handle_go;
use super::position::handle_position;
use chess::position::Position;
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

/// Initialisation
fn initialise() {}

/// Stop any ongoing search
fn stop_search(stop: &AtomicBool, search_thread: &mut Option<JoinHandle<()>>) {
    stop.store(true, Ordering::Relaxed);

    if let Some(handle) = search_thread.take() {
        let _ = handle.join();
    }
}

pub fn listen() {
    println!("id name Potato");
    println!("id author kz04px");
    println!("uciok");
    io::stdout().flush().ok();

    let mut pos = Position::from_fen("startpos").unwrap();
    let stdin = io::stdin();
    let mut pending = None;
    let stop = Arc::new(AtomicBool::new(false));
    let mut search_thread: Option<JoinHandle<()>> = None;

    // Wait for the first "isready"
    if let Some(line) = stdin.lock().lines().next() {
        let Ok(line) = line else { return };

        match line.trim() {
            "isready" => {
                initialise();
                println!("readyok");
                io::stdout().flush().ok();
            }
            "quit" => return,
            _ => {
                initialise();
                pending = Some(line);
            }
        }
    }

    // Main listening loop
    let lines = pending.into_iter().map(Ok).chain(stdin.lock().lines());
    for line in lines {
        let Ok(line) = line else { break };
        let line = line.trim();
        let (command, rest) = line.split_once(' ').unwrap_or((line, ""));

        match command {
            "isready" => println!("readyok"),
            "ucinewgame" => pos = Position::from_fen("startpos").unwrap(),
            "position" => handle_position(&mut pos, rest),
            "go" => {
                // Join any ongoing search
                stop_search(&stop, &mut search_thread);
                stop.store(false, Ordering::Relaxed);

                let pos = pos.clone();
                let rest = rest.to_string();
                let stop = Arc::clone(&stop);
                search_thread = Some(thread::spawn(move || handle_go(&pos, &rest, &stop)));
            }
            "stop" => stop_search(&stop, &mut search_thread),
            "quit" => {
                stop_search(&stop, &mut search_thread);
                break;
            }
            _ => {}
        }

        io::stdout().flush().ok();
    }

    stop_search(&stop, &mut search_thread);
}
