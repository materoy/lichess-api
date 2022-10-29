use challenge::Challenge;
use event::Event;
use futures_util::StreamExt;
use reqwest::Client;

mod challenge;
mod event;
mod game;
mod utils;

const BASE_URL: &str = "https://lichess.org";
const TOKEN: &str = "lip_wTkRSjgSw2Vyf7Y9ESDa";

pub type ResultReturn = Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> ResultReturn {
    let client = reqwest::Client::new();

    create_challenge(client).await?;
    Ok(())
}

async fn create_challenge(client: Client) -> ResultReturn {
    let url = format!("{}/api/challenge/materoy-bryght", BASE_URL);
    let response = client.post(url).bearer_auth(TOKEN).send().await?;

    let response_string = response.text().await?;
    if let Some(challenge) = Challenge::from_json_str(&response_string) {
        println!(
            "You are currently challenging: {}",
            challenge?.challenger.name
        );
    }

    stream_event(&client).await?;

    Ok(())
}

async fn stream_event(client: &Client) -> ResultReturn {
    let url = format!("{}/api/stream/event", BASE_URL);
    let response = client.get(url).bearer_auth(TOKEN).send().await?;

    let mut response_stream = response.bytes_stream();
    while let Some(item) = response_stream.next().await {
        match item {
            Ok(output) => {
                match utils::string_from_bytes(&output) {
                    Ok(string_output) => {
                        if let Some(event) = Event::from_json_str(string_output) {
                            let event = event?;
                            match event.r#type {
                                event::EventType::GameStart => {
                                    if let Some(game) = event.game {
                                        stream_game(&client, &game.id).await?;
                                    }
                                }
                                event::EventType::GameFinish => {
                                    // TODO close game stream
                                }
                                event::EventType::Challenge => {}
                                event::EventType::ChallengeCanceled => {}
                                event::EventType::ChallengeDeclined => {}
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

async fn stream_game(client: &Client, game_id: &str) -> ResultReturn {
    let url = format!("{}/api/stream/game/{}", BASE_URL, game_id);
    let response = client.get(url).bearer_auth(TOKEN).send().await?;

    let mut response_stream = response.bytes_stream();
    while let Some(item) = response_stream.next().await {
        if let Ok(game_output) = item {
            let string_output = utils::string_from_bytes(&game_output)?;

            println!("GameStream output: {}", string_output);
        }
    }

    Ok(())
}
