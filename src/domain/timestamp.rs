use rand::RngExt;
use rand::rngs::ThreadRng;

use super::phase::Phase;

pub fn even_minute(rng: &mut ThreadRng) -> u32 {
    rng.random_range(0..30) * 2
}

pub fn odd_minute(rng: &mut ThreadRng) -> u32 {
    even_minute(rng) + 1
}

pub fn hour_for(phase: Phase, rng: &mut ThreadRng) -> u32 {
    let (lo, hi) = phase.hour_range();
    rng.random_range(lo..=hi)
}

pub fn timestamp(hour: u32, minute: u32) -> String {
    format!("{:02}:{:02}", hour, minute)
}

pub fn fill_n(template: &str, n: u32) -> String {
    template.replace("{n}", &n.to_string())
}
