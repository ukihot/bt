# 6. 演出の到達点(スクリプトされた行)

出典: `CLAUDE.md` §7、実装 `src/domain/generate.rs` `src/screens/playing/intrusion.rs`
`src/screens/lost.rs` `src/fonts.rs`

演出の到達点は順序固定。すべて `scripted: true` としてカーソルの `Pending` ウィンドウに
入らない(触れられない)行として実装されている。

## 1. 序盤: 筆癖の違和感

画面ごとに異なる文体([05-threats-and-tells.md](05-threats-and-tells.md))に馴染むほど、
逸脱が見えてくる。専用の演出コードはなく、通常の生成ロジックがそのまま担う。

## 2. 中盤: 誤操作への障り(二人称の障り)

`mistake_beat(clock, verb)`(`generate.rs:185`)。プレイヤーが初めて `resolve()` で
`corruption > 0.0` となる誤操作をした瞬間、その画面の焼成室に業務ログの文法のまま二人称が
現れる:

```
06:52 あなたは 行を 削除した
```

`GameData.first_mistake_done` で1ラン中1回だけに制限されている(`spawn.rs:160-169`)。書体は
Potta One に切り替わる(`LineFont::Mistake`, `src/fonts.rs`)— 他の全ての通常・脅威・呼びかけ
行が Noto Sans JP で統一されているのに対し、この行だけが視覚的に浮く。

## 3. 終盤: 呼ばれる

プレイヤーが開始時に入力した名で一度だけ呼ばれる。発火条件は `GameData::maybe_queue_name_call`
(`src/game_data.rs:63`): `Zone::Counter` かつ `Phase::Night` かつ奇数分が揃った瞬間、ラン中
1回だけ(`name_call_done`)。

```
07:14 ◯◯さん、裏口
```

この行は焼成室・外・売り場のどの画面にも属さない — グリッドの外、独立した `IntrusionSlot`
エンティティとして描画される(`src/screens/playing/intrusion.rs`)。**構造的に不可触**:
`Pending` を一切持たないので、そもそも印をつける仕組みが存在しない(禁止マナーとしての注意
喚起ではなく、実装レベルでの保証)。

表示中に `Z`/`X` を押すと「応えた」ことになり、`INTRUSION_LIFETIME_SECONDS`(6秒)経過まで
無操作なら「静観」— どちらも `domain::resolve(ShouldNotReact, ...)` で判定される
(`resolve_intrusion`)。削除も検印も「返事」である。

書体は Yuji Syuku(`LineFont::Call`)— 呼びかけ演出・Lost画面の侵食ログと共通。

## 4. 最終: 終わらない

守り切っても終了ログは出ない。閉店記帳の後、翌朝の仕込みログが3画面とも普通に始まり、静かに
流れ続ける。日課は引き継がれた。専用の「クリア」処理は実装されていない — `corruption_check`
が100に達しない限り、ゲームは単にループし続ける。

## ゲームオーバー時の描写(「別の何かの記録」)

侵食度が100に達すると `AppState::Lost` に遷移し、`domain::corrupted_line`(`generate.rs:205`)
が1.4秒おきに新しい行を流し続ける。この関数だけは意図的に共有クロックを読まず、時刻・数値を
独立乱数で決める — 時間そのものが意味を失ったことの表現(バグではない)。

```
何かが7本 焼き上がり
帳場に誰かが座っている
あなたの名が3回 記帳された
この店は、まだ開いている
続けてください
扉に鍵はかかっていない
```

9秒後、無言でタイトルへ戻る。勝敗UI・スコア表示は一切ない。

## 日替わりの記帳

`domain::day_marker(day)`(`generate.rs:221`)。「開店から{day}日目 仕込みを始める」という
スクリプト行で、日をまたぐたびに焼成室にだけ流れる(`phase_tick`, `spawn.rs:74-76`) — 店主の
記録である焼成室だけがこの記帳を持つ。
