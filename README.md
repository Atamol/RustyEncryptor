![License](https://img.shields.io/badge/license-MIT-green)<br>
![Rust](https://img.shields.io/badge/Rust-1.80.1-orange?style=flat-square&logo=rust)

# rusty_encryptor

シンプルで高速なAES-256-GCM暗号化ツールです．CLIを使って簡単にファイルやテキストの暗号化・復号化ができます．

## 特徴
- AES-256-GCMによる高速な暗号化・復号化
- シンプルなコマンドラインインターフェース
- ファイルやテキストのセキュアな暗号化

## 必要なライブラリ

このプロジェクトでは以下のライブラリを使用しています．

```
[dependencies]
clap = "4.1"
ring = "0.16"
base64 = "0.21"
```

## インストール

プロジェクトをクローンし，依存関係をインストールします．

```
git clone https://github.com/Atamol/rusty_encryptor.git
```

```
cd rusty_encryptor
cargo build --release
```

## 使用方法

### 暗号化

テキストをAES-256-GCMで暗号化します．暗号化には32バイト（base64エンコード済み）の鍵が必要です．

```
./target/release/rusty_encryptor -e "Hello, world!" -k "YOUR_BASE64_ENCODED_32_BYTE_KEY"
```

### 復号化

暗号化されたテキストを復号化します．同じ鍵を使用して復号化します．

```
./target/release/rusty_encryptor -d "ENCRYPTED_TEXT" -k "YOUR_BASE64_ENCODED_32_BYTE_KEY"
```

### 注意
- AES-256-GCMは12バイトのNonceを使用します．現在の実装では固定値を使用していますが，実際の運用ではランダムに生成し，暗号文とともに管理する必要があります．
- 鍵の管理には十分な注意が必要です．適切な管理ツールを利用してください．
