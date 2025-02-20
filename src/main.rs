use std::fs;
use std::path::PathBuf;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Game {
    id: String,
    name: String,
    url: String,
}

fn main() {
    let games = fetch_games();
    for game in games {
        let game_data = download_game(&game);
        save_game(&game, game_data);
    }
}

fn fetch_games() -> Vec<Game> {
    let client = Client::new();
    let response: Vec<Game> = client.get("https://api.roblox.com/games")
        .send()
        .unwrap()
        .json()
        .unwrap();
    response
}

fn download_game(game: &Game) -> Vec<u8> {
    let client = Client::new();
    let response = client.get(&game.url)
        .send()
        .unwrap();
    response.bytes().unwrap().to_vec()
}

fn save_game(game: &Game, data: Vec<u8>) {
    let path = PathBuf::from(format!("{}.rbxl", game.name));
    fs::write(path, data).unwrap();
}