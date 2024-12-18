use serenity::all::{Context, GuildId};
use serenity::model::permissions::Permissions;

#[allow(unused)]
pub async fn bot_permissions(
    ctx: &Context,
    guild_id: &GuildId,
) ->  Permissions {
    let bot_user = ctx.cache.current_user();
    let permissions = guild_id
        .member(&ctx.http, bot_user.id)
        .await
        .unwrap()
        .permissions
        .unwrap();
    return permissions;
}

#[allow(unused)]
pub async fn bot_permission_checker(
    guild_id: &GuildId,
    ctx: &Context,
    permission: Permissions
) -> bool {
    let permissions = bot_permissions(ctx, guild_id).await;
    return permissions.contains(permission)
}

#[allow(unused)]
pub async fn bot_permission_checker_manage_roles(
    guild_id: &GuildId,
    ctx: &Context
) -> bool {
    let permission = Permissions::MANAGE_ROLES;
    return bot_permission_checker(guild_id, ctx, permission).await
}

#[allow(unused)]
pub async fn bot_permission_add_manage_role_enter(
    guild_id: &GuildId,
    ctx: &Context
) {
    let bot_user_id = ctx.cache.current_user().id;
    let mut bot_member = guild_id
        .member(&ctx.http, bot_user_id)
        .await
        .unwrap();
    bot_member.permissions.insert(Permissions::MANAGE_ROLES);
}