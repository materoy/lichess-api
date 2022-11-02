use futures_util::{Stream, StreamExt};
use reqwest::Client;

use crate::{
    challenge::Challenge,
    event::{Event, EventType},
    utils, ResultReturn,
};

pub(crate) const BASE_URL: &str = "https://lichess.org";

pub struct Lichess {
    client: Client,
    token: String,
}

impl Lichess {
    pub fn new() -> Self {
        Lichess {
            client: reqwest::Client::new(),
            token: utils::read_token().unwrap(),
        }
    }
    pub async fn create_challenge(&self) -> ResultReturn {
        let url = format!("{}/api/challenge/materoy-bryght", BASE_URL);
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .send()
            .await?;

        let response_string = response.text().await?;
        if let Some(challenge) = Challenge::from_json_str(&response_string) {
            println!(
                "You are currently challenging: {}",
                challenge?.challenger.name
            );
        }

        self.stream_event(|event| {}).await?;

        Ok(())
    }

    pub async fn stream_event<F>(&self, on_event: F) -> ResultReturn
    where
        F: Fn(&Event),
    {
        let url = format!("{}/api/stream/event", BASE_URL);
        let response = self.client.get(url).bearer_auth(&self.token).send().await?;

        let mut response_stream = response.bytes_stream();
        while let Some(item) = response_stream.next().await {
            match item {
                Ok(output) => {
                    match utils::string_from_bytes(&output) {
                        Ok(string_output) => {
                            if let Some(event) = Event::from_json_str(string_output) {
                                let event = event?;
                                on_event(&event);
                                match event.r#type {
                                    EventType::GameStart => {
                                        if let Some(game) = event.game {
                                            self.stream_game(&game.id, |game| {}).await?;
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

    pub async fn stream_game<F>(&self, game_id: &str, on_event: F) -> ResultReturn
    where
        F: Fn(&str),
    {
        let url = format!("{}/api/stream/game/{}", BASE_URL, game_id);
        let response = self.client.get(url).bearer_auth(&self.token).send().await?;

        let mut response_stream = response.bytes_stream();
        while let Some(item) = response_stream.next().await {
            if let Ok(game_output) = item {
                let string_output = utils::string_from_bytes(&game_output)?;

                println!("GameStream output: {}", string_output);
                on_event(string_output);
            }
        }

        Ok(())
    }
}