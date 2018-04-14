extern crate serenity;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate failure;

mod handler;
mod language;

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

#[derive(Serialize, Deserialize)]
struct JsonInfo {
    token: String,
    bot_id: u64
}

fn get_json_info() -> Result<JsonInfo, Error> {
    // File read
    let mut json = String::new();
    File::open(TOKEN_JSON)?.read_to_string(&mut json)?;
    
    let info: JsonInfo = serde_json::from_str(&json)?;
    Ok(info)
}

fn main() {
    let token = get_json_info().unwrap_or_else(|err| {
        eprintln!("Unavailable Discord token: {:?}", err);
        exit(1)
    });
    
    let mut client = Client::new(&token.token, CodeRunnerHandler::new(token.bot_id))
        .unwrap_or_else(|err| {
            eprintln!("Err creating client: {:?}", err);
            exit(1)
        });
    
    client.start().unwrap_or_else(|err| {
        eprintln!("Err running client: {:?}", err);
        exit(1)
    });
}
