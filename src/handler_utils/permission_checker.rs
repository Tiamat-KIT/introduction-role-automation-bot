use serenity::all::{GuildId, Permissions};

#[allow(unused)]
pub async fn bot_permission_checker(guild_id: &GuildId, ctx: &poise::serenity_prelude::Context) -> Permissions {
    let permissions = guild_id
        .member(
            &ctx.http,
            ctx.cache.current_user()
            .id
        )
        .await
        .unwrap()
        .permissions
        .unwrap();
    return permissions;
}