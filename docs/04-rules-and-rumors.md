# 4. ルール変更と禁忌集

出典: `CLAUDE.md` §3.4, §5、実装 `src/domain/{rules,rumors}.rs`

## 売り場が唯一のルール発信源

売り場は接客ログの体裁のまま、客の噂話を拾って記録する。この一文が成立した瞬間から、焼成室の
反応ルールがひとつ書き換わる。理由は説明されない。**焼成室には一切書かれない** — ルールに
関する文面の出所を売り場一箇所だけに絞ることで、操作対象の画面(焼成室)でルールが宣言・確定
したかのように見える事故を構造的に防いでいる(`CLAUDE.md` §3.4・§4)。

店に古くから伝わる言い伝えも、今まさに効いているルール変更も、同じ客の噂話として同列に流れる。
両者が矛盾した場合は、あとから流れてきた噂のほうが優先される — 理由は語られない。

## `RuleLedger` — 発話順の台帳として解決する

`src/domain/rules.rs`。噂を「そのランで実際に語られた `RumorId` の列」としてただ記録するだけの
台帳(`heard: Vec<RumorId>`)。ルールの上書き・矛盾解決はすべて `verdict()` が担い、履歴を
書き換えることはしない — あとから信用を失った噂も「聞かなかったこと」にはならず、クエリ時に
無効化されるだけ。

### `verdict(pane, threat, ctx) -> Verdict`

`heard` を**新しい順**に走査し、話者が信用を失っていない(`is_discredited` でない)項目の中から、
`threat` に言及する最初の `Enable`/`Void` を採用する。

```
for id in heard.rev():
    if speaker_of(id) is discredited: skip
    match effect:
        Enable{pane, threat} if matches → Active
        Void{threat, condition} if matches and condition holds → Suppressed
一致なし:
    ThreatKind::Repeat → Active       # 初日から反応対象の既定異常
    それ以外 → Suppressed              # 噂で Enable されるまで起こらない
```

「矛盾するルールが出た場合は新しいものが優先される」(`CLAUDE.md` §3.4)を、噂の種類が増えても
ペアごとのハードコードなしに実現している。`Verdict` はもう `taboo` ペイロードを持たない単純な
`Active`/`Suppressed` の2値 — 分類ごとの正解を外せば必ず「タタリ」になる仕様に統一されたことで
(下記「正解アクション定義表」参照)、個別の重み付けが不要になった(2026-07-20)。

### `reset_day()` — 日次リセット

`RuleLedger::reset_day()` は `heard` を丸ごとクリアするだけの操作(`Cast` には触れない)。
共有クロックが日をまたぐたびに `screens::playing::spawn::phase_tick` から呼ばれ、聞いた噂の
効果は`Discredit`を含めてすべてその日限りで消える — 翌日また同じ噂を聞き直さない限り、
`Enable`/`Void`/`Relieve`/`Discredit` のどれも「聞かなかったこと」に戻る(2026-07-20実装)。

### `relief_bonus(threat) -> f32`

同じ新しい順スキャンで `Effect::Relieve` を探す姉妹クエリ。`Relieve` は `Active`/`Suppressed`
とは独立した軸 — 援助系の禁忌がその脅威を「反応可能」にするわけではなく、既に反応可能な脅威を
正しく処理したときの corruption 回復を上乗せするだけ。

### `Cast` — 話者はランごとにランダム割り当て

禁忌の内容と機械的効果(`Effect`)はカタログ固定データだが、**誰が言うか**は `RuleLedger::new()`
のたびに `rand::rng()` で再抽選される(`Cast::roll`)。「同じ噂は毎回同じ客が言う」という覚える
だけのパターンにならないための設計(ローグライク原則、`CLAUDE.md` §3.6: ラン間で持ち越すのは
知識だけ)。

`Effect::Discredit` は対象を `CustomerId` ではなく `target: RumorId`(他のカタログ項目)で
指す。話者はランごとに変わるので、カタログ側は「誰を」ではなく「どの項目の話者を」
discredit するかだけを固定する。

- discredit は連鎖しない: discredit した側の話者が別の噂で discredit されても、元の
  discredit 自体は取り消されない(`discredit_does_not_chain_through_its_own_speaker` テスト)
- `Cast::roll` は「discredit の対象と、discredit する側の話者が同一人物」になるケースを
  reject sampling で排除する(自分で自分の噂を「あの人は信用できない」と言うことはない)

