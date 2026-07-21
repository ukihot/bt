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

`heard` を**新しい順**に走査し、`threat` に言及する最初の `Enable`/`Void` を採用する。

```
for id in heard.rev():
    match effect:
        Enable{pane, threat} if matches → Active
        Void{threat, condition} if matches and condition holds → Suppressed
一致なし:
    ThreatKind::Repeat → Active                       # 初日から反応対象の既定異常
    ThreatKind::ItemMiscount(HotDog) → Active          # 同じく初日から反応対象
    それ以外 → Suppressed                              # 噂で Enable されるまで起こらない
```

`ItemMiscount(ItemKind)` の既定値が品目によって違う(`HotDog` だけ `Active`、他は
`Suppressed`)のは意図的な非対称 — 詳しくは下記「品目の数え違い」参照。

「矛盾するルールが出た場合は新しいものが優先される」(`CLAUDE.md` §3.4)を、噂の種類が増えても
ペアごとのハードコードなしに実現している。`Verdict` はもう `taboo` ペイロードを持たない単純な
`Active`/`Suppressed` の2値 — 分類ごとの正解を外せば必ず「タタリ」になる仕様に統一されたことで
(下記「正解アクション定義表」参照)、個別の重み付けが不要になった(2026-07-20)。

### `reset_day()` — 日次リセット

`RuleLedger::reset_day()` は `heard` を丸ごとクリアするだけの操作(`Cast` には触れない)。
共有クロックが日をまたぐたびに `screens::playing::spawn::phase_tick` から呼ばれ、聞いた噂の
効果はすべてその日限りで消える — 翌日また同じ噂を聞き直さない限り、
`Enable`/`Void`/`Relieve` のどれも「聞かなかったこと」に戻る(2026-07-20実装)。

### `relief_bonus(threat) -> f32`

同じ新しい順スキャンで `Effect::Relieve` を探す姉妹クエリ。`Relieve` は `Active`/`Suppressed`
とは独立した軸 — 援助系の禁忌がその脅威を「反応可能」にするわけではなく、既に反応可能な脅威を
正しく処理したときの corruption 回復を上乗せするだけ。

### `Cast` — 話者はランごとにランダム割り当て

禁忌の内容と機械的効果(`Effect`)はカタログ固定データだが、**誰が言うか**は `RuleLedger::new()`
のたびに `rand::rng()` で再抽選される(`Cast::roll`)。「同じ噂は毎回同じ客が言う」という覚える
だけのパターンにならないための設計(ローグライク原則、`CLAUDE.md` §3.6: ラン間で持ち越すのは
知識だけ)。

> **2026-07-21: `Effect::Discredit` を削除。** 「◯◯の噂をした人物は嘘つきだ」型の効果として
> 唯一実装されていた項目9→項目7(裏口)の discredit ペアは、対象の裏口の噂ごと削除した(下記
> 「操作対象の範囲」参照)。実例のない `Discredit` だけをコードに残す理由がなかったため、
> `is_discredited` の解決ロジックと `Cast::roll` の discredit 避け再抽選も含めて機構ごと
> 削除している。禁忌同士の絡み合いという方向性自体は今後も検討の余地があるが、現状は
> `CATALOG` に実例なし。

## `Effect` の種類(3種)

| Effect | 効果 |
|---|---|
| `None` | 純粋な伏線。聞いても機械的には何も変わらない(奇数分・二度聞き・名指し・焼き上がりの字、など常に真の禁忌を予告するだけの項目) |
| `Enable { pane, threat }` | 聞いたその日のうちだけ、`threat` が `pane` で反応対象になる(`taboo` パラメータは2026-07-20に廃止 — 下記「正解アクション定義表」参照) |
| `Void { threat, condition }` | 聞いたその日のうちだけ、`condition` が成立する間 `threat` は反応対象でなくなる |
| `Relieve { threat, bonus }` | 聞いたその日のうちだけ、`threat` を正しく処理すると追加で `bonus` だけ corruption を回復する(援助系) |

すべての効果が「聞いたその日のうちだけ」なのは `RuleLedger::reset_day()` による日次リセットの
結果(上記参照)。翌日も効かせたい場合は、その日また同じ噂を聞き直す必要がある。

`Condition` は現状 `DayHasThree`(日数に3を含む日、例: 3, 13, 23...)の1種のみ。

## 禁忌集カタログ(全11項目、`src/domain/rumors.rs`)

売り場の客の噂話として、ランダムに1件ずつ流れる(`rumor_line`)。プレイスホルダーの `{name}`
はその噂の話者名(`Cast` が決める)で埋められる。すべて焼成室(`Pane::Kiln`)向けの効果——
`Outside`/`Floor` はもう操作対象ではないため(`CLAUDE.md` §3.2)、この2画面固有だった項目
(旧6〜9: 閉店時間・裏口・外の異常なし・裏口discredit)は2026-07-21に削除した。同日、代わりに
品目(クロワッサン/塩パン/ホットドッグ/食パン)ごとの数え違いを4項目追加した——詳細は下記
「品目の数え違い」参照。

