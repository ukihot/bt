use bevy::prelude::*;

pub const BG: Color = Color::srgb(0.04, 0.04, 0.04);
pub const FG: Color = Color::srgb(0.85, 0.83, 0.78);
pub const DIM: Color = Color::srgb(0.45, 0.44, 0.42);

/// The three panes' shared fill -- a bare hint of green over `BG`, just
/// enough to read each monitor as its own screen (第8節: 色による強調は禁止
/// なので、選別なく3画面すべてに同一値を敷くだけに留める)。The delta from
/// `BG` has to survive 8-bit quantization to actually render as distinct --
/// a difference of only a couple sRGB units per channel rounds away to
/// nothing on screen.
pub const MONITOR_BG: Color = Color::srgb(0.05, 0.10, 0.06);
