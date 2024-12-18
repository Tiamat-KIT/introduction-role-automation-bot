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

6. ~~ForkしたリポジトリをCloneする~~ Forkしたリポジトリの内容をCodeボタンから「Download ZIP」を選択してZIPファイルをダウンロードして解凍する

    Codeボタンはこれと似たようなやつがどこかにあるぞ！ -> ![image](https://github.com/user-attachments/assets/6fc68289-5e0d-4654-91b2-c009845bfad5)

8. genというフォルダを作っておく（**Forkしたリポジトリの中で作成してください！**）
9. リポジトリにあるDockerfileから、Dockerを通して仮想環境を起動する。
    `docker compose up -d`をコマンドプロンプトで実行
10. 導入してあるShuttle CLIで`shuttle init`をして、それで生成されたファイルのうち、**`.shuttle/config.toml`からidだけ保存**する
    実行するコマンドはコレ
    ```bash
    # これで起動できるはず
    docker compose up -d
    # コンテナ内のシェル（コンテナ内のOSはLinuxなので、コマンドプロンプトみたいな別もの）に入る
    docker compose exec rust-shuttle bash
    # これでshuttleのproject_idが生成されるはず
    shuttle init
    # ディレクトリ移動する
    cd [上のコマンドで生成されたフォルダ]/.shuttle
    # idだけ取得
    cat config.toml | grep -Po '(?<=id = ).*' | sed 's/"//g'
    # 元の位置に戻って、生成されたファイルを全消し（idはPROJECT_IDになるので、絶対に保存しておいてほしい）
    cd ../.. && rm -rf [上のコマンドで生成されたフォルダ]
    # シェルから出る
    exit
    ```
    をコマンドプロンプトで実行
    
    ![image](https://github.com/user-attachments/assets/b089c91b-257a-4c3f-9132-942b2f4d01e6)

12. Discord Developer Potalで「New Application」をクリックして、新規作成
13. Botメニューから、「RESET TOKEN」でトークン発行（発行したトークンがDISCORD_TOKEN）
14. Botメニューで、以下のようにINTENTを有効化
    ![image](https://github.com/user-attachments/assets/5c789a9b-8f1e-4fda-ae22-b9d89c5386e1)
15. Discordのグループの設定から、ロールのID（ROLE_ID）、自己紹介のチャンネルのID（INTRODUCTION_CHANNEL_ID）を用意する
16. Forkしたリポジトリの「Setting」から「Security」->「Actions」
17. 「New Repository Secret」で以下の設定をしてほしいところに書いてある5つのデータ（名前と対応する値）を登録
18. Forkしたリポジトリの「Actions」の「Shuttle Deploy」の「Run Workflow」を押す
19. Discord Developer Potalで、OAuth2 URL Generatorで以下の設定をする
    ![image](https://github.com/user-attachments/assets/d355d51c-5dfa-4c6e-8298-81febd2cdd68)
    ![image](https://github.com/user-attachments/assets/bde88ae3-ff38-4bff-8631-be1d6129bab0)
20. 生成されたURLを使ってBotを招待する
21. Discordのグループサーバの設定からBotのロールが、自動的に設定したいロールの下にくるようにする
    ![image](https://github.com/user-attachments/assets/ee5b7ab6-0237-4f59-b870-1984b807a90a)


これで出来ます。
## 設定をしてほしいところ
GitHub Actionsで自動デプロイできるようにしてあります。

- SHUTTLE_API_KEY
- PROJECT_ID
- DISCORD_TOKEN
- INTRODUCTION_CHANNEL_ID
- ROLE_ID

これらをSecretとして設定してください

### コマンドプロンプトってどれ？
これ。
![image](https://github.com/user-attachments/assets/56b52522-36d1-4ef5-9e6f-3a8636bf9bd0)

もしくは
![image](https://github.com/user-attachments/assets/bc287e8a-97d3-4af8-b07b-573e1edcbabc)
cmdって書いて実行

## Docker周りでつまづいたら

```bash
# コンテナを削除
docker-compose rm rust-shuttle

# ちゃんと指定したコンテナがなくなっているか確認
docker-compose ps -a　

# 個別のコンテナを作り直し
docker-compose build --no-cache rust-shuttle

# 動作確認
docker-compose up -d
```
