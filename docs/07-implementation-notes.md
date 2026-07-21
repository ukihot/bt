# 7. 実装状況とCLAUDE.mdとの差分

出典: `CLAUDE.md` §9, §10、および `src/` 全体の読解。

## モジュールマップ

```
src/
├── domain/          # 純粋ロジック。Component/Resource/systemsを持たず、
│                     # cargo test だけで単体テストできる(Bevy App不要)
│   ├── clock.rs      # DayClock — 店にひとつだけの共有クロック
│   ├── phase.rs       # Phase — clockから導出、独自タイマーなし
│   ├── zone.rs        # Zone — 脅威の距離(外周→内側→帳場)
│   ├── pane.rs        # Pane — 焼成室/外/売り場の識別・capacity・スポーン間隔
│   ├── verb.rs        # Verb — 削除/検印の2つのみ
│   ├── log_line.rs    # Classification, LogLine, resolve() — 判定ロジック
│   ├── customer.rs    # Customer, CustomerId — 噂の話者
│   ├── rumors.rs      # ThreatKind, Effect, CATALOG — 禁忌集のデータ
│   ├── rules.rs       # RuleLedger, Cast, verdict() — 禁忌集の解決エンジン
│   ├── threats.rs     # 禁忌集の判定を持たない、コンテンツ生成だけの層
│   ├── timestamp.rs   # 時刻・本数の文字列生成ヘルパー
│   └── generate.rs    # 画面ごとの行生成、重み付け抽選の統合
├── app_state.rs     # AppState(Title/Playing/Lost)
├── game_data.rs     # GameData — 3画面共有のグローバル状態
├── fonts.rs         # 3書体のロード
├── theme.rs         # BG/FG/DIM/MONITOR_BG の4色のみ
└── screens/
    ├── title.rs      # 名前入力画面
    ├── lost.rs       # 侵食ログ演出→タイトルへ
    └── playing/
        ├── pane.rs       # PaneRuntime(画面ごとのspawn_timer等)
        ├── pending.rs    # Pending — カーソル窓とeviction
        ├── render.rs     # 状態→表示テキストの変換、per-cell wipe()アニメーション
        ├── spawn.rs      # phase_tick, line_spawn — 行生成・遅延判定の駆動
        ├── input.rs      # J/K/Z/X の入力処理(常に焼成室のみ)
        ├── intrusion.rs  # 呼ばれる専用の不可触スロット
        ├── pause.rs      # ESC一時停止(AppState遷移ではなくVisibility切替)
        ├── glitch.rs     # CRTグリッチ(環境演出のみ)
        ├── corruption.rs # 侵食度100到達の監視→Lost遷移
        └── setup.rs      # UIツリーの構築・破棄
```

## 解消済み: 操作対象の範囲(§3 再設計 vs §10 実装)

`CLAUDE.md` §3.1〜§3.2(2026-07-17再設計)はこう定めている:

> 操作できるのは焼成室だけ。カーソル(J/K)と削除・検印(Z/X)はすべて焼成室のログに対しての
> み作用する。外・売り場に操作対象は存在せず、**画面を切り替えるという概念自体がない**

以前は実装(`src/screens/playing/{pane,input,setup}.rs`)がこの再設計より前のアーキテクチャの
ままで、`ActivePane`リソース・`H`/`L`によるパネル切り替え・外/売り場固有のカーソル窓が実在し、
プレイヤーが外や売り場のログにもカーソルを合わせて削除/検印できてしまう状態だった。

**2026-07-21、この差分を解消した:**

- `ActivePane` リソース・`handle_pane_switch`・`H`/`L` の入力ハンドラを削除。`handle_line_input`
  は常に焼成室の `Pending` だけを操作する
- パネル見出しの選択表示(`PaneHeader`/`sync_pane_headers`)を削除し、焼成室にだけ常時
  `CURSOR_MARK` を立てる静的な見出しに置き換えた(`setup.rs`)
- 画面下部の凡例から `"画面切替 H/L"` を削除
- `Outside`/`Floor` は `Pending`(カーソル窓・eviction)自体は引き続き持つが(表示上必要な
  仕組みのため)、行が `Pending` から外れても `domain::resolve()` を一切呼ばなくなった
  (`spawn.rs::line_spawn`)——呼んでしまうと、操作不能なこの2画面の`mark`は常に`None`のため、
  「正解」がある分類は必ずタタリと判定されてしまう
