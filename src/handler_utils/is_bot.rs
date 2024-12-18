use serenity::all::{Context, Message};

#[allow(unused)]
pub fn is_bot(
    msg: &Message
) -> bool{
    return msg.author.bot;
}

#[allow(unused)]
pub fn is_msg_owner_bot(
    ctx: &Context,
    msg: &Message
) -> bool{
    return msg.author.id == ctx.cache.current_user().id;
}