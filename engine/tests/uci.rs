use std::io::Write;
use std::process::{Command, Stdio};

fn run(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_potato"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start engine");

    child
        .stdin
        .take()
        .expect("engine stdin was not piped")
        .write_all(input.as_bytes())
        .expect("failed to write to engine stdin");

    let output = child
        .wait_with_output()
        .expect("engine did not exit cleanly");

    String::from_utf8(output.stdout).expect("engine wrote non-utf8 output")
}

#[test]
fn test_non_uci_first_line_produces_no_output() {
    let output = run("xboard\nprotover 2\n");
    assert_eq!(output, "");
}

#[test]
fn test_isready() {
    let output = run("uci\nisready\nquit\n");
    assert!(output.lines().any(|line| line == "readyok"));
}

#[test]
fn test_isready_answered_every_time_its_sent() {
    let output = run("uci\nisready\nisready\nisready\nquit\n");
    assert_eq!(output.lines().filter(|line| *line == "readyok").count(), 3);
}

#[test]
fn test_quit_before_isready_sends_no_readyok() {
    let output = run("uci\nquit\n");
    assert!(!output.lines().any(|line| line == "readyok"));
}

#[test]
fn test_skipping_isready_still_processes_the_first_command() {
    let output = run("uci\nposition startpos\ngo\nquit\n");

    assert!(!output.lines().any(|line| line == "readyok"));
    assert!(output.lines().any(|line| line.starts_with("bestmove")));
}

#[test]
fn test_go_from_startpos_returns_a_legal_move() {
    let output = run("uci\nisready\nposition startpos\ngo\nquit\n");
    let bestmove = output
        .lines()
        .find_map(|line| line.strip_prefix("bestmove "))
        .expect("no bestmove in output");

    let legal_first_moves = [
        "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3",
        "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3",
    ];
    assert!(
        legal_first_moves.contains(&bestmove),
        "unexpected bestmove {bestmove}"
    );
}

#[test]
fn test_go_after_position_with_moves() {
    let output = run("uci\nisready\nposition startpos moves e2e4 e7e5\ngo\nquit\n");
    assert!(output.lines().any(|line| line.starts_with("bestmove")));
}

#[test]
fn test_go_from_fen_with_a_single_legal_move() {
    // White king a1 has only one legal move: the black king on b3 covers a1's
    // other two flight squares (a2 and b2), leaving only b1.
    let output = run("uci\nisready\nposition fen 8/8/8/8/8/1k6/8/K7 w - - 0 1\ngo\nquit\n");
    assert!(output.lines().any(|line| line == "bestmove a1b1"));
}

#[test]
fn test_go_with_no_legal_moves_returns_null_move() {
    // Checkmate: white to move, no legal moves at all.
    let output = run(
        "uci\nisready\nposition fen rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 2\ngo\nquit\n",
    );
    assert!(output.lines().any(|line| line == "bestmove 0000"));
}

#[test]
fn test_stop_after_go_still_yields_a_bestmove() {
    let output = run("uci\nisready\nposition startpos\ngo\nstop\nquit\n");
    let bestmove = output
        .lines()
        .find_map(|line| line.strip_prefix("bestmove "))
        .expect("no bestmove in output");

    let legal_first_moves = [
        "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3",
        "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3",
    ];
    assert!(
        legal_first_moves.contains(&bestmove),
        "unexpected bestmove {bestmove}"
    );
    // Exactly one bestmove: "stop" must not trigger a second search response.
    assert_eq!(
        output.lines().filter(|l| l.starts_with("bestmove")).count(),
        1
    );
}

#[test]
fn test_stop_with_no_search_in_progress_is_harmless() {
    let output = run("uci\nisready\nstop\nposition startpos\ngo\nquit\n");
    assert!(output.lines().any(|line| line.starts_with("bestmove")));
}

#[test]
fn test_ucinewgame_resets_position() {
    let output = run(
        "uci\nisready\nposition fen 8/8/8/8/8/1k6/8/K7 w - - 0 1\nucinewgame\nposition startpos\ngo\nquit\n",
    );
    let bestmove = output
        .lines()
        .find_map(|line| line.strip_prefix("bestmove "))
        .expect("no bestmove in output");

    let legal_first_moves = [
        "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3",
        "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3",
    ];
    assert!(
        legal_first_moves.contains(&bestmove),
        "unexpected bestmove {bestmove}"
    );
}
