use std::io::Write;
use std::process::{Command, Stdio};

fn run(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_potato"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
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
fn test_uci_handshake() {
    let output = run("uci\nquit\n");
    let lines: Vec<&str> = output.lines().collect();

    assert_eq!(lines[0], "id name Potato");
    assert_eq!(lines[1], "id author kz04px");
    assert_eq!(lines[2], "uciok");
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
    // "ucinewgame" is the skipped-isready command here rather than "go",
    // since there's no search yet - go is exercised separately below.
    let output = run("uci\nucinewgame\nisready\nquit\n");

    assert_eq!(output.lines().filter(|line| *line == "readyok").count(), 1);
}

#[test]
fn test_go_is_not_yet_implemented() {
    // No search exists yet, so "go" runs on its background thread, panics,
    // and that panic is swallowed when its handle is joined - the engine
    // keeps running (no bestmove, no crash) rather than the whole process
    // going down.
    let output = run("uci\nisready\nposition startpos\ngo\nquit\n");

    assert!(!output.lines().any(|line| line.starts_with("bestmove")));
}

#[test]
fn test_stop_after_go_is_harmless_before_search_exists() {
    let output = run("uci\nisready\nposition startpos\ngo\nstop\nquit\n");
    assert!(!output.lines().any(|line| line.starts_with("bestmove")));
}

#[test]
fn test_stop_with_no_search_in_progress_is_harmless() {
    let output = run("uci\nisready\nstop\nquit\n");
    assert!(output.lines().any(|line| line == "readyok"));
}
