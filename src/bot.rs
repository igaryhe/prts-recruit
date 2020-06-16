use std::env;
use telegram_types::bot::methods::{Method, TelegramResult};

#[derive(Clone, Debug)]
pub struct Bot {
    pub token: String,
    pub url: String
}

impl Bot {
    pub fn new() -> Self {
        let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let url = env::var("TELEGRAM_BOT_URL").unwrap() + &token;
        Bot {
            token,
            url
        }
    }

    pub async fn call<T: Method>(&self, params: T) -> surf::Result<T::Item> {
        let url = T::url(&self.token);
        let mut res = surf::post(url).body_json(&params)?.await?;
        let TelegramResult {result, ..} = res.body_json().await?;
        Ok(result.unwrap())
    }
}