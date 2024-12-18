# Serenity Role Automation Manager Bot

## どういうBot？
Rustでプログラミングした、Discord Botです。
自己紹介を終えたら、自動的にロールを追加してくれます

## 導入手順
ちょっと手順長めになりますが許してください。
GitHubアカウントを作成した上で、PCに
Docker Desktopというのを導入してください。
加えて、Shuttleというプログラムのサーバを無料で立てられる
サービスにも登録する必要があります。





## 設定をしてほしいところ
GitHub Actionsで自動デプロイできるようにしてあります。

- SHUTTLE_API_KEY
- DISCORD_TOKEN
- INTRODUCTION_CHANNEL_ID
- PROJECT_ID
- ROLE_ID

これらをSecretとして設定してください
