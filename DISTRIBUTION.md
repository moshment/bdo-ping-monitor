# 配布用パッケージ作成手順

## 自分でビルドして配布する方法

### 1. Windowsマシンでビルド

```powershell
# Tauri CLIが必要
cargo install tauri-cli --locked

# プロジェクトディレクトリへ
cd bdo-ping-tauri

# ビルド実行
cargo tauri build
```

### 2. 単一exeファイルを抽出

ビルド完了後、以下のファイルが生成されます：

```
src-tauri\target\release\bdo-ping-monitor.exe
```

### 3. 配布用ZIP作成

```powershell
# 配布フォルダ作成
mkdir BDO-Ping-Monitor-v1.0

# exeコピー
copy src-tauri\target\release\bdo-ping-monitor.exe BDO-Ping-Monitor-v1.0\

# README同梱
notepad BDO-Ping-Monitor-v1.0\README.txt
```

README.txt内容：
```
黒い砂漠 サーバーピングモニター v1.0

【使い方】
1. bdo-ping-monitor.exe をダブルクリック
2. アプリが起動したら自動的に測定開始
3. 終了時はウィンドウを閉じるだけ

【対応サーバー】
- malni-2ch
- Valencia-2ch

【測定方法】
TCP接続レイテンシを5秒ごとに測定

【注意】
- Windowsファイアウォール警告が出たら「許可」を選択
- WebView2が必要（Windows 11標準／Windows 10は別途インストールが必要な場合あり）
```

### 4. ZIP圧縮

```powershell
Compress-Archive -Path BDO-Ping-Monitor-v1.0 -DestinationPath BDO-Ping-Monitor-v1.0.zip
```

## 配布物構成

```
BDO-Ping-Monitor-v1.0/
├── bdo-ping-monitor.exe    (約 8-12MB)
└── README.txt              (使い方)
```

## 受け取った人に必要なもの

- Windows 10 以降
- **WebView2 ランタイム**（初回のみ、多くの場合既にインストール済み）
  - https://developer.microsoft.com/ja-jp/microsoft-edge/webview2/

## オプション：インストーラー版

インストーラー形式（.msi）の場合：

```
src-tauri\target\release\bundle\msi\BDO-Ping-Monitor_1.0.0_x64_en-US.msi
```

または NSIS インストーラー：
```
src-tauri\target\release\bundle\nsis\BDO-Ping-Monitor_1.0.0_x64-setup.exe
```

## 不明な場合

自分のWindowsマシンで以下を順番に実行：

```powershell
# 1. PowerShellを開く
# 2. Rustが入っていなければインストール: https://rustup.rs
# 3. Visual Studio Build Toolsをインストール
#    https://aka.ms/vs/17/release/vs_BuildTools.exe
#    → C++でデスクトップ開発 を選択

# 4. Tauri CLIインストール
cargo install tauri-cli --locked

# 5. ビルド
cd bdo-ping-tauri
cargo tauri build

# 6. 待つ（10-15分）
```

完了するとexeができます。