- ドメイン層(`domain::rumors`/`threats`/`generate`)から、`Outside`/`Floor` 固有だった反応型
  脅威(`ThreatKind::OutsideRepeat`/`ClosingTime`/`BackDoor`、禁忌#7〜#9)と、それらに紐づく
  禁忌集の該当項目、および禁忌#8を対象にしていた `Effect::Discredit` を削除した——詳細は
  [04-rules-and-rumors.md](04-rules-and-rumors.md#2026-07-21-の設計変更実装済み-操作対象の範囲)
  [05-threats-and-tells.md](05-threats-and-tells.md) 参照

## `CLAUDE.md` §9 に記載済みの未確定事項(参考として集約)

- 禁忌集の全項目リストと解放順(現状7項目で打ち止め、増量時の設計は未検討)
- 脅威図鑑:「どう書くか」ベースの書き分け一覧は未整理
- 経済パラメータ(売上と侵食率のバランス)— `income` は内部計算のみでUI・corruptionへの
  フィードバックなし
- 売り場発ルール変更の語彙集の網羅性、同時に有効なルール数の上限
- 外のステータス表示のエスカレーション閾値(25/50/75)と語彙は第一稿、数値調整は未実施
- `Condition` は `DayHasThree` の1種、`Effect` も3種のみ — 禁忌集が増えたときにこの表現力で
  足りるかは未検証
- 客ごとのトーン・語調変化(`Customer` は名前のみ、個性づけは未実装)
- 噂が使う時間帯語彙(朝・午後・夕方等)と `Phase::hour_range()` との対応表の具体的な区切り
- 仮題「Bakery Text」の正式決定
- **禁忌同士の絡み合い(discredit型等)の強化。** 方向性の検討のみで実装は0件(2026-07-21、
  唯一の実例だった`Effect::Discredit`を機構ごと削除——詳細は
  [04-rules-and-rumors.md](04-rules-and-rumors.md#未実装方向性のみ決定の項目) 参照)

## 2026-07-20 実装済みの大きな変更

- **タタリの統一と検印(Stamp)の救済。** `Classification::correct_action()` が分類ごとの唯一の
  正解(通常業務行=検印/反応すべき行=削除/反応してはいけない行=静観)を返す形に作り直し、
  検印が一度も正解にならないという欠落を、新しい `ThreatKind` を増やすことなく解消した。
  `LogLine.taboo`・`Effect::Enable` の `taboo` 引数・`Verdict::Active` の `taboo` ペイロードは
  すべて削除済み — 正解を外せば分類問わず一律「タタリ」(corruption + zone_bump)になる仕様に
  統一されたため不要になった。詳細は
  [04-rules-and-rumors.md](04-rules-and-rumors.md#正解アクション定義表) と
  [02-screens-and-controls.md](02-screens-and-controls.md#反応の三分類-分類ごとに正解はちょうど1つそれ以外はタタリ) 参照
- **ルール効果の日次リセット。** `RuleLedger::reset_day()` が `heard` を丸ごとクリアする形で
  実装済み(`Cast` は残る)。`screens::playing::spawn::phase_tick` が日をまたぐたびに呼ぶ。
  `domain::rule_reset_notice` が売り場側の日替わり通知(焼成室の `day_marker` と対になる)も
  合わせて実装済み。詳細は
  [04-rules-and-rumors.md](04-rules-and-rumors.md#2026-07-20-の設計変更実装済み) 参照

## テスト方針

`src/domain/` 配下は Bevy の `Component`/`Resource`/systems を一切持たないため、`cargo test`
だけで単体テストできる。各ファイルに `#[cfg(test)] mod tests` があり、`resolve()` の全分岐、
`RuleLedger::verdict` の優先順位、`DayClock` の非後退性、`Pending` の eviction/cursor追従
などがカバーされている。ECS層(`screens/`)側は `Pending`/`render::body_glyphs`/`mark_glyph`/
`pause` の純粋関数部分のみ単体テストがあり、systems 自体の統合テストは現状ない。
