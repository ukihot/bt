#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Zone {
    Perimeter,
    Inside,
    Counter,
}

impl Zone {
    pub fn next(self) -> Self {
        match self {
            Zone::Perimeter => Zone::Inside,
            Zone::Inside | Zone::Counter => Zone::Counter,
        }
    }

    pub fn location_pool(self) -> &'static [&'static str] {
        match self {
            Zone::Perimeter => &["裏口", "駐車場", "外周の柵", "搬入口"],
            Zone::Inside => &["厨房", "客席", "通路", "倉庫"],
            Zone::Counter => &["帳場", "レジ横", "金庫の前", "この机"],
        }
    }
}