| # | 本文の要旨 | 効果 |
|---|---|---|
| 0 | 奇数分に帳面をつけると良くないらしい | `None`(伏線 — 焼成室の偶数分丸めの逸脱、呼びかけの奇数分の予告) |
| 1 | 夜の納品だけは断ったほうがいい | `Enable{Kiln, NightDelivery}` |
| 2 | 三のつく日は窯の火を落とさないほうがいい | `Void{Repeat, DayHasThree}` |
| 3 | 同じ話を二度されても続けて聞かないほうがいい | `None`(伏線) |
| 4 | 名前を呼ばれてもすぐには信じないほうがいい | `None`(伏線 — 「呼ばれる」演出の予告) |
| 5 | 焼き上がりの字だけは崩さないでほしい | `None`(伏線 — 「焼き上がり」表記の逸脱の予告。3基本異常はもともと常に`ShouldReact`なので、この噂は機械的な効果を持たない) |
| 6 | 帳面の同じ行に気づいたらすぐに消すと楽になるらしい | `Relieve{Repeat, bonus:3.0}`(援助系) |
| 7 | クロワッサンの数が違っていたら、ちゃんと消したほうがいい | `Enable{Kiln, ItemMiscount(Croissant)}` |
| 8 | 塩パンだけは、数え間違いを見逃さないほうがいい | `Enable{Kiln, ItemMiscount(ShioPan)}` |
| 9 | 食パンの数だけは、間違えたままにしないほうがいい | `Enable{Kiln, ItemMiscount(MilkLoaf)}` |
| 10 | ホットドッグの数は、三のつく日だけ気にしなくていい | `Void{ItemMiscount(HotDog), DayHasThree}` |

禁忌集の項目リスト = 実質のレベルデザイン。日ごとに一項ずつ意味を持ち始める構成
(`CLAUDE.md` §5)。項目 6 だけが唯一の援助系(`Effect::Relieve`)で、`Zone` を一切動かさず
corruption の回復だけを厚くする。

## `ThreatKind`(6種、`src/domain/rumors.rs`)

| ThreatKind | 既定状態 | 出現画面 |
|---|---|---|
| `Repeat` | 初日から `Active`(焼成室の既定の反復。項目2で voidable) | Kiln |
| `NightDelivery` | 既定 `Suppressed`(項目1で `Enable`) | Kiln、かつ夜のフェーズのみ |
| `ItemMiscount(Croissant)` | 既定 `Suppressed`(項目7で `Enable`) | Kiln |
| `ItemMiscount(ShioPan)` | 既定 `Suppressed`(項目8で `Enable`) | Kiln |
| `ItemMiscount(MilkLoaf)` | 既定 `Suppressed`(項目9で `Enable`) | Kiln |
| `ItemMiscount(HotDog)` | 初日から `Active`(項目10で三のつく日だけ voidable) | Kiln |

