use challenge::Challenge;
use futures_util::StreamExt;
use reqwest::Client;

mod challenge;
mod event;
mod game;

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
    println!("{}", response_string);
    let challenge = Challenge::from_json_str(&response_string)?;
    println!("Challenge output: {}", challenge.id);

    // stream_event(client).await?;

    Ok(())
}

async fn stream_event(client: Client) -> ResultReturn {
    let url = format!("{}/api/stream/event", BASE_URL);
    let response = client.get(url).bearer_auth(TOKEN).send().await?;

    let mut response_stream = response.bytes_stream();
    while let Some(item) = response_stream.next().await {
        match item {
            Ok(output) => {
                let string = std::str::from_utf8(&output)?;
                println!("Event stream output: {}", string)
            }
            Err(err) => eprintln!("Error ocurred: {}", err),
        }
    }

    Ok(())
}

async fn stream_game(client: Client, game_id: &str) -> ResultReturn {
    let url = format!("{}/api/stream/event", BASE_URL);
    let response = client.get(url).bearer_auth(TOKEN).send().await?;

    let mut response_stream = response.bytes_stream();
    while let Some(item) = response_stream.next().await {
        println!("{:?}", item?);
    }

    Ok(())
}
