use serde::{Deserialize, Serialize};

use crate::challenge::Challenge;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub r#type: EventType,
    #[serde(rename = "gameStart")]
    pub game_start: Option<GameEventInfo>,
    #[serde(rename = "gameFinish")]
    pub game_finish: Option<GameEventInfo>,
    pub challenge: Option<Challenge>,
    #[serde(rename = "challengeCanceled")]
    pub challenge_canceled: Option<Challenge>,
    #[serde(rename = "challengeDeclined")]
    pub challenge_declined: Option<Challenge>,
}

#[derive(Debug, Serialize, Deserialize)]
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
