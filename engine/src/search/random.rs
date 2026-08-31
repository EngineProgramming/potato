use crate::uci::go::GoOptions;
use chess::{mv::Mv, position::Position};
use rand::seq::IteratorRandom;
use std::sync::atomic::AtomicBool;

/// Pick a random legal move
#[must_use]
pub fn search(pos: &mut Position, _options: &GoOptions, _stop: &AtomicBool) -> Option<Mv> {
    pos.pseudolegal_moves()
        .into_iter()
        .filter(|mv| {
            let legal = pos.makemove::<false>(mv);
            pos.undomove();
            legal
        })
        .choose(&mut rand::rng())
}
