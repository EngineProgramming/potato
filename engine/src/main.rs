mod search;
mod uci;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let first_line = stdin.lock().lines().next();
    let Some(Ok(first_line)) = first_line else {
        return;
    };

    #[allow(clippy::single_match)]
    match first_line.trim() {
        "uci" => uci::listen(),
        _ => {}
    }
}
