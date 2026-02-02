# yaziアーキテクチャ実装ガイド

## 実装した内容

yaziのアーキテクチャを参考に、以下の要素をpueue-tuiに実装しました：

### 1. **Command System** (`command.rs`)

yaziの`CmdCow`に相当するコマンドシステム：

```rust
let cmd = Command::new("arrow").arg("1");
let cmd = Command::quit();
```

- コマンド名と引数を持つ構造体
- `Cow<'static, str>`でゼロコストな文字列管理
- ビルダーパターンでの構築

### 2. **Router** (`router.rs`)

キー入力をCommandに変換：

```rust
// 例: 'j' → Command("arrow", ["1"])
// 例: 'q' → Command("quit", [])
```

- yaziのRouter patternを実装
- 将来的に設定ファイル対応可能な設計

### 3. **Actor System** (`actors/`)

各機能をActorとして実装：

```rust
pub trait Actor {
    type Options;
    fn act(cx: &mut Ctx, options: Self::Options) -> Result<()>;
}
```

**Ctx（実行コンテキスト）**:
```rust
pub struct Ctx<'a> {
    pub core: &'a mut Core,      // 状態へのアクセス
    pub level: usize,             // ネストレベル
    #[cfg(debug_assertions)]
    pub backtrace: Vec<&'static str>,  // デバッグ用
}
```

### 4. **Dispatcher** (`dispatcher.rs`)

イベントをルーティング：

- システムイベント（Resize, Focus等）→ 直接処理
- キーイベント → Router経由でCommand化
- Commandの実行をExecutorに委譲

### 5. **Executor** (`executor.rs`)

Commandを実行し、Actorを呼び出し：

```rust
match command.name.as_ref() {
    "quit" => Quit::act(&mut cx, ()),
    "arrow" => Arrow::act(&mut cx, step),
    _ => Ok(())
}
```

## 新しいActorを追加する方法

### ステップ1: Actorファイルを作成

`src/actors/my_feature.rs`:

```rust
use crate::actors::{Actor, Ctx};

pub struct MyFeature;

impl Actor for MyFeature {
    type Options = String;  // 引数の型

    fn act(cx: &mut Ctx, options: Self::Options) -> color_eyre::Result<()> {
        // cx.core で状態にアクセス
        // cx.enter("my_feature") でデバッグ情報を記録
        
        // 実装...
        
        Ok(())
    }
}
```

### ステップ2: `actors.rs` に登録

```rust
pub mod my_feature;
```

### ステップ3: Executorに追加

`src/executor.rs`:

```rust
use crate::actors::{my_feature::MyFeature, ...};

match command.name.as_ref() {
    "my_feature" => {
        cx.enter("my_feature");
        let arg = command.first_arg().unwrap_or("");
        let result = MyFeature::act(&mut cx, arg.to_string());
        cx.exit();
        result
    }
    ...
}
```

### ステップ4: Routerにキーマッピングを追加

`src/router.rs`:

```rust
(KeyCode::Char('m'), KeyModifiers::NONE) => {
    Some(Command::new("my_feature").arg("argument"))
}
```

## yaziから学んだ設計原則

### 1. **システムイベントは直接処理**

```rust
// Dispatcherで直接処理
Event::Resize => self.dispatch_resize(),
Event::Quit => self.dispatch_quit(),

// ではなく、Router経由にはしない理由：
// - パフォーマンス（文字列マッチング不要）
// - 確実性（必ず処理される）
// - 型安全性（コンパイル時チェック）
```

### 2. **Ctxでネストレベル管理**

```rust
cx.enter("actor_name");  // レベル++, バックトレース追加
// ... 処理 ...
cx.exit();               // レベル--, バックトレース削除
```

無限再帰を防ぎ、デバッグ情報を提供します。

### 3. **Single Source of Truth**

`Core`がすべての状態を保持。各Actorは`Ctx`経由でアクセス：

```rust
cx.core.should_quit = true;
```

### 4. **イベント駆動アーキテクチャ**

```rust
// イベントを発行
Event::Render.emit();

// メインループで処理
loop {
    event_rx.recv_many(&mut events, MAX_EVENTS).await;
    self.process_events(&mut events)?;
}
```

## 現在のディレクトリ構造

```
src/
├── main.rs          # エントリーポイント
├── app.rs           # メインループ（Event::init, render）
├── event.rs         # Event定義とグローバルチャネル
├── dispatcher.rs    # イベントルーティング
├── router.rs        # キー→Command変換
├── command.rs       # Command構造体
├── executor.rs      # Command実行、Actor呼び出し
├── core.rs          # アプリケーション状態
├── actors.rs        # Actor trait, Ctx定義
├── actors/
│   ├── quit.rs      # 終了
│   ├── render.rs    # 再描画
│   └── arrow.rs     # カーソル移動（未完成）
├── ui.rs            # UI描画
├── terminal.rs      # ターミナル制御
└── cli.rs           # CLI引数
```

## 次のステップ

### 優先度高：Core状態の拡張

`src/core.rs`にタスクリストなどを追加：

```rust
pub struct Core {
    pub should_quit: bool,
    pub tasks: Vec<Task>,      // タスクリスト
    pub selected: usize,        // 選択中のインデックス
    pub filter: Option<String>, // フィルター
}
```

### 優先度中：Arrowの完全実装

`src/actors/arrow.rs`でカーソル移動を実装：

```rust
fn act(cx: &mut Ctx, step: i32) -> Result<()> {
    let new_idx = (cx.core.selected as i32 + step)
        .max(0)
        .min(cx.core.tasks.len() as i32 - 1);
    cx.core.selected = new_idx as usize;
    
    // 再描画をトリガー
    Render::act(cx, ())?;
    Ok(())
}
```

### 優先度低：IO Actor（daemon通信）

バックグラウンドでのデータ取得を独立したActorに：

```rust
pub struct IoActor {
    client: Client,
    rx: mpsc::Receiver<IoEvent>,
}

impl IoActor {
    async fn run(&mut self) {
        while let Some(event) = self.rx.recv().await {
            match event {
                IoEvent::FetchTasks => {
                    let tasks = self.client.get_tasks().await?;
                    Event::TasksLoaded(tasks).emit();
                }
            }
        }
    }
}
```

## yaziとの違い（意図的な簡略化）

| 機能 | yazi | pueue-tui | 理由 |
|------|------|-----------|------|
| Scheduler | あり | なし（今後追加） | 現時点では不要 |
| Plugin System | Lua | なし | 要件にない |
| 複数Layer | あり | なし | 単一画面で十分 |
| 設定ファイル | TOML | なし（今後追加） | まずは動作を優先 |
| act!マクロ | あり | なし | Rustの明示的な呼び出しで十分 |

## ビルドと実行

```bash
# ビルド
cargo build

# 実行
cargo run

# 現在は 'q' または Ctrl+C で終了のみ可能
```

## まとめ

yaziの優れたアーキテクチャパターンを採用しつつ、pueue-tuiの要件に合わせて簡略化しました。今後の拡張に備えた柔軟な設計になっています。

詳細は `ARCHITECTURE.md` を参照してください。
