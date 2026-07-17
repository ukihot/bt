//! The game's simulation rules: what a line of log text is, how it's
//! generated, and what happens when the player reacts (or doesn't) to it.
//! Deliberately free of Bevy ECS types (no `Component`/`Resource`/systems)
//! so the rules here can be unit-tested without spinning up an `App`.

mod clock;
mod generate;
mod log_line;
mod pane;
mod phase;
mod threats;
mod timestamp;
mod verb;
mod zone;

pub use clock::DayClock;
pub use generate::{corrupted_line, day_marker, generate, mistake_beat, name_call};
pub use log_line::{Classification, LineFont, LogLine, Outcome, resolve};
pub use pane::Pane;
pub use phase::Phase;
pub use verb::Verb;
pub use zone::Zone;
