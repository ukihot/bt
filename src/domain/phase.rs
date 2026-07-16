#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Prep,
    Morning,
    Peak,
    Evening,
    Night,
}

impl Phase {
    pub fn next(self) -> Self {
        match self {
            Phase::Prep => Phase::Morning,
            Phase::Morning => Phase::Peak,
            Phase::Peak => Phase::Evening,
            Phase::Evening => Phase::Night,
            Phase::Night => Phase::Prep,
        }
    }

    pub fn duration_secs(self) -> f32 {
        match self {
            Phase::Prep => 18.0,
            Phase::Morning => 22.0,
            Phase::Peak => 26.0,
            Phase::Evening => 20.0,
            Phase::Night => 16.0,
        }
    }

    pub fn spawn_interval_secs(self) -> f32 {
        match self {
            Phase::Prep => 3.2,
            Phase::Morning => 2.2,
            Phase::Peak => 1.1,
            Phase::Evening => 1.8,
            Phase::Night => 2.6,
        }
    }

    pub fn hour_range(self) -> (u32, u32) {
        match self {
            Phase::Prep => (3, 6),
            Phase::Morning => (7, 10),
            Phase::Peak => (11, 13),
            Phase::Evening => (17, 19),
            Phase::Night => (21, 23),
        }
    }
}
