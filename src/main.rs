use prts_recruit::{process, get_ops, format_result, bot::Bot};
use serde_json;
use tide::Request;
use telegram_types::bot::methods::{DeleteWebhook, SetWebhook, ChatTarget, SendMessage};
use telegram_types::bot::types::{Update, UpdateContent};

#[async_std::main]
async fn main() -> surf::Result<()> {
    let bot = Bot::new();
    bot.call(DeleteWebhook).await?;
    bot.call(SetWebhook::new(&bot.url).max_connections(100)).await?;
    let mut app = tide::new();
    app.at(&bot.token).post(|mut req: Request<()>| async move {
        let Update {content, ..} = req.body_json().await.unwrap();
        match content {
            UpdateContent::Message(msg) => {
                let bot = Bot::new();
                let target = ChatTarget::Id(msg.chat.id.clone());
                let operators = get_ops();
                let tag_list = serde_json::from_str(msg.text.unwrap().as_str());
                let output = match tag_list {
                    Ok(tags) => {
                        let result = process(tags, &operators);
                        format_result(result, &operators)
                    },
                    Err(_) => {
                        format!("Wrong input format!")
                    },
                };
                bot.call(SendMessage::new(target, output)).await.unwrap();
            },
            _ => ()
        }
        Ok("")
    });
    app.listen("127.0.0.1:9561").await?;
    Ok(())
}