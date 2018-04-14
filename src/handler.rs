use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use failure::Error;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

use DOCKER_DIR;

const VALID_LANGS: [&str; 3] = ["python", "ruby", "javascript"];

pub struct CodeRunnerHandler {
    bot_id: u64
}

impl CodeRunnerHandler {
    pub fn new(bot_id: u64) -> CodeRunnerHandler {
        CodeRunnerHandler { bot_id }
    }
}

impl EventHandler for CodeRunnerHandler {
    fn message(&self, _: Context, msg: Message) {
        if msg.mentions.iter().any(|user| user.id == self.bot_id) {
            
            let matches: Vec<_> = msg.content.match_indices("```").collect();
            
            if matches.len() != 2 {
                msg.channel_id.say("I need exactly 1 markdown code block please!");
            } else {
                let first = matches[0].0;
                let second = matches[1].0 + 3;
                let code = Code::new(msg.content[first..second].to_string());
                
                println!("{:?}", &code);
                msg.channel_id.say(code.run());
            }
        }
    }
}

#[derive(Debug)]
struct Code {
    // This is an un-parsed markdown code block
    // Like this:
    // ```Rust
    //     hi
    // ```
    block: String,
    // The language of this code block
    language: String,
    // The actual code
    code: String
}

impl Code {
    fn new(block: String) -> Code {
        let (firstline_end, _) = block.to_string().match_indices('\n').next().unwrap();
        
        let language = &block[3..firstline_end];
        let language = language.trim().to_lowercase();
        
        let code = &block[firstline_end + 1..block.len() - 4];
        
        Code {
            block: block.to_string(),
            language,
            code: code.to_string()
        }
    }
    
    fn run(self) -> String {
        if !VALID_LANGS.contains(&self.language.as_str()) {
            "Language not found".to_string()
        } else {
            run_docker(self.language, self.code)
                .unwrap_or_else(|err| err.cause().to_string())
        }
    }
}

fn run_docker(language_prefix: String, code: String) -> Result<String, Error> {
    let working_docker_dir: PathBuf = [DOCKER_DIR, &language_prefix].iter().collect();
    let working_docker_dir = working_docker_dir.to_str()
        .unwrap_or_else(|| return "Some path wasn't utf8!");
    
    // The name of the file that contains the code
    let filename: PathBuf = [working_docker_dir, "code"].iter().collect();
    
    // Tag for the docker container
    let tag = language_prefix + "_runner";
    
    // Write the code to the file to be built into the container
    let mut file = File::create(&filename)?;
    file.write(code.as_bytes())?;
    
    let _build_output = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(&tag)
        .arg(working_docker_dir)
        .output()?;
    
    let output = Command::new("docker")
        .arg("run")
        .arg(&tag)
        //.arg("--memory=\"500m\"")
        //.arg("--cpus=\".5\"")
        .output()?;
    
    Ok("Stdout:```\n".to_owned()
        + &String::from_utf8(output.stdout)?
        + &"\n```\nStderr:```\n".to_owned()
        + &String::from_utf8(output.stderr)?
        + &"\n```")
}
