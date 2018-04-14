use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

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
                let code = &msg.content[first..second];
            
                msg.channel_id.say(code);
            }
        }
    }
}
