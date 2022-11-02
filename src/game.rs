use serde::{Deserialize, Serialize};

use crate::{challenge::*, event::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<String>,
    pub variant: Option<Variant>,
    pub speed: Option<Speed>,
    pub perf: Option<Perf>,
    pub rated: Option<bool>,
    #[serde(rename = "initialFen")]
    pub initial_fen: Option<String>,
    pub fen: String,
    pub player: Option<String>,
    pub turns: Option<u16>,
    pub started_at_turn: Option<u16>,
    pub source: Option<GameSource>,
    pub status: Option<GameStatus>,
    pub created_at: Option<u128>,
    pub last_move: Option<String>,
    pub wc: Option<u32>,
    pub bc: Option<u32>,
    pub lm: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStatus {
    pub id: u32,
    name: String,
}

impl Game {
    pub fn from_json_str(json: &str) -> Option<Result<Self, serde_json::Error>> {
        crate::utils::json_deserialize(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_game_data_serialization() {
        let initial_dump = r#"{"id":"kq8SsXb8","variant":{"key":"standard","name":"Standard","short":"Std"},
        "speed":"correspondence","perf":"correspondence","rated":false,
        "fen":"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "player":"white","turns":0,"startedAtTurn":0,"source":"friend","status":{"id":20,"name":"started"},
        "createdAt":1667070660041}"#;

        let deserialized = Game::from_json_str(initial_dump).unwrap();
        assert!(deserialized.is_ok());
    }

    #[test]
    fn game_move_serialization() {
        let move_output =
            r#"{"fen":"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b","lm":"e2e4"}"#;
        let deserialized = Game::from_json_str(move_output).unwrap();
        assert!(deserialized.is_ok());

        let move2_output =
            r#"{"fen":"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w","lm":"b8c6"}"#;
        let deserialized = Game::from_json_str(move2_output).unwrap();
        assert!(deserialized.is_ok());

        let move3_output =
            r#"{"fen":"r1bqkbnr/pppp1ppp/2n5/4p1N1/4P3/8/PPPP1PPP/RNBQKB1R b","lm":"f3g5"}"#;
        let deserialized = Game::from_json_str(move3_output).unwrap();
        assert!(deserialized.is_ok());
    }

    #[test]
    fn final_game_data_serialization() {
        let final_dump = r#"{"id":"kq8SsXb8","variant":{"key":"standard","name":"Standard","short":"Std"},
        "speed":"correspondence","perf":"correspondence","rated":false,
        "fen":"r1bqkbnr/pppp1ppp/2n5/4p1N1/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 3 3",
        "player":"black","turns":5,"startedAtTurn":0,"source":"friend","status":{"id":31,"name":"resign"},
        "createdAt":1667070660041,"winner":"black","lastMove":"f3g5"}"#;

        let deserialized = Game::from_json_str(final_dump).unwrap();
        assert!(deserialized.is_ok())
    }
}