## `Effect` の種類(4種)

| Effect | 効果 |
|---|---|
| `None` | 純粋な伏線。聞いても機械的には何も変わらない(奇数分・二度聞き・名指し・焼き上がりの字、など常に真の禁忌を予告するだけの項目) |
| `Enable { pane, threat }` | 聞いたその日のうちだけ、`threat` が `pane` で反応対象になる(`taboo` パラメータは2026-07-20に廃止 — 下記「正解アクション定義表」参照) |
| `Void { threat, condition }` | 聞いたその日のうちだけ、`condition` が成立する間 `threat` は反応対象でなくなる |
| `Relieve { threat, bonus }` | 聞いたその日のうちだけ、`threat` を正しく処理すると追加で `bonus` だけ corruption を回復する(援助系) |
| `Discredit { target }` | `target` の話者を、聞いた日のうちだけ信用不能にする(その話者由来の `Enable`/`Void`/`Relieve` は聞かなかったことになる) |

すべての効果が「聞いたその日のうちだけ」なのは `RuleLedger::reset_day()` による日次リセットの
結果(上記参照)。翌日も効かせたい場合は、その日また同じ噂を聞き直す必要がある。

`Condition` は現状 `DayHasThree`(日数に3を含む日、例: 3, 13, 23...)の1種のみ。

## 禁忌集カタログ(全11項目、`src/domain/rumors.rs`)

売り場の客の噂話として、ランダムに1件ずつ流れる(`rumor_line`)。プレイスホルダーの `{name}`
はその噂の話者名(`Cast` が決める)で埋められる。

| # | 本文の要旨 | 効果 |
|---|---|---|
| 0 | 奇数分に帳面をつけると良くないらしい | `None`(伏線 — 焼成室の偶数分丸めの逸脱、呼びかけの奇数分の予告) |
| 1 | 夜の納品だけは断ったほうがいい | `Enable{Kiln, NightDelivery}` |
| 2 | 三のつく日は窯の火を落とさないほうがいい | `Void{Repeat, DayHasThree}` |
| 3 | 同じ話を二度されても続けて聞かないほうがいい | `None`(伏線) |
| 4 | 名前を呼ばれてもすぐには信じないほうがいい | `None`(伏線 — 「呼ばれる」演出の予告) |
| 5 | 焼き上がりの字だけは崩さないでほしい | `None`(伏線 — 「焼き上がり」表記の逸脱の予告。3基本異常はもともと常に`ShouldReact`なので、この噂は機械的な効果を持たない) |
| 6 | この店、閉店時間だけは聞かないほうがいい | `Enable{Floor, ClosingTime}` |
| 7 | 裏口には近づかないほうがいい | `Enable{Outside, BackDoor}` |
| 8 | 外の異常なしは二回続けて信じてはいけない | `Enable{Outside, OutsideRepeat}` |
| 9 | 裏口の話をしていた人のことはあまり当てにならない | `Discredit{target: 7}` |
| 10 | 帳面の同じ行に気づいたらすぐに消すと楽になるらしい | `Relieve{Repeat, bonus:3.0}`(援助系) |

禁忌集の項目リスト = 実質のレベルデザイン。日ごとに一項ずつ意味を持ち始める構成
(`CLAUDE.md` §5)。項目 9 は項目 7 を discredit する — 一度 8番の裏口の噂を信じて `BackDoor`
を有効化した後でも、9番を聞くと「聞かなかったこと」に戻る。項目 10 だけが唯一の援助系
(`Effect::Relieve`)で、`Zone` を一切動かさず corruption の回復だけを厚くする。

## `ThreatKind`(5種、`src/domain/rumors.rs`)

| ThreatKind | 既定状態 | 出現画面 |
|---|---|---|
| `Repeat` | 初日から `Active`(焼成室・売り場の既定の反復。項目2で voidable) | Kiln, Floor |
| `OutsideRepeat` | 既定 `Suppressed`(項目8で `Enable` されるまで存在しない) | Outside |
| `NightDelivery` | 既定 `Suppressed`(項目1で `Enable`) | Kiln、かつ夜のフェーズのみ |
| `ClosingTime` | 既定 `Suppressed`(項目6で `Enable`) | Floor |
| `BackDoor` | 既定 `Suppressed`(項目7で `Enable`、項目9で discredit 可) | Outside |

終盤に示してよい事実はただ一つ: **客が語った最後の噂の中身が、プレイヤーの入店より後に起きた
出来事を指している**こと。それ以上は何も語らない(`CLAUDE.md` §5)。

