![License](https://img.shields.io/badge/license-MIT-green)<br>
![Rust](https://img.shields.io/badge/Rust-1.80.1-orange?style=flat-square&logo=rust)

# Rusty_Encryptor

シンプルで高速なAES-256-GCM暗号化ツール．CLIを使って簡単にファイルやテキストの暗号化・復号化ができる．

## 特徴
- AES-256-GCMによる高速な暗号化・復号化
- シンプルなコマンドラインインターフェース
- ファイルやテキストのセキュアな暗号化

## 必要なライブラリ

このプロジェクトでは以下のライブラリを使用している．

```
[dependencies]
clap = "4.1"
ring = "0.16"
base64 = "0.21"
```

### 暗号化

テキストをAES-256-GCMで暗号化する．暗号化には32バイト（base64エンコード済み）の鍵が必要である．

```
./target/release/rusty_encryptor -e "Hello, world!" -k "YOUR_BASE64_ENCODED_32_BYTE_KEY"
```

### 復号化

暗号化されたテキストを復号化する．同じ鍵を使用して復号化する．

```
./target/release/rusty_encryptor -d "ENCRYPTED_TEXT" -k "YOUR_BASE64_ENCODED_32_BYTE_KEY"
```

### 注意
- AES-256-GCMは12バイトのNonceを使用する．現在の実装では固定値を使用しているが，実際の運用ではランダムに生成し，暗号文とともに管理する必要がある．
- 鍵の管理には十分な注意が必要である．適切な管理ツールを利用すること．
