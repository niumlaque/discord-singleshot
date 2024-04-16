use super::{Result, StringSplitter};
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use std::sync::Arc;

pub struct SendOnce;

impl SendOnce {
    pub async fn send(
        token: impl AsRef<str>,
        channel_id: u64,
        data: impl StringSplitter,
    ) -> Result<()> {
        let client = Client::builder(token, GatewayIntents::GUILD_MESSAGES).await?;
        let http = Arc::clone(&client.http);
        let channel = ChannelId::new(channel_id);

        for text in data.get() {
            channel.say(&http, text).await?;
        }

        Ok(())
    }
}
