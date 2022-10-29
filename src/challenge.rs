use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub url: String,
    pub color: String,
    pub direction: String,
    #[serde(rename = "timeControl")]
    pub time_control: TimeControl,
    pub variant: Variant,
    pub challenger: Challenger,
    pub dest_user: Option<ChallengeUser>,
    pub perf: Perf,
    pub rated: bool,
    pub speed: Speed,
    pub status: Status,
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
pub enum Status {
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

impl Challenge {
    pub fn from_json_str(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        #[derive(Serialize, Deserialize)]
        pub struct ChallengeOutput {
            pub challenge: Challenge,
        }

        Ok(serde_json::from_str::<ChallengeOutput>(json)?.challenge)
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
        Challenge::from_json_str(test_json).unwrap();
    }
}
