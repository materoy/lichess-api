use std::error::Error;

use challenge::Challenge;
use event::Event;
use futures_util::StreamExt;
use reqwest::Client;

pub mod challenge;
pub mod event;
pub mod game;
pub mod utils;

pub(crate) type ResultReturn = Result<(), Box<dyn std::error::Error>>;

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
    pub async fn create_challenge(&self, user_id: &str) -> Result<Challenge, anyhow::Error> {
        let url = format!("{BASE_URL}/api/challenge/{user_id}");
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .send()
            .await?;

        let response_string = response.text().await?;
        match Challenge::from_json_str(&response_string) {
            Some(challenge) => {
                let challenge = challenge?;
                println!(
                    "You are currently challenging: {}",
                    challenge.challenger.name
                );
                Ok(challenge)
            }
            None => Err(anyhow::anyhow!("")),
        }
    }

    pub async fn cancel_challenge(&self, challenge_id: &str) -> Result<String, anyhow::Error> {
        let url = format!("{BASE_URL}/api/challenge/{challenge_id}/cancel");
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .send()
            .await?;

        let response_string = response.text().await?;

        Ok(response_string)
    }

    pub async fn stream_event<F>(
        &self,
        on_event: F,
    ) -> impl futures_util::Stream<Item = reqwest::Result<Event>>
    where
        F: Fn(&Event),
    {
        let url = format!("{}/api/stream/event", BASE_URL);
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .await
            .unwrap();

        let response_stream = response.bytes_stream();

        response_stream.map(move |response| {
            let response = response.unwrap();
            let string_output = utils::string_from_bytes(&response);
            let event = Event::from_json_str(string_output.unwrap()).unwrap();
            let event = event.unwrap();
            on_event(&event);
            Ok(event)
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_challenge() {
        let api = Lichess::new();
        let challenge = api.create_challenge("materoy-bryght").await.unwrap();
        let response = api.cancel_challenge(&challenge.id).await.unwrap();
        assert_eq!(response, String::from("{\"ok\":true}"))
    }
}
