use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub url: String,
    pub color: String,
    pub direction: String,
    pub time_control: TimeControl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeControl {}

impl Challenge {
    pub fn from_json_str(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        #[derive(Serialize, Deserialize)]
        pub struct ChallengeOutput {
            pub challenge: Challenge,
        }

        Ok(serde_json::from_str::<ChallengeOutput>(json)?.challenge)
    }
}
