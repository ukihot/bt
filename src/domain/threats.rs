use rand::RngExt;
use rand::rngs::ThreadRng;

use super::clock::DayClock;
use super::log_line::{Classification, LogLine};
use super::timestamp::{even_minute_of, fill_n, timestamp};

/// 表記の乱れ: 「焼き上がり」ではなく「焼きあがり」と書く
fn deviation_spelling(clock: DayClock, rng: &mut ThreadRng) -> String {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..20);
    format!("{} バゲットが{}本 焼きあがり", timestamp(h, m), n)
}

/// 数え違い: バゲットを「本」ではなく「個」で数える
fn deviation_counter(clock: DayClock, rng: &mut ThreadRng) -> String {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..20);
    format!("{} バゲットが{}個 焼き上がり", timestamp(h, m), n)
}

/// 記載漏れ: 動詞を欠いた、書きかけの記録
fn deviation_missing_verb(clock: DayClock, rng: &mut ThreadRng) -> String {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..30);
    format!("{} 配達伝票を{}件", timestamp(h, m), n)
}

const DEVIATION_GENERATORS: &[fn(DayClock, &mut ThreadRng) -> String] =
    &[deviation_spelling, deviation_counter, deviation_missing_verb];

/// 焼成室固有の脅威: 店の筆癖(第4節)からの技術的な逸脱。内容だけを見れば
/// 正常な業務記録と区別がつかず、`ShouldReact`(正解は削除、`Classification::
/// correct_action`)固定 -- ログ単独で読み取れる異常なので、噂によって
/// 有効/無効が切り替わることもない(`rules::ThreatKind`に対応する変種を
/// 持たない、この2つのファイルが唯一関与しない異常)。
pub fn deviation_threat_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let generator = DEVIATION_GENERATORS[rng.random_range(0..DEVIATION_GENERATORS.len())];
    let text = generator(clock, rng);
    LogLine::new(text, Classification::ShouldReact)
}

/// 禁忌#2(夜の納品)・#8(裏口)共通の形: 時刻 + `{n}`入りのテンプレート +
/// 常に`ShouldNotReact`。この2つは本文と`n`の抽選範囲が違うだけで構造は
/// 同一なので、別々の関数に複製しない。
fn timestamped_taboo_line(
    clock: DayClock,
    body: &str,
    n_range: std::ops::Range<u32>,
    rng: &mut ThreadRng,
) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(n_range);
    let text = format!("{} {}", timestamp(h, m), fill_n(body, n));
    LogLine::new(text, Classification::ShouldNotReact)
}

/// 禁忌#2(夜の納品): 深夜便を受け取ったという焼成室の記録。噂を聞くまで
/// は `generate::weights_for` 側でこのバケットの重みが0なので、この関数
/// 自体そもそも呼ばれない -- 聞いた後、夜の時間帯にだけ現れる。
pub fn night_delivery_threat_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    timestamped_taboo_line(clock, "深夜便の配達を{n}件 受け取った", 1..10, rng)
}

/// 禁忌#8(裏口): 誰かが裏口の様子を見に行ったという、外画面の定点報告。
pub fn back_door_threat_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    timestamped_taboo_line(clock, "裏口の様子を{n}回 確認しに行った", 1..5, rng)
}

/// 禁忌#7(閉店時間): 客が閉店時間を尋ねてくる、売り場の応対記録の体裁。
/// `{n}`を持たないので`timestamped_taboo_line`は使わない(空の範囲を渡す
/// ためだけの`rng`を要求するのは無駄なコード)が、時刻を付けるのは他の
/// 売り場行と同じ -- 抜けていたのは単なる漏れで、意図した違いではない。
pub fn closing_time_threat_line(clock: DayClock) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let text = format!("{} お客様が「閉店は何時ですか」とお尋ねになりました", timestamp(h, m));
    LogLine::new(text, Classification::ShouldNotReact)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deviation_threat_line_is_always_legible_and_always_react() {
        let mut rng = rand::rng();
        for _ in 0..200 {
            let line = deviation_threat_line(DayClock::at(11, 30), &mut rng);
            assert_eq!(line.classification, Classification::ShouldReact);
            assert!(!line.text.is_empty());
        }
    }

    #[test]
    fn night_delivery_threat_line_is_should_not_react_and_carries_a_timestamp() {
        let mut rng = rand::rng();
        let line = night_delivery_threat_line(DayClock::at(23, 0), &mut rng);
        assert_eq!(line.classification, Classification::ShouldNotReact);
        assert!(line.text.starts_with("23:"));
    }

    #[test]
    fn closing_time_threat_line_is_should_not_react_and_carries_a_timestamp() {
        let line = closing_time_threat_line(DayClock::at(15, 30));
        assert_eq!(line.classification, Classification::ShouldNotReact);
        assert!(line.text.starts_with("15:30"));
    }

    #[test]
    fn back_door_threat_line_is_should_not_react_and_carries_a_timestamp() {
        let mut rng = rand::rng();
        let line = back_door_threat_line(DayClock::at(3, 0), &mut rng);
        assert_eq!(line.classification, Classification::ShouldNotReact);
        assert!(line.text.starts_with("03:"));
    }
}
