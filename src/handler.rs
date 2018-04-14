use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

use language::{Language, Python};

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
                let code = Code::new(&msg.content[first..second]);
                
                println!("{:?}", &code);
                msg.channel_id.say(code.block + &code.language);
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
    fn new(block: &str) -> Code {
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
    
    fn get_language<L>(&self) -> Option<Language> {
      //where L: Language {
        match self.language.as_str() {
            "python" => Some(Python::new(self.code)),
            _ => None
        }
    }
}
