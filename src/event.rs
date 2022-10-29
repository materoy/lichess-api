use serde::{Deserialize, Serialize};

use crate::challenge::Challenge;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub r#type: EventType,
    pub game: Option<GameEventInfo>,
    pub challenge: Option<Challenge>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    #[serde(rename = "gameStart")]
    GameStart,
    #[serde(rename = "gameFinish")]
    GameFinish,
    #[serde(rename = "challenge")]
    Challenge,
    #[serde(rename = "challengeCanceled")]
    ChallengeCanceled,
    #[serde(rename = "challengeDeclined")]
    ChallengeDeclined,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameEventInfo {
    pub id: String,
    pub source: GameSource,
    pub compat: Compat,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameSource {
    #[serde(rename = "lobby")]
    Lobby,
    #[serde(rename = "friend")]
    Friend,
    #[serde(rename = "ai")]
    Ai,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "tournament")]
    Tournament,
    #[serde(rename = "position")]
    Position,
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "importlive")]
    ImportLive,
    #[serde(rename = "simul")]
    Simul,
    #[serde(rename = "relay")]
    Relay,
    #[serde(rename = "pool")]
    Pool,
    #[serde(rename = "swiss")]
    Swiss,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compat {
    pub bot: bool,
    pub board: bool,
}

impl Event {
    pub fn from_json_str(json: &str) -> Option<Result<Self, serde_json::Error>> {
        crate::utils::json_deserialize(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_game_start_deserialization_passes() {
        let game_start = r#"{"type":"gameStart","game":{"fullId":"flQMdb6ThTNq",
        "gameId":"flQMdb6T","fen":"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "color":"black","lastMove":"","source":"friend","variant":{"key":"standard","name":"Standard"},
        "speed":"correspondence","perf":"correspondence","rated":false,"hasMoved":false,
        "opponent":{"id":"materoy-bryght","username":"materoy-bryght","rating":1500},
        "isMyTurn":false,"compat":{"bot":false,"board":true},"id":"flQMdb6T"}}"#;

        let result = Event::from_json_str(game_start).unwrap();
        assert!(result.is_ok());
        if let Ok(event) = result {
            assert_eq!(event.r#type, EventType::GameStart);
            assert!(event.game.is_some());
        }
    }

    #[test]
    fn event_game_finish_deserialization_passes() {
        let game_finish = r#"{"type":"gameFinish","game":{"fullId":"flQMdb6ThTNq","gameId":"flQMdb6T",
        "fen":"rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
        "color":"black","lastMove":"g1f3","source":"friend","variant":{"key":"standard","name":"Standard"},
        "speed":"correspondence","perf":"correspondence","rated":false,"hasMoved":true,
        "opponent":{"id":"materoy-bryght","username":"materoy-bryght","rating":1500},
        "isMyTurn":false,"compat":{"bot":false,"board":true},"id":"flQMdb6T"}}"#;

        let result = Event::from_json_str(game_finish).unwrap();
        assert!(result.is_ok());
        if let Ok(event) = result {
            assert_eq!(event.r#type, EventType::GameFinish);
            assert!(event.game.is_some());
        }
    }

    #[test]
    fn event_challenge_declined_deserialization() {
        let challenge_declined = r#"{"type":"challengeDeclined",
        "challenge":{"id":"X4aQIuYQ","url":"https://lichess.org/X4aQIuYQ","status":"declined",
        "challenger":{"id":"materoy","name":"materoy","title":null,"rating":1500,"provisional":true,"online":true},
        "destUser":{"id":"materoy-bryght","name":"materoy-bryght","title":null,"rating":1500,"provisional":true,"online":true},
        "variant":{"key":"standard","name":"Standard","short":"Std"},"rated":false,"speed":"correspondence",
        "timeControl":{"type":"unlimited"},"color":"random","finalColor":"white",
        "perf":{"icon":"","name":"Correspondence"},"declineReason":"I'm not accepting challenges at the moment."}}"#;

        let result = Event::from_json_str(challenge_declined).unwrap();
        assert!(result.is_ok());
        if let Ok(event) = result {
            assert_eq!(event.r#type, EventType::ChallengeDeclined);
            assert!(event.challenge.is_some());
        }
    }
}
