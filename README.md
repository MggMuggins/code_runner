# code_runner
This is a little discord bot to run random code snippets from the cat in a sensible and safe way.

# `token.json`
This repository requires a token.json (located in res/token.json). The file should look something like this:
```Json
{
    "token": "BOT_TOKEN",
    "bot_id": 1
}
```
`bot_id` is the actual discord user id of the bot, that way he can know when somebody mentions him.

# Docker
This bot uses docker on the backend to run commands. These docker images are minimal, and all should be based on Alpine Linux. For now, observe what is done in the source in terms of how to set up a new language. If somebody has more experience with docker and knows that I'm doing something dumb, please open an issue!
