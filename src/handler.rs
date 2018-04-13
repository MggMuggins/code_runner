use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

pub struct CodeRunnerHandler;

impl EventHandler for CodeRunnerHandler {
    fn message(&self, _: Context, msg: Message) {
        println!("{:?}", msg);
    }
}
