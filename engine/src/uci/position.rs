use chess::position::Position;

pub fn handle_position(pos: &mut Position, rest: &str) {
    let tokens: Vec<&str> = rest.split_whitespace().collect();
    let moves_idx = tokens.iter().position(|&t| t == "moves");
    let (setup, moves) = match moves_idx {
        Some(idx) => (&tokens[..idx], &tokens[idx + 1..]),
        None => (&tokens[..], &[][..]),
    };

    let npos = match setup {
        ["startpos", ..] => Position::from_fen("startpos"),
        ["fen", fen_parts @ ..] => Position::from_fen(&fen_parts.join(" ")),
        _ => return,
    };

    let Ok(mut npos) = npos else { return };

    for movestr in moves {
        let Ok(mv) = movestr.parse() else { break };

        if !npos.makemove::<true>(&mv) {
            npos.undomove();
            break;
        }
    }

    *pos = npos;
}

#[cfg(test)]
mod tests {
    use super::*;

    const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn startpos() {
        let mut pos = Position::from_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        handle_position(&mut pos, "startpos");
        assert_eq!(pos.get_fen(), STARTPOS_FEN);
    }

    #[test]
    fn fen() {
        let mut pos = Position::from_fen("startpos").unwrap();
        let fen = "4k3/8/8/8/8/8/8/4K3 w - - 0 1";
        handle_position(&mut pos, &format!("fen {fen}"));
        assert_eq!(pos.get_fen(), fen);
    }

    #[test]
    fn startpos_with_moves() {
        let mut pos = Position::from_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        handle_position(&mut pos, "startpos moves e2e4 e7e5");
        assert_eq!(
            pos.get_fen(),
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2"
        );
    }

    #[test]
    fn fen_with_moves() {
        let mut pos = Position::from_fen("startpos").unwrap();
        handle_position(&mut pos, "fen 4k3/8/8/8/8/8/8/4K3 w - - 0 1 moves e1e2");
        assert_eq!(pos.get_fen(), "4k3/8/8/8/8/8/4K3/8 b - - 1 1");
    }

    #[test]
    fn unknown_setup_keeps_position_unchanged() {
        let mut pos = Position::from_fen("startpos").unwrap();
        let before = pos.get_fen();
        handle_position(&mut pos, "bogus");
        assert_eq!(pos.get_fen(), before);
    }

    #[test]
    fn invalid_fen_keeps_position_unchanged() {
        let mut pos = Position::from_fen("startpos").unwrap();
        let before = pos.get_fen();
        handle_position(&mut pos, "fen not a valid fen");
        assert_eq!(pos.get_fen(), before);
    }

    #[test]
    fn illegal_move_stops_replay_without_corrupting_position() {
        let fen = "4k3/1P2r3/1q6/5N2/2n3b1/4Q1p1/3n4/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen("startpos").unwrap();

        handle_position(&mut pos, &format!("fen {fen} moves e1e2"));

        assert_eq!(pos.get_fen(), fen);
    }
}
