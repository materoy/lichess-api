use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub url: String,
    pub color: String,
    pub direction: Option<Direction>,
    #[serde(rename = "timeControl")]
    pub time_control: TimeControl,
    pub variant: Variant,
    pub challenger: Challenger,
    pub dest_user: Option<ChallengeUser>,
    pub perf: Perf,
    pub rated: bool,
    pub speed: Speed,
    pub status: ChallengeStatus,
    #[serde(rename = "initialFen")]
    pub initial_fen: Option<String>,
    #[serde(rename = "declineReason")]
    pub decline_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeControl {
    pub increment: Option<u32>,
    pub limit: Option<u32>,
    pub show: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    pub key: String,
    pub name: String,
    pub short: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Challenger {
    pub rating: i32,
    pub provisional: bool,
    pub patron: Option<bool>,
    pub id: String,
    pub name: String,
    pub online: bool,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeUser {
    pub rating: i32,
    pub provisional: bool,
    pub online: bool,
    pub name: String,
    pub title: Option<String>,
    pub patron: Option<bool>,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Perf {
    pub icon: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "accepted")]
    Accepted,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Speed {
    #[serde(rename = "ultraBullet")]
    UltraBullet,
    #[serde(rename = "bullet")]
    Bullet,
    #[serde(rename = "blitz")]
    Blitz,
    #[serde(rename = "rapid")]
    Rapid,
    #[serde(rename = "classical")]
    Classical,
    #[serde(rename = "correspondence")]
    Correspondence,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

impl Challenge {
    pub fn from_json_str(json: &str) -> Option<Result<Self, serde_json::Error>> {
        #[derive(Serialize, Deserialize)]
        pub struct ChallengeOutput {
            pub challenge: Challenge,
        }

        match utils::json_deserialize::<ChallengeOutput>(json) {
            Some(output) => match output {
                Ok(challenge_output) => Some(Ok(challenge_output.challenge)),
                Err(e) => Some(Err(e)),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialization_passes() {
        let test_json = r#"{"challenge":
         {"id":"oCbyqJYk","url":"https://lichess.org/oCbyqJYk","status":"created",
         "challenger":{"id":"materoy","name":"materoy","title":null,
         "rating":1500,"provisional":true,"online":true},
         "destUser":{"id":"materoy-bryght","name":"materoy-bryght","title":null,"rating":1500,"provisional":true,"online":true},
         "variant":{"key":"standard","name":"Standard","short":"Std"},
         "rated":false,"speed":"correspondence",
         "timeControl":{"type":"unlimited"},"color":"random","finalColor":"black",
         "perf":{"icon":"","name":"Correspondence"},"direction":"out"},"socketVersion":0}"#;

        assert!(Challenge::from_json_str(test_json).is_some());
        assert!(Challenge::from_json_str(test_json).unwrap().is_ok())
    }
}
