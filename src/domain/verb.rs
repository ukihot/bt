/// The player's two actions -- both are marks applied to a line while it's
/// still on screen; neither one resolves anything by itself (see
/// `screens::playing::pending::Pending`). 静観 (watching) isn't a verb -- it's
/// just what happens when a line exits unmarked, represented elsewhere as
/// `None`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verb {
    /// 削除. Marked with a strikethrough; resolves as deletion on exit.
    Delete,
    /// 検印. Marked with a stamp; resolves as an approving pass on exit.
    Stamp,
}

impl Verb {
    pub fn label(self) -> &'static str {
        match self {
            Verb::Delete => "削除",
            Verb::Stamp => "検印",
        }
    }
}
