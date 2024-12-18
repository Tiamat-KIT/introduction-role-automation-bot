mod handler_utils;
mod manages;
mod command;

#[allow(unused)]
use anyhow::Context as _;
// use command::permission_check::permission_check;
use handler_utils::text_checker::intro_text_checker;

use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::all::{ChannelId, EventHandler, RoleId,Message};
use serenity::async_trait;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use tracing::{info,error};
use crate::manages::role::member::member_role_checker;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::prelude::Context, msg: Message) {
        if ctx.cache.current_user().id == msg.author.id {
            return;
        }

        info!("Message: {:?}", msg.content);
        let is_check_ok = intro_text_checker(
            &msg,
            &ChannelId::new(std::env::var("INTRODUCTION_CHANNEL_ID")
                .expect("INTRODUCTION_CHANNEL_ID was not found").parse::<u64>().expect("INTRODUCTION_CHANNEL_ID was not found"))
            ).await;
        // ここまでは正常らしい。ここから先の値でエラー。
        info!("is_check_ok: {}", is_check_ok);
        if is_check_ok {
            let role_id = RoleId::new(std::env::var("ROLE_ID")
                .expect("ROLE_ID was not found").parse::<u64>().expect("ROLE_ID was not found"));
            
            //　特定チャンネルにメッセージを送信した後の処理
            // ユーザーにロール追加
            let guild_id = msg.guild_id.unwrap();
            let user_id = msg.author.id;
            if member_role_checker(
                &ctx,
                &guild_id,
                &role_id, 
                &user_id
            ).await {
                if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http, 
                    "既にロールが付与されています"
                ).await {
                    error!("Could not send message: {:?}", why);
                }
                return;
            } else {
                if let Ok(member) = guild_id.member(&ctx.http, user_id).await {
                    match member
                        .add_role(
                            &ctx
                            .http,
                            role_id
                        ).await {
                        Ok(_) => {
                            if let Err(why) = msg
                                .channel_id
                                .say(
                                    ctx
                                    .http
                                    .clone(),
                                    "ロールを付与しました"
                                ).await {
                                info!("Could not send message: {:?}", why);
                            }
                        }
                        Err(why) => {
                            error!("Could not add role: {:?}", why);
                            if let Err(why) = msg
                                .channel_id
                                .say(
                                    ctx
                                    .http
                                    .clone(),
                                    format!("ロールの付与に失敗しました\n管理者にお問い合わせください")
                                ).await {
                                error!("Could not add role: {:?}", why);
                            }
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: serenity::prelude::Context, _ready: serenity::model::gateway::Ready) {
        info!("Bot is ready");
        let bot = ctx.cache.current_user().id;
        info!("Bot user id: {}", bot);
    }
}

#[derive(Debug)]
pub struct Data {
    // ロールは、ユーザーがサーバー内で持つ役割を示す。
    // これは、持っている場合も持っていない場合もある。
    // また、複数所持していることもある
    // role: Arc<Mutex<HashSet<Role>>>,
}

pub type PoiseError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, PoiseError>;



fn secrets_to_env(secret_store: &SecretStore) {
    info!("Setting secrets to environment variables");
    std::env::set_var(
        "DISCORD_TOKEN",
        secret_store.get("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN was not found")
    );
    std::env::set_var(
        "INTRODUCTION_CHANNEL_ID", 
        secret_store.get("INTRODUCTION_CHANNEL_ID")
        .expect("INTRODUCTION_CHANNEL_ID was not found")
    );
    std::env::set_var(
        "ROLE_ID", 
        secret_store.get("ROLE_ID")
        .expect("ROLE_ID was not found")
    );
    std::env::set_var(
        "RUST_BACKTRACE", 
        1.to_string()
    );
    info!("Secrets have been set to environment variables")
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {

    secrets_to_env(&secret_store);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // commands: vec![permission_check()],
            pre_command: |ctx| {
                Box::pin(async move {
                    let command_name = &ctx.command().name;
                    let command_required_permissions = &ctx.command().required_bot_permissions;
                    info!("Command_Name: {:?}\nCommand_Require_Bot_Permission: {:?}", command_name,command_required_permissions);
                    let guild = ctx.guild_id().unwrap();
                    let guild_channel_name = ctx.guild_channel().await.unwrap().name;
                    info!("Guild ID: {} \nGuild_Channel_Name: {}",guild, guild_channel_name);
                })
            },
            on_error: |error| Box::pin(async move {
                match error {
                    poise::FrameworkError::Setup { error, .. } => {
                        error!("Setup Error occurred: {:?}", error);
                    },
                    poise::FrameworkError::EventHandler { error,.. } => {
                        error!("EventHandler Error occurred: {:?}", error);
                    },
                    poise::FrameworkError::Command { error, .. } => {
                        error!("Command Error occurred: {:?}", error);
                    },
                    poise::FrameworkError::CommandPanic { payload, .. } => {
                        error!("Command Panic Error occurred: {:?}", payload.unwrap());
                    },
                    poise::FrameworkError::MissingBotPermissions { missing_permissions, .. } => {
                        error!("MissingBotPermissions Error occurred: {:?}", missing_permissions);
                    },
                    poise::FrameworkError::MissingUserPermissions { missing_permissions, .. } => {
                        error!("MissingUserPermissions Error occurred: {:?}", missing_permissions.unwrap());
                    },
                    poise::FrameworkError::UnknownInteraction { 
                        interaction, .. 
                    } => {
                        error!("UnknownInteraction Error occurred: {:?}", interaction);
                    }
                    _ => {
                        error!("An error occurred: {:?}", error);
                    }
                }
                // error!("An error occurred: {:?}", error::Error::source(error));
            }),
            ..Default::default()
        })
        .setup(|ctx, ready, framework: &poise::Framework<Data, PoiseError>| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let bot_user = ctx.cache.current_user().id;
                for guild in ready.guilds.iter() {
                    if let Ok(member) = guild.id.member(&ctx, bot_user).await {
                        // 権限をデバッグ出力
                        info!("Bot permissions: {:?}", member.permissions);
                    }
                }
                
                Ok(Data {})
            })
        })
        .build();

    // メッセージの読み取り、メンバーの読み取りの権限を持つ
    // メンバーの追加、ロールの追加、ロールの削除の権限を持つ　

    let intents = GatewayIntents::privileged()
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::GUILD_MEMBERS
    | GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::GUILDS;
    
    let client = ClientBuilder::new(
            &std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN was not found"),
            intents
        ) 
        .framework(framework)
        .event_handler(Handler)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    Ok(client.into())
}    

