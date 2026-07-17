# 3. 一日の構成と共有クロック

出典: `CLAUDE.md` §3.6, §3.7, §6、実装 `src/domain/{clock,phase,zone}.rs` `src/game_data.rs`
`src/screens/playing/{spawn,corruption}.rs`

## 共有クロック(店にひとつだけの時刻)

3画面は同じ店を同時刻に映す別々の定点観測であるため、「いま何時か」は店にひとつしかない
(`DayClock`, `src/domain/clock.rs`)。各画面はこの単一の時刻を、自分の書き手の癖に合わせて
書き写しているだけで、画面ごとに勝手な時刻を持ち出さない。

- 24時間が実時間4分で経過する固定レート(`GAME_MINUTES_PER_REAL_SECOND = 6.0`)
- `advance(real_seconds)` が一方向にのみ時刻を進め、日をまたいだ回数(通常0、稀に1)を返す
- 各画面の実際のスポーン頻度(`Pane::spawn_interval_secs`)は画面ごとに異なるが、これは
  「クロックを覗く頻度」の違いであって、クロックそのものが画面ごとに違うわけではない
- **焼成室の偶数分丸め**: 焼成室の通常行は真の時刻が奇数分であっても偶数分に丸めて記帳される
  (`timestamp::even_minute_of`)。奇数分がそのまま出た場合、それ自体が異常のサイン
  ([05-threats-and-tells.md](05-threats-and-tells.md) の呼びかけ参照)であり、クロックの不具合ではない

## フェーズ(`Phase`, `src/domain/phase.rs`)

`DayClock` の現在時刻から一意に逆算される、独自タイマーを持たない導出値。

| フェーズ | 時間帯 | 概況(`CLAUDE.md` §6) |
|---|---|---|
| 仕込み | 0–7時 | 焼成室・売り場とも静か。禁忌集や筆癖を「読む」時間。外の「異常なし」もまだ揺るがない |
| 開店(Morning) | 7–11時 | 焼成室の通常業務行が主。経済が回る。軽度の異常が混ざり始める |
| ピーク | 11–14時 | 売り場の流速が最大。ルール変更の頻度も上がる。焼成室も負荷が高い。外は変わらず静か — 油断が生まれる |
| 閉店(Evening) | 14–19時 | 焼成室の流速が落ち、売り場のルール変更は減るが、外の報告が不穏な方向へ動き始める |
| 夜(Night) | 19–24時 | 外の変化に気づきにくい時間帯。呼ばれるが起こり得るのもこの時間帯([06-narrative-beats.md](06-narrative-beats.md)) |

`hour_range()` が24時間を隙間なく分割しており、`for_hour(hour)` はこの分割だけを情報源に
フェーズを導出する。フェーズ切り替え時、各画面の `PaneRuntime::retime` がそのフェーズの
スポーン間隔で自分のタイマーを再設定する(`src/screens/playing/spawn.rs:79-85`)。

## 日をまたぐ構造(ローグライク的なエスカレーション)

`phase_tick`(`spawn.rs:63`)が `DayClock::advance` の戻り値 `wraps` の回数だけ、日替わりの
処理を回す:

1. `game_data.day += 1`
2. `game_data.zone = zone.next()` — 脅威の距離が1段階進む(下記 `Zone` 参照)
3. 焼成室にだけ日替わりの記帳(`domain::day_marker`)が流れる(スクリプト行、カーソル到達不可)

日を追うごとに、外の報告の不穏度の基準線が一段上がり、売り場のルール変更もより頻繁・複雑に
なる(原則2)。焼成室に集中するほど、ルールの書き換えと距離の変化から目が離れていく — その
皮肉自体が終盤の難易度になる。

## `Zone`(脅威の物理的な距離)

`src/domain/zone.rs`。`Perimeter`(外周)→ `Inside`(内側)→ `Counter`(帳場)の一方向にのみ
進む3段階。`Pane::matches_zone` を介して「今どの画面が狙われやすいか」の重み付けに反映される
(`src/domain/generate.rs` の `zone_bonus`)。

`Zone` が進むのは以下の2経路のみ:
- 日をまたいだとき(自動で1段階)
- `Outcome.zone_bump == true` のとき — 分類ごとの正解([02-screens-and-controls.md](02-screens-and-controls.md#反応の三分類-分類ごとに正解はちょうど1つそれ以外はタタリ)
  参照)を外した「タタリ」はすべて`zone_bump`を伴う(`src/screens/playing/spawn.rs` の `apply_outcome`)。
  2026-07-20より前は一部の`ShouldReact`だけがこれを伴う非対称な仕様だったが、現在はどの分類でも
  正解を外せば必ずここを通る

`Zone::Counter` に達した状態で夜かつ奇数分が揃うと、「呼ばれる」の条件が満たされる
([06-narrative-beats.md](06-narrative-beats.md))。

## ゲームオーバー(侵食度)

侵食度 `corruption`(0.0–100.0、3画面共有の単一値、プレイヤーには数値を一切見せない)は
`resolve()` の `Outcome.corruption` を毎回加算し、0–100にクランプして管理される
(`GameData.corruption`, `apply_outcome`)。`corruption_check`(`src/screens/playing/corruption.rs`)
が毎フレーム100到達を監視し、到達した瞬間 `AppState::Lost` へ遷移する。

`Lost` 画面(`src/screens/lost.rs`)では、ログが「別の何かの記録」に置き換わる演出
(`domain::corrupted_line` — 唯一、共有クロックを読まず時刻を独立乱数で決める関数)を9秒間流し、
無言でタイトルへ戻る。勝敗UI・クリア演出は一切ない(原則5: 終わらない)。

## 経済(`income`)

正常なログを検分し続けると収入が積み上がり、誤操作・見逃しは収入を減らす(`log_line::resolve`
の `income` フィールド)。ただし `CLAUDE.md` の「目標表示禁止」原則に従い、UIには一切表示されず、
現状は侵食度への直接的なフィードバックも未実装(`CLAUDE.md` §9)。
