# 🧠 mjbk - Brainfuck 系 日本語トークン言語インタプリタ（Rust 製）

> #### ⚠️ このプロジェクトは完全に学習用で実装されているため，設計やエラーハンドリングが不十分かつ，脆弱性やバグが含まれており，未完成です．

`mjbk` は Brainfuck をベースにしたオリジナルプログラミング言語です。
文字化け風のトークンで記述されたコードを解釈し，Brainfuck と同様の動作を行います。

## ✨ 特徴

- 🦀 Rust 製
- 🗾 トークンが日本語風にマッピングされている．
- ✅ トークン列 → 字句解析 → 構文解析 → 実行 の順にインタプリタを構築
- 🧪 拡張子は　`.mjbk`

## 🔤 トークン対応表

| 記号 | 意味           | トークン例 |
| ---- | -------------- | ---------- |
| `+`  | インクリメント | `繝ｫ`      |
| `-`  | デクリメント   | `隱`       |
| `>`  | ポインタ右移動 | `繧ｪ`      |
| `<`  | ポインタ左移動 | `縺`       |
| `.`  | 出力           | `繝?`      |
| `,`  | 入力           | `吶`       |
| `[`  | ループ開始     | `峨`       |
| `]`  | ループ終了     | `焚`       |

## 📦 ビルドと実行

### 🔧 依存関係

- `clap`（オプション解析用）

### 🚀 実行方法

```bash
> cargo run -- src/example/hello.mjbk
```

```bash
> cargo run -- -h
Minimal Japanese Mojibake Brainfuck-style interpreter type language.

Usage: mojibake <FILE>

Arguments:
  <FILE>  実行する .mjbk ファイルへのパス

Options:
  -h, --help     Print help
  -V, --version  Print version
```
