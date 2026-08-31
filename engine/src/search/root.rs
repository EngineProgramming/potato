use crate::{search::eval::eval, uci::go::GoOptions};
use chess::{mv::Mv, position::Position};
use rand::seq::IteratorRandom;
use std::{sync::atomic::AtomicBool, time::Instant};

/// Pick a random legal move from those that maximise our material after 1 ply
#[must_use]
pub fn search(pos: &mut Position, _options: &GoOptions, _stop: &AtomicBool) -> Option<Mv> {
    let mut best_moves = Vec::new();
    let mut best_score = i32::MIN;
    let mut nodes = 0;
    let t0: Instant = Instant::now();

    for mv in pos.pseudolegal_moves() {
        let legal = pos.makemove::<false>(&mv);
        if !legal {
            pos.undomove();
            continue;
        }

        let score = -eval(pos);
        pos.undomove();

        nodes += 1;

        match score.cmp(&best_score) {
            std::cmp::Ordering::Greater => {
                best_score = score;
                best_moves.clear();
                best_moves.push(mv);
            }
            std::cmp::Ordering::Equal => best_moves.push(mv),
            std::cmp::Ordering::Less => {}
        }
    }
    let dt = t0.elapsed().as_secs_f64().max(f64::MIN_POSITIVE);
    let bestmove = best_moves.into_iter().choose(&mut rand::rng());
    let pv = if let Some(bm) = &bestmove {
        bm.to_string()
    } else {
        "0000".to_string()
    };

    println!(
        "info depth {} nodes {} time {} nps {} pv {}",
        1,
        nodes,
        dt as u64,
        (nodes as f64 / dt) as u64,
        pv,
    );

    bestmove
}
