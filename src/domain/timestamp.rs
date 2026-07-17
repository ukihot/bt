use super::clock::DayClock;

/// The baker's writing habit (CLAUDE.md §3.7/§4): normal 焼成室 lines round
/// the shared clock's true minute down to the nearest even value. This is a
/// presentation-layer rounding on top of one real clock, not a property of
/// the clock itself -- an odd minute surviving this rounding is what makes a
/// deviation line legible as a deviation.
pub fn even_minute_of(clock: DayClock) -> u32 {
    let m = clock.minute();
    m - (m % 2)
}

pub fn timestamp(hour: u32, minute: u32) -> String {
    format!("{:02}:{:02}", hour, minute)
}

pub fn fill_n(template: &str, n: u32) -> String {
    template.replace("{n}", &n.to_string())
}

pub fn fill_name(template: &str, name: &str) -> String {
    template.replace("{name}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_minute_of_rounds_down_to_the_nearest_even_minute() {
        assert_eq!(even_minute_of(DayClock::at(7, 30)), 30);
        assert_eq!(even_minute_of(DayClock::at(7, 31)), 30);
        assert_eq!(even_minute_of(DayClock::at(7, 0)), 0);
    }
}
