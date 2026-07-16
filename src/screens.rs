//! One module per `AppState` screen. Each owns its own components,
//! resources, and systems, and is wired into the app as a self-contained
//! `Plugin`.

pub mod lost;
pub mod playing;
pub mod title;

pub use lost::LostPlugin;
pub use playing::PlayingPlugin;
pub use title::TitlePlugin;
