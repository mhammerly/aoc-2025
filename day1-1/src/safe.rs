use std::str::FromStr;

const DEFAULT_MIN: u64 = 0;
const DEFAULT_MAX: u64 = 99;

/// A turn on a safe dial. `L(99)` means a turn to the left 99 places.
pub enum Turn {
    L(u16),
    R(u16),
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::L(distance) => write!(f, "L{}", distance),
            Self::R(distance) => write!(f, "R{}", distance),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseTurnError {
    #[error(transparent)]
    InvalidDistance(#[from] std::num::ParseIntError),
    #[error("invalid direction")]
    InvalidDirection,
}

impl FromStr for Turn {
    type Err = ParseTurnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("L", distance) => Ok(Turn::L(distance.parse()?)),
            ("R", distance) => Ok(Turn::R(distance.parse()?)),
            _ => Err(ParseTurnError::InvalidDirection),
        }
    }
}

/// Stats about dial position.
#[derive(Default)]
pub struct DialStats {
    pub landed_on_min: u16,
}

/// A safe dial. When turning past `self.max`, `self.current` will overflow starting back at
/// `self.min`. Similarly, when turning past `self.min`, `self.current` will underflow starting
/// back at `self.max`.
pub struct Dial {
    pub current: u64,
    pub min: u64,
    pub max: u64,
    pub stats: DialStats,
}

impl Dial {
    pub fn new_with_range(start: u64, min: u64, max: u64) -> Dial {
        assert!(start >= min && start <= max);
        Dial {
            current: start,
            min,
            max,
            stats: Default::default(),
        }
    }

    pub fn new(start: u64) -> Dial {
        Dial::new_with_range(start, DEFAULT_MIN, DEFAULT_MAX)
    }

    pub fn turn(&mut self, turn: Turn) -> u64 {
        let old_current = self.current;
        match turn {
            Turn::R(distance) => self.add(distance.into()),
            Turn::L(distance) => self.sub(distance.into()),
        };
        tracing::debug!("Pos {} + {} = Pos {}", old_current, turn, self.current);
        self.current
    }

    #[inline]
    fn rollover(&self, i: u64) -> (u64, u64) {
        let range = self.max - self.min + 1;
        let full_rotations = i / range;
        let remainder = i % range;
        (full_rotations, remainder)
    }

    fn add(&mut self, i: u64) -> u64 {
        let (_full_rotations, remainder) = self.rollover(i);
        let space = self.max - self.current;

        if remainder <= space {
            self.current += remainder;
        } else {
            self.current = self.min + (remainder - space - 1);
        }

        if self.current == self.min {
            self.stats.landed_on_min += 1;
        }

        self.current
    }

    fn sub(&mut self, i: u64) -> u64 {
        let (_full_rotations, remainder) = self.rollover(i);
        let space = self.current - self.min;

        if remainder <= space {
            self.current -= remainder;
        } else {
            self.current = self.max - (remainder - space - 1);
        }

        if self.current == self.min {
            self.stats.landed_on_min += 1;
        }

        self.current
    }
}
