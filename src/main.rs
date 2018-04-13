extern crate serenity;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate failure;

mod handler;

use std::process::exit;
use std::fs::File;
use std::io::Read;

use failure::Error;
use serenity::client::Client;

use handler::CodeRunnerHandler;

#[cfg(not(release))]
const TOKEN_JSON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/token.json");
#[cfg(release)]
const TOKEN_JON: &str = "";

fn get_discord_token() -> Result<String, Error> {
    #[derive(Serialize, Deserialize)]
    struct Token {
        token: String
    }
    
    // File read
    let mut json = String::new();
    File::open(TOKEN_JSON)?.read_to_string(&mut json)?;
    
    let token: Token = serde_json::from_str(&json)?;
    Ok(token.token)
}

fn main() {
    let token = get_discord_token().unwrap_or_else(|err| {
        eprintln!("Unavailable Discord token: {:?}", err);
        exit(1)
    });
    println!("Token: {}", token);
    
    let mut client = Client::new(&token, CodeRunnerHandler).unwrap_or_else(|err| {
        eprintln!("Err creating client: {:?}", err);
        exit(1)
    });
    
    client.start().unwrap_or_else(|err| {
        eprintln!("Err running client: {:?}", err);
        exit(1)
    });
}
