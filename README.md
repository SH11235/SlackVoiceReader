# SlackVoiceReader

VoiceVox（ずんだもん等）を使って Slack のスレッドのコメントを読み上げるアプリ

## おおまかな仕様

Slack の監視対象スレッドを指定してアプリを起動する。
1 秒ごとに Slack の該当スレッドをチェックし、新規メッセージがあればその中の最新のものを VoiceVox で読み上げる。
メッセージの読み上げが終わるまでの間は新たに Slack のメッセージ取得を行わない（読み上げ中に複数コメントがあると最新のもの以外はスルーされる）。

## セットアップ

VoiceVox と Slack アプリの準備が必要になる。

1.  VoiceVox（読み上げソフト）

    VoiceVox ダウンロード: https://voicevox.hiroshiba.jp/

2.  Slack アプリ

- アプリ作成

  https://api.slack.com/apps からアプリを作成し、OAuth & Permissions から以下の権限を付与する。

  - channels:history
  - groups:history
  - im:history
  - mpim:history
  - Work Space install & OAuth Token 取得

    権限を設定後ワークスペースにインストールし、OAuth Token を取得する（アプリで使用する）。

## 実行環境

Python のスクリプトは python 以下に、Rust のスクリプトは rust 以下に用意した（仕様はほぼ同じ）。

- Python

  Python3 系をインストールする。
  必要なライブラリは pyaudio。

  ```sh
  pip install pyaudio
  ```

- Rust

  rustup https://rustup.rs/

  使用しているクレートは `rust/Cargo.toml` 参照

## 実行

- VoiceVox を起動する。
- Slack の監視対象のスレッドを作成する（コメントする）
- スレッドのリンクをコピーする

  例: https://hogefoo.slack.com/archives/ChannelId/p1698910038548459

  ChannelId がチャンネル ID

  1698910038.548459 がタイムスタンプ（下 6 桁の前に小数点をつけたもの）

  にそれぞれ対応している

- OAuth Token、スレッド情報をスクリプトに記載する

    - slack_token
    - channel_id
    - thread_ts
    - latest_timestamp(thread_ts と同じ値)

- スクリプトを実行する

    - Python

        ```sh
        cd python
        python slack_voicevox.py
        ```

    - Rust

        ```sh
        cd rust
        cargo run
        ```

  以後、新規メッセージがあるたびに読み上げる。

