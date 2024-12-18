# Serenity Role Automation Manager Bot

## どういうBot？
Rustでプログラミングした、Discord Botです。
自己紹介を終えたら、自動的にロールを追加してくれます

## 導入手順
ちょっと手順長めになりますが許してください。
1. GitHubアカウントを作成
2. PCにDocker Desktopを導入。
3. [Shuttle](https://console.shuttle.dev/)というプログラムのサーバを無料で立てられるサービスに登録
4. [Shuttle](https://console.shuttle.dev/)のサイトから、API_KEYというものを取得する（これは、SHUTTLE_API_KEY。）
5. このリポジトリをFork

    これ -> ![image](https://github.com/user-attachments/assets/791192a4-abd7-4222-b4cc-348c5687b588)

6. ~~ForkしたリポジトリをCloneする~~ Codeボタンから「Download ZIP」を選択してZIPファイルをダウンロードして解凍する

    Codeボタンはこれと似たようなやつがどこかにあるぞ！ -> ![image](https://github.com/user-attachments/assets/6fc68289-5e0d-4654-91b2-c009845bfad5)

8. genというフォルダを作っておく（**Forkしたリポジトリの中で作成してください！**）
9. リポジトリにあるDockerfileから、Dockerを通して仮想環境を起動する。
    `docker compose up -d`
10. 導入してあるShuttle CLIで`shuttle init`をして、それで生成されたファイルのうち、**`.shuttle/config.toml`からidだけ保存**する
    ```bash
    docker compose exec rust-shuttle grep -oP 'id="\K[^"]*' /workspace/[9のコマンドで生成されたフォルダ]/.shuttle/config.toml
    ```
11. 生成されたファイルを全消し（idはPROJECT_IDになるので、絶対に保存しておいてほしい）
12. Discord Developer Potalで「New Application」をクリックして、新規作成
13. Botメニューから、「RESET TOKEN」でトークン発行（発行したトークンがDISCORD_TOKEN）
14. Botメニューで、以下のようにINTENTを有効化
    ![image](https://github.com/user-attachments/assets/5c789a9b-8f1e-4fda-ae22-b9d89c5386e1)
15. Discordのグループの設定から、ロールのID（ROLE_ID）、自己紹介のチャンネルのID（INTRODUCTION_CHANNEL_ID）を用意する
16. Forkしたリポジトリの「Setting」から「Security」->「Actions」
17. 「New Repository Secret」で以下の設定をしてほしいところに書いてある5つのデータ（名前と対応する値）を登録
18. Forkしたリポジトリの「Actions」の「Shuttle Deploy」の「Run Workflow」を押す

これで出来ます。
## 設定をしてほしいところ
GitHub Actionsで自動デプロイできるようにしてあります。

- SHUTTLE_API_KEY
- PROJECT_ID
- DISCORD_TOKEN
- INTRODUCTION_CHANNEL_ID
- ROLE_ID

これらをSecretとして設定してください
