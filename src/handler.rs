use std::ffi::OsStr;
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output};

use failure::Error;
use serenity::model::channel::Message;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::id::ChannelId;
use serenity::prelude::{Context, EventHandler};

use DOCKER_DIR;

pub struct CodeRunnerHandler {
    bot_id: u64
}

impl CodeRunnerHandler {
    pub fn new(bot_id: u64) -> CodeRunnerHandler {
        CodeRunnerHandler { bot_id }
    }
    
    fn run_code_from(&self, content: String, channel_id: ChannelId) {
        let code = match Code::new(content) {
            Ok(code) => code,
            Err(errmsg) => {
                let _err = channel_id.say(errmsg);
                return;
            }
        };
        let _err = channel_id.say(code.run());
    }
}

impl EventHandler for CodeRunnerHandler {
    fn message(&self, _: Context, msg: Message) {
        if msg.mentions.iter().any(|user| user.id == self.bot_id) {
            self.run_code_from(msg.content, msg.channel_id);
        }
    }
    
    fn message_update(&self, _: Context, msg: MessageUpdateEvent) {
        let mentions = msg.mentions.unwrap_or(Vec::new());
        let content = msg.content.unwrap_or(String::new());
        
        if mentions.iter().any(|user| user.id == self.bot_id) {
            self.run_code_from(content, msg.channel_id);
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
    fn new(block: String) -> Result<Code, String> {
        let matches: Vec<_> = block.match_indices("```").collect();
        
        if matches.len() != 2 {
            Err("I need exactly 1 markdown code block please!".to_string())
        } else {
            let first = matches[0].0;
            let second = matches[1].0 + 3;
            Ok(Code::from_block(block[first..second].to_string()))
        }
    }
    
    fn from_block(block: String) -> Code {
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
        let language_found = if let Ok(mut dir) = read_dir(DOCKER_DIR) {
            dir.any(|entry| if let Ok(entry) = entry {
                entry.file_name() == OsStr::new(&self.language)
            } else {
                false
            })
        } else {
            false
        };
        
        if !language_found {
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
    
    // The name of the file that contains the code to be run
    let filename: PathBuf = [working_docker_dir, "code"].iter().collect();
    
    // Tag for the docker container
    let tag = "runner_".to_string() + &language_prefix;
    
    // Write the code to the file to be built into the container
    let mut file = File::create(&filename)?;
    file.write(code.as_bytes())?;
    
    let build_output = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(&tag)
        .arg(working_docker_dir)
        .output()?;
    
    if !build_output.status.success() {
        return get_output_string(build_output);
    }
    
    let output = Command::new("docker")
        .arg("run")
        .arg("--memory=300m")
        // Low as we can
        .arg("--cpu-shares=2")
        .arg("--rm")
        .arg(&tag)
        .output()?;
    
    get_output_string(output)
}

fn get_output_string(output: Output) -> Result<String, Error> {
    let mut stdout = String::from_utf8(output.stdout)?;
    escape_graves(&mut stdout);
    let mut stderr = String::from_utf8(output.stderr)?;
    escape_graves(&mut stderr);

    if stdout.trim() == "" {
        Ok("\n```\n".to_owned()
            + &stderr
            + &"\n```")
    } else if stderr.trim() == "" {
        Ok("\n```\n".to_owned()
            + &stdout
            + &"\n```")
    } else {
        Ok("Stdout:```\n".to_owned()
            + &stdout
            + &"\n```\nStderr:```\n".to_owned()
            + &stderr
            + &"\n```")
    }
}

fn escape_graves(text: &mut String) {
    let first = {
        let matches: Vec<_> = text.match_indices("```").collect();
        if matches.len() == 0 {
            return;
        }
        matches[0].0
    };
    
    text.insert(first + 2, '\\');
    escape_graves(text);
}
