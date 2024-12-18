use serenity::all::{ChannelId, Message};

pub async fn intro_text_checker(msg: &Message,channel_id: &ChannelId) -> bool {
    // ユーザーが特定のチャンネルにメッセージを送信した場合のテキストチェックの結果を送る
    let is_msg_to_introduction_channel = msg.channel_id == *channel_id;
    let is_twitter_link = msg.content.contains("x.com");
    let iriam_link = msg.content.contains("iriam.app");
    return is_msg_to_introduction_channel && is_twitter_link && iriam_link;
}