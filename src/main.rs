use prts_recruit::{process, get_ops, format_result, parse_string, bot::Bot, operator::Tag};
use tide::Request;
use telegram_types::bot::methods::{DeleteWebhook, SetWebhook, ChatTarget, SendMessage};
use telegram_types::bot::types::{Update, UpdateContent, ParseMode};

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
                let text = msg.text.unwrap();
                let operators = get_ops();
                let tag_list: Vec<Tag> = parse_string(text);
                let output = {
                        let result = process(tag_list, &operators);
                        format_result(result, &operators)
                };
                bot.call(SendMessage::new(target, output)
                         .parse_mode(ParseMode::Markdown)).await.unwrap();
            },
            _ => ()
        }
        Ok("")
    });
    app.listen("127.0.0.1:9561").await?;
    Ok(())
}