## 正解アクション定義表

「求められている行動が検印なのか削除なのか静観なのか」を、脅威・分類ごとに1つだけ定義する表。
2026-07-20 のリファクタで `Classification::correct_action()` として実装が一本化された
(以前は `LogLine.correct_verb` を各生成関数が個別に指定していたため、単一の定義表としては
存在しなかった)。

| 対象 | 分類 | 正解 | 備考 |
|---|---|---|---|
| 通常業務行(`Normal`) | `Normal` | 検印 | 何もしない(静観)ことも含め、それ以外はすべてタタリ |
| 表記の乱れ(「焼きあがり」) | `ShouldReact` | 削除 | 常時有効 |
| 数え違い(「個」) | `ShouldReact` | 削除 | 常時有効 |
| 記載漏れ | `ShouldReact` | 削除 | 常時有効 |
| 反復(`Repeat`, `Active`時) | `ShouldReact` | 削除 | 項目2(三のつく日)で`Suppressed`になると`Normal`表示に切り替わり、正解も検印になる |
| 反復(`OutsideRepeat`, `Active`時) | `ShouldReact` | 削除 | 項目8を聞いた当日だけバケットが存在する |
| 夜の納品(項目1で`Enable`) | `ShouldNotReact` | 静観 | 夜のフェーズのみ出現。項目1を聞いた当日だけ |
| 閉店時間(項目6で`Enable`) | `ShouldNotReact` | 静観 | 項目6を聞いた当日だけ |
| 裏口(項目7で`Enable`、項目9でdiscredit可) | `ShouldNotReact` | 静観 | discreditされると出現自体が止まる。項目7を聞いた当日だけ |
| 呼びかけ(奇数分限定) | `ShouldNotReact` | 静観 | |
| 呼ばれる(`IntrusionSlot`) | `ShouldNotReact` | 静観 | 押しても静観してもどちらも`resolve()`で判定されるが、正解は静観 |

**検印(Stamp)が正解になる行がなかった設計上の欠落は解消済み。** 通常業務行の正解を検印に
定めたことで、削除・静観の2択に収束していた状態を、3つの動詞すべてが拮抗する形に作り直した
(2026-07-20実装、`CLAUDE.md` §3.3・§9)。

## 2026-07-20 の設計変更(実装済み)

以下は方針決定・実装ともに完了した変更。`CLAUDE.md` §3.3・§3.4・§4・§9 に対応する記述あり。

- **タタリの統一(§3.3)。** 分類ごとに正解の動詞をちょうど1つ定義し(上表)、正解を外した
  場合の結果を分類問わず「corruption増加 + zone_bump」に一本化した。かつての `taboo` フラグ
  (一部の`ShouldReact`だけ見逃すとzone_bumpも立つ、という非対称な例外)は廃止した
  (`LogLine.taboo`/`Effect::Enable`の`taboo`引数/`Verdict::Active`の`taboo`ペイロード、いずれも
  削除済み)。表記の乱れ(「焼きあがり」)を噂を聞いた日だけ特別扱いする案は、この統一によって
  不要になったため見送った — 3基本異常はもともと常に`ShouldReact`であり、最初からこの表に
  乗っている
- **ルール効果の日次リセット(§3.4)。** `RuleLedger::reset_day()` が `heard` を丸ごとクリアする
  形で実装済み。`Discredit` を含め全効果が同じ扱いでリセットされる — 「聞いた効果はその日限り」
  という単純な規則に統一し、`Discredit`だけ例外にする特別扱いはしていない
- **リセット通知の診断ログ(§3.4)。** `domain::rule_reset_notice` が焼成室側の `day_marker` と
  対になる形で実装済み。日をまたぐたびに `screens::playing::spawn::phase_tick` が売り場の
  `pending_scripted` へ1件積む。「今日はいつも通りで大丈夫みたいですよ」等、接客の言い回しに
  留めた小さな語彙プールから選ぶ(第一稿、`CLAUDE.md` §9 に語彙拡充は今後の課題として記載)

## 未実装・方向性のみ決定の項目

- **禁忌同士の絡み合いの強化。** `Discredit` は現状1組(項目9→項目7)のみ。今後は「ある噂を
  信じるかどうかの判断そのものを、別の噂が後から覆す」構造をカタログ全体にもっと増やす方針
  (2026-07-20 方針決定、カタログ未反映)
