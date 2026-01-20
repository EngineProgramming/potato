use chess::position::Position;
use std::sync::atomic::AtomicBool;

/// UCI "go" arguments
#[derive(Default)]
#[allow(dead_code)]
pub struct GoOptions {
    pub wtime: Option<u64>,
    pub btime: Option<u64>,
    pub winc: Option<u64>,
    pub binc: Option<u64>,
    pub movestogo: Option<u32>,
    pub depth: Option<u32>,
    pub nodes: Option<u64>,
    pub movetime: Option<u64>,
    pub infinite: bool,
}

impl GoOptions {
    pub fn parse(rest: &str) -> Self {
        let mut options = Self::default();
        let mut tokens = rest.split_whitespace().peekable();

        while let Some(token) = tokens.next() {
            match token {
                "infinite" => options.infinite = true,
                "wtime" => options.wtime = tokens.next().and_then(|t| t.parse().ok()),
                "btime" => options.btime = tokens.next().and_then(|t| t.parse().ok()),
                "winc" => options.winc = tokens.next().and_then(|t| t.parse().ok()),
                "binc" => options.binc = tokens.next().and_then(|t| t.parse().ok()),
                "movestogo" => options.movestogo = tokens.next().and_then(|t| t.parse().ok()),
                "depth" => options.depth = tokens.next().and_then(|t| t.parse().ok()),
                "nodes" => options.nodes = tokens.next().and_then(|t| t.parse().ok()),
                "movetime" => options.movetime = tokens.next().and_then(|t| t.parse().ok()),
                _ => {}
            }
        }

        options
    }
}

pub fn handle_go(_pos: &Position, rest: &str, _stop: &AtomicBool) {
    let _options = GoOptions::parse(rest);

    // No search implemented yet.
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let options = GoOptions::parse("");
        assert_eq!(options.wtime, None);
        assert_eq!(options.btime, None);
        assert_eq!(options.winc, None);
        assert_eq!(options.binc, None);
        assert_eq!(options.movestogo, None);
        assert_eq!(options.depth, None);
        assert_eq!(options.nodes, None);
        assert_eq!(options.movetime, None);
        assert!(!options.infinite);
    }

    #[test]
    fn parse_time_controls() {
        let options =
            GoOptions::parse("wtime 300000 btime 290000 winc 1000 binc 2000 movestogo 40");
        assert_eq!(options.wtime, Some(300_000));
        assert_eq!(options.btime, Some(290_000));
        assert_eq!(options.winc, Some(1_000));
        assert_eq!(options.binc, Some(2_000));
        assert_eq!(options.movestogo, Some(40));
    }

    #[test]
    fn parse_depth_nodes_movetime() {
        let options = GoOptions::parse("depth 10 nodes 100000 movetime 5000");
        assert_eq!(options.depth, Some(10));
        assert_eq!(options.nodes, Some(100_000));
        assert_eq!(options.movetime, Some(5_000));
    }

    #[test]
    fn parse_infinite() {
        let options = GoOptions::parse("infinite");
        assert!(options.infinite);
    }

    #[test]
    fn parse_ignores_unknown_tokens() {
        let options = GoOptions::parse("ponder searchmoves e2e4 d2d4 depth 5");
        assert_eq!(options.depth, Some(5));
    }

    #[test]
    fn parse_missing_value_leaves_field_unset() {
        let options = GoOptions::parse("depth");
        assert_eq!(options.depth, None);
    }
}