かつてあった `OutsideRepeat`/`ClosingTime`/`BackDoor`(禁忌#9/#7/#8)は2026-07-21に削除した
——`Outside`/`Floor` はもう操作対象ではなく、削除/検印/静観の「正解」を確かめる手段が存在しない
ため(`CLAUDE.md` §3.2・§9「操作対象の範囲」)。

### 品目の数え違い(`ItemMiscount`) — 旗揚げゲームの混乱を品目ごとに再現する

2026-07-21追加(`CLAUDE.md` §4・§5)。`domain::rumors::ItemKind` がバゲット以外の主要4品目
(クロワッサン・塩パン・ホットドッグ・食パン)を表し、それぞれ独立した `ThreatKind::
ItemMiscount(ItemKind)` を持つ——「パンの数え違い」という1つの脅威ではなく、品目の数だけ
別々の `Enable`/`Void` 履歴を持つ。文面は `generate::item_miscount_line` が
`item.wrong_counter()`(実在する助数詞だが、その品目には間違っているもの)を使って生成し、
`Verdict` が `Active`/`Suppressed` のどちらでも**まったく同じ形**になる——分かるのは
テキストからではなく、その品目について聞いた噂を覚えているかどうかだけ。

品目ごとの既定状態(上表)もわざと揃えていない: `HotDog` だけ `Repeat` と同じく初日から
`Active`(項目10の噂を聞くまでは、三のつく日でも普段でも危険側)、残り3品目は
`NightDelivery` と同じく `Suppressed`(それぞれの `Enable` 噂を聞くまでは安全側)。旗揚げ
ゲームの「赤上げて、白上げて」——根幹のルールがどっちだったか一瞬わからなくなる感覚——を、
品目という店に元からある軸に同じ `Enable`/`Void` の仕組みを繰り返し適用することで再現する
のが狙い(`CLAUDE.md` §5「カタログの増やし方」)。

品目の**正しい**数え方(助数詞: クロワッサン=個, 塩パン=個, ホットドッグ=本, 食パン=斤)は
`generate::item_normal_line` が別途、通常業務行として流す——これがないと`item_miscount_line`
の「間違った数え方」を読み比べる基準がなく、第4節の「異常は筆癖でバレる」という前提そのものが
成立しない。

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
| 品目の数え違い(`ItemMiscount`, `Active`時) | `ShouldReact` | 削除 | 品目ごとに独立(上記「品目の数え違い」参照)。文面は`Active`/`Suppressed`で変わらない |
| 夜の納品(項目1で`Enable`) | `ShouldNotReact` | 静観 | 夜のフェーズのみ出現。項目1を聞いた当日だけ |
| 呼びかけ(奇数分限定) | `ShouldNotReact` | 静観 | |
| 呼ばれる(`IntrusionSlot`) | `ShouldNotReact` | 静観 | 押しても静観してもどちらも`resolve()`で判定されるが、正解は静観 |

上表はすべて**焼成室**の行にのみ適用される。`Outside`/`Floor` はもう操作対象ではないため
(`CLAUDE.md` §3.2)、この2画面の行に「正解」という概念自体が存在しない — 2026-07-21以降、
`screens::playing::spawn::line_spawn` はこの2画面の行を `Pending` から外す際に `resolve()` を
一切呼ばない(下記「操作対象の範囲」参照)。

**検印(Stamp)が正解になる行がなかった設計上の欠落は解消済み。** 通常業務行の正解を検印に
定めたことで、削除・静観の2択に収束していた状態を、3つの動詞すべてが拮抗する形に作り直した
(2026-07-20実装、`CLAUDE.md` §3.3・§9)。

## 2026-07-21 の設計変更(実装済み): 操作対象の範囲

`CLAUDE.md` §3.1〜§3.2 が定める「操作できるのは焼成室だけ」に実装を合わせた変更。詳細は
[07-implementation-notes.md](07-implementation-notes.md) 参照。ここでは禁忌集・`ThreatKind`
まわりへの影響だけ要約する:

- `Outside`/`Floor` 固有だった `ThreatKind`(`OutsideRepeat`/`ClosingTime`/`BackDoor`、
  禁忌#9/#7/#8)と、それらを有効化していた禁忌集の3項目、および禁忌#8を対象にしていた
  `Effect::Discredit`(項目9)を削除した
- `screens::playing::spawn::line_spawn` は `Outside`/`Floor` の行が `Pending` から外れても
  `domain::resolve()` を呼ばなくなった — 呼んでしまうと、操作不能なこの2画面の行は`mark`が
  常に`None`のままなので、`Normal`/`ShouldReact`のどちらであっても必ず「タタリ」と判定されて
  しまう(正解の動詞が検印/削除である以上、押しようがない画面では絶対に当たらない)
- `H`/`L` キーとその入力ハンドラ、`ActivePane` リソース、パネル見出しの選択表示(色ではなく
  `CURSOR_MARK`の切り替えで示していた)を削除した。`J`/`K`/`Z`/`X` は常に焼成室の `Pending`
  だけを操作する

## 2026-07-21 の設計変更(実装済み): 品目の数え違い

上記と同日、禁忌集のバリエーションを増やす方針として「パン屋の品目という店に元からある軸へ、
既存の `Enable`/`Void` の仕組みを繰り返し適用する」形を採用した(`CLAUDE.md` §5「カタログの
増やし方」)。詳細は上記「品目の数え違い」参照。要点のみ:

- `ItemKind`(`Croissant`/`ShioPan`/`HotDog`/`MilkLoaf`)と
  `ThreatKind::ItemMiscount(ItemKind)` を追加——品目ごとに独立した脅威として扱う
- 品目ごとに `Enable`/`Void` 噂を1つずつ(計4項目)追加し、`RuleLedger::verdict` の
  フォールバックだけ `HotDog` を特別扱いして初日から `Active` にした——他の3品目は
  `Suppressed` のまま、既定値をわざと揃えていない
- `generate::item_miscount_line`(`Verdict`に応じて分類だけ変わる、文面は不変)と
  `generate::item_normal_line`(品目の正しい数え方を示す通常業務行)を追加し、
  `Weights` に `item_miscount` バケットを新設した(`weights_for` は `Kiln` 以外で常に0)

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
  形で実装済み。全効果が同じ扱いでリセットされる — 「聞いた効果はその日限り」という単純な
  規則に統一している
- **リセット通知の診断ログ(§3.4)。** `domain::rule_reset_notice` が焼成室側の `day_marker` と
  対になる形で実装済み。日をまたぐたびに `screens::playing::spawn::phase_tick` が売り場の
  `pending_scripted` へ1件積む。「今日はいつも通りで大丈夫みたいですよ」等、接客の言い回しに
  留めた小さな語彙プールから選ぶ(第一稿、`CLAUDE.md` §9 に語彙拡充は今後の課題として記載)

## 未実装・方向性のみ決定の項目

- **禁忌同士の絡み合いの強化。** 「ある噂を信じるかどうかの判断そのものを、別の噂が後から
  覆す」構造(`Discredit`型)をカタログ全体に増やす方針自体は残っているが、唯一の実装例
  (項目9→項目7)は対象の裏口の噂ごと2026-07-21に削除したため、現状は方針のみで実装は0件
  (`CLAUDE.md` §9 参照)
