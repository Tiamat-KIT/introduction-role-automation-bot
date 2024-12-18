use serenity::all::{GuildId, RoleId, UserId};

pub async fn member_role_checker(
    ctx: &poise::serenity_prelude::Context,
    guild_id: &GuildId,
    role_id: &RoleId,
    user_id: &UserId
) -> bool{
    // ユーザーが特定のロールを持っているかどうかをチェック
    let member = guild_id.member(&ctx.http, user_id).await.unwrap();
    let has_role = member.roles.contains(&role_id);
    return has_role;
}