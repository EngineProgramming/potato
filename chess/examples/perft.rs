use chess::{perft::perft, position::Position};
use std::time::Instant;

fn main() -> Result<(), ()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    let mut fen = "startpos".to_string();
    let mut depth = 6u32;

    for part in args.split("--").skip(1).map(str::trim) {
        if let Some(value) = part.strip_prefix("fen ") {
            fen = value.trim().to_string();
        } else if let Some(value) = part.strip_prefix("depth ") {
            depth = value.trim().parse().expect("Invalid depth");
        }
    }

    let mut pos = Position::from_fen(&fen)?;
    let mut total_nodes = 0;
    let t0 = Instant::now();

    for i in 1..=depth {
        let nodes = perft(&mut pos, i);
        let elapsed = t0.elapsed();
        total_nodes += nodes;
        let nps = (total_nodes as f64 / elapsed.as_secs_f64().max(f64::MIN_POSITIVE)) as u64;
        println!(
            "info depth {} nodes {} time {} nps {}",
            i,
            nodes,
            elapsed.as_millis(),
            nps
        );

        if i == depth {
            println!("nodes {}", nodes);
        }
    }

    Ok(())
}
