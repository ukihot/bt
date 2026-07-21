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

/// 禁忌#2(夜の納品)専用の形: 時刻 + `{n}`入りのテンプレート + 常に
/// `ShouldNotReact`。焼成室以外にはもう反応禁止型の脅威がないため
/// (2026-07-21、`Outside`/`Floor`固有だった#7・#8は削除済み。CLAUDE.md
/// §9参照)、汎用ヘルパーとしては残していない -- 今後また同じ形の脅威が
/// 増えたら、その時点で切り出す。
pub fn night_delivery_threat_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..10);
    let text = format!("{} {}", timestamp(h, m), fill_n("深夜便の配達を{n}件 受け取った", n));
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
}
