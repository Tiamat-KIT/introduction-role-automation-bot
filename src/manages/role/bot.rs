use serenity::all::{GuildId, RoleId};

#[allow(unused)]
pub async fn bot_role_checker(
    ctx: &poise::serenity_prelude::Context,
    guild_id: &GuildId,
    role_id: &RoleId
) -> bool{
    let bot_user = ctx
        .cache
        .current_user();

    let member = guild_id
        .member(&ctx.http, bot_user.id)
        .await
        .unwrap();

    let has_role = member.roles.contains(role_id);
    return has_role;
}