# Bakery Text ドキュメント

`CLAUDE.md` の仕様と `src/` の実装を突き合わせて書き起こした、ゲームデザインの整理資料。
- 設計意図を知りたいときは各章を参照
- 実装が仕様通りかを知りたいときは [07-implementation-notes.md](07-implementation-notes.md) を参照

各ファイルの記述はコード読解に基づく事実確認込み。`CLAUDE.md` と矛盾する記述を見つけた場合は
`CLAUDE.md` が正 — このドキュメントは派生物であり、一次情報源ではない。

## 目次

1. [01-concept.md](01-concept.md) — コア体験・和ホラーの絶対原則
2. [02-screens-and-controls.md](02-screens-and-controls.md) — 3画面構成・操作・判定の遅延
3. [03-day-cycle.md](03-day-cycle.md) — 共有クロック・フェーズ・日をまたぐ構造
4. [04-rules-and-rumors.md](04-rules-and-rumors.md) — 禁忌集・ルール変更・RuleLedgerの解決順序
5. [05-threats-and-tells.md](05-threats-and-tells.md) — 筆癖・脅威の分類・重み付け抽選
6. [06-narrative-beats.md](06-narrative-beats.md) — スクリプトされた演出の到達点
7. [07-implementation-notes.md](07-implementation-notes.md) — コード構成とCLAUDE.mdとの差分・未実装箇所
