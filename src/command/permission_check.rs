use poise::command;
use crate::{Context,PoiseError};

#[allow(unused)]
/// 権限を確認するコマンド
/// 
/// このコマンドは、Botの権限を確認するコマンドです。
/// これの結果を見て、Botの権限が足りない場合は、権限を追加してください。
#[command(
    slash_command,
    required_bot_permissions = "MANAGE_ROLES",
    guild_only
)]
pub async fn permission_check(ctx: Context<'_>) -> Result<(), PoiseError> {
    // 権限の確認
    ctx.say("権限の確認を行います").await?;
    let guild_id = ctx.guild_id().expect("Guild ID was not found");
    let bot_id = ctx.cache().current_user().id;
    match guild_id.member(&ctx.http(), bot_id).await {
        Ok(member) => {
            match member.permissions {
                Some(permission) => {
                    if permission.manage_roles() {
                        ctx.say("権限があります").await?;
                    } else if permission.administrator() {
                        ctx.say("管理者権限があります").await?;
                    }
                    else {
                        ctx.say("ロール操作権限がありません").await?;
                    }
                },
                None => {
                    ctx.say("権限が全く与えられていません。").await?;
                },
            }
        }
        Err(err) => {
            ctx.say(format!("メンバーが見つかりません {}",err)).await?;
        }
    }
    Ok(())
}