# Serenity Role Automation Manager Bot

## どういうBot？
Rustでプログラミングした、Discord Botです。
自己紹介を終えたら、自動的にロールを追加してくれます

## 導入手順
ちょっと手順長めになりますが許してください。
1. GitHubアカウントを作成
2. PCにDocker Desktopを導入。
3. Shuttleというプログラムのサーバを無料で立てられるサービスに登録
4. Shuttleのサイトから、API_KEYというものを取得する（これは、SHUTTLE_API_KEY。）
5. このリポジトリをFork
6. Dockerfileから、Dockerを通して仮想環境を起動する
7. 導入してあるShuttle CLIで`shuttle init`をして、それで生成されたファイルのうち、`.shuttle/config.toml`からidだけ保存する
8. 生成されたファイルを全消し（idはPROJECT_IDになるので、絶対に保存しておいてほしい
9. Discord Developer Potalで「New Application」をクリックして、新規作成
10. Botメニューから、「RESET TOKEN」でトークン発行（発行したトークンがDISCORD_TOKEN）
11. 以下のようにINTENTを有効化
    ![image](https://github.com/user-attachments/assets/5c789a9b-8f1e-4fda-ae22-b9d89c5386e1)
12. Discordのグループの設定から、ロールのID（ROLE_ID）、自己紹介のチャンネルのID（INTRODUCTION_CHANNEL_ID）を用意する
13. Forkしたリポジトリの「Setting」から「Security」->「Actions」
14. 「New Repository Secret」で以下の設定をしてほしいところに書いてある5つのデータ（名前と対応する値）を登録
15. Forkしたリポジトリの「Actions」の「Shuttle Deploy」の「Run Workflow」を押す

これで出来ます。
## 設定をしてほしいところ
GitHub Actionsで自動デプロイできるようにしてあります。

- SHUTTLE_API_KEY
- PROJECT_ID
- DISCORD_TOKEN
- INTRODUCTION_CHANNEL_ID
- ROLE_ID

これらをSecretとして設定してください
