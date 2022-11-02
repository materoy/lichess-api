use futures_util::StreamExt;
use reqwest::Client;

use crate::{
    challenge::Challenge,
    event::{Event, EventType},
    utils, ResultReturn,
};

pub(crate) const BASE_URL: &str = "https://lichess.org";
pub(crate) const TOKEN: &str = "lip_wTkRSjgSw2Vyf7Y9ESDa";

pub struct Lichess {
    client: Client,
}

impl Lichess {
    pub fn new() -> Self {
        Lichess {
            client: reqwest::Client::new(),
        }
    }
    pub async fn create_challenge(&self) -> ResultReturn {
        let url = format!("{}/api/challenge/materoy-bryght", BASE_URL);
        let response = self.client.post(url).bearer_auth(TOKEN).send().await?;

        let response_string = response.text().await?;
        if let Some(challenge) = Challenge::from_json_str(&response_string) {
            println!(
                "You are currently challenging: {}",
                challenge?.challenger.name
            );
        }

        self.stream_event().await?;

        Ok(())
    }

    pub async fn stream_event(&self) -> ResultReturn {
        let url = format!("{}/api/stream/event", BASE_URL);
        let response = self.client.get(url).bearer_auth(TOKEN).send().await?;

        let mut response_stream = response.bytes_stream();
        while let Some(item) = response_stream.next().await {
            match item {
                Ok(output) => {
                    match utils::string_from_bytes(&output) {
                        Ok(string_output) => {
                            if let Some(event) = Event::from_json_str(string_output) {
                                let event = event?;
                                match event.r#type {
                                    EventType::GameStart => {
                                        if let Some(game) = event.game {
                                            self.stream_game(&game.id).await?;
                                        }
                                    }
                                    EventType::GameFinish => {
                                        // TODO close game stream
                                    }
                                    EventType::Challenge => {}
                                    EventType::ChallengeCanceled => {}
                                    EventType::ChallengeDeclined => {}
                                }
                            }
                        }
                        Err(e) => eprintln!("UTF-8 parsing error: {}", e),
                    }
                }
                Err(err) => eprintln!("Stream error ocurred: {}", err),
            }
        }

        Ok(())
    }

    pub async fn stream_game(&self, game_id: &str) -> ResultReturn {
        let url = format!("{}/api/stream/game/{}", BASE_URL, game_id);
        let response = self.client.get(url).bearer_auth(TOKEN).send().await?;

        let mut response_stream = response.bytes_stream();
        while let Some(item) = response_stream.next().await {
            if let Ok(game_output) = item {
                let string_output = utils::string_from_bytes(&game_output)?;

                println!("GameStream output: {}", string_output);
            }
        }

        Ok(())
    }
}
