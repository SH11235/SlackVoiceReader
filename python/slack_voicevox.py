import os
import io
import time
import wave
import pyaudio
import requests
from dotenv import load_dotenv

# .env ファイルの読み込み
load_dotenv()

# Slack APIの設定
slack_token = os.getenv("SLACK_TOKEN")
channel_id = os.getenv("CHANNEL_ID")
thread_ts = os.getenv("THREAD_TS")  # 実行前にスレッドのタイムスタンプを確認しておく
latest_timestamp = os.getenv("THREAD_TS")  # 初期値はthread_tsと同じ

# VoiceVox APIのエンドポイント設定
voicevox_url = os.getenv("VOICEVOX_URL")
synthesize_endpoint = f"{voicevox_url}/audio_query"
audio_endpoint = f"{voicevox_url}/synthesis"

# PyAudioの初期化
p = pyaudio.PyAudio()

# 利用可能なオーディオデバイスの一覧を表示
info = p.get_host_api_info_by_index(0)
num_devices = info.get('deviceCount')
for i in range(0, num_devices):
    if (p.get_device_info_by_host_api_device_index(0, i).get('maxOutputChannels')) > 0:
        print("Output Device id ", i, " - ", p.get_device_info_by_host_api_device_index(0, i).get('name'))

# ユーザーにオーディオデバイスのIDを入力させる
output_device_id = int(input("Enter the output device id: "))

# Slackのスレッドを監視するループ
while True:
    start_time = time.time()
    # Slack APIを使って最新のメッセージを取得
    headers = {
        "Authorization": f"Bearer {slack_token}"
    }
    response = requests.get("https://slack.com/api/conversations.replies",
                            headers=headers,
                            params={"channel": channel_id, "ts": thread_ts})
    # for debug
    # response.json() をファイルにはきだす
    # with open('response.json', "w", encoding="utf-8") as f:
    #     f.write(str(response.json()))
    end_time = time.time()
    print("Slack API request time:", end_time - start_time, "seconds")
    messages = response.json()["messages"]
    # 新しいメッセージがあるかチェック
    if messages and messages[-1]["ts"] != latest_timestamp:
        # 最新のメッセージを更新
        latest_timestamp = messages[-1]["ts"]

        # メッセージのテキストを取得
        text = messages[-1]["text"]
        print("New message:", text)
        
        # VoiceVox APIを使って音声ファイルを生成
        start_time = time.time()
        params = {"text": text, "speaker": 1}
        response = requests.post(synthesize_endpoint, params=params)
        audio_query = response.json()
        # print("audio_query:")
        # print(audio_query)
        params = {"speaker": 1}
        response = requests.post(audio_endpoint, params=params, json=audio_query)
        end_time = time.time()
        print("VoiceVox API request time:", end_time - start_time, "seconds")
        audio_data = response.content

        # 音声ファイルを再生
        start_time = time.time()
        wf = wave.open(io.BytesIO(audio_data), 'rb')
        stream = p.open(format=p.get_format_from_width(wf.getsampwidth()),
                        channels=wf.getnchannels(),
                        rate=wf.getframerate(),
                        output=True,
                        output_device_index=output_device_id)
        end_time = time.time()
        print("Audio play setup time:", end_time - start_time, "seconds")
        start_time = time.time()
        data = wf.readframes(1024)
        while data:
            stream.write(data)
            data = wf.readframes(1024)
        stream.stop_stream()
        stream.close()
        print("Done")
        end_time = time.time()
        print("Audio play time:", end_time - start_time, "seconds")
    # 一定時間待機（APIのレートリミットを考慮）
    print("sleeping...")
    time.sleep(1)
