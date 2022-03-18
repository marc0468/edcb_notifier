# EDCB Notifier

Discord、LINE、Slackにメッセージを送信するツールです。
Windows環境ではbatでcurlを実行すると日本語が文字化けするため、このツールを作成しました。

## 使い方

下記コマンドで実行します。

```bat
edcb_notifier.exe <config.jsonのパス> <送信したいメッセージ>
```

以下のような`config.json`を作成してください。
`is_disabled`が`true`の場合メッセージを送信します。

```config.json
{
    "discord": {
        "webhook_url": "url",
        "is_disabled": false
    },
    "line": {
        "token": "token",
        "is_disabled": false
    },
    "slack": {
        "webhook_url": "url",
        "is_disabled": false
    }
}
```
