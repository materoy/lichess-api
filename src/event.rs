use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Challenge {
    id: String,
    url: String,
    status: ChallengeStatus,
}

#[derive(Debug, Serialize, Deserialize)]
enum ChallengeStatus {
    created,
}
