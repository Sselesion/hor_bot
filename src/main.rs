pub mod prev;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    prev::init_logging(0);
    info!("Starting command bot...");
    debug!("Some debug");
    warn!("Some warn");
    error!("Some warn");

    let token = std::env::var("TELOXIDE_TOKEN").expect("cannot import token from env");
    let bot = Bot::new(token);
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("You username is {username}."))
                .await?
        }
        Command::UsernameAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is {username} and age is {age}."),
            )
            .await?
        }
    };
    Ok(())
}
