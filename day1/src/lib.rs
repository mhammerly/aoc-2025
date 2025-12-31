use std::str::FromStr;

const DEFAULT_MIN: u64 = 0;
const DEFAULT_MAX: u64 = 99;

/// A turn on a [`Dial`]. `L(99)` means a turn to the left 99 places.
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

/// Error type for parsing [`Turn`]s.
#[derive(thiserror::Error, Debug)]
pub enum ParseTurnError {
    /// Indicates that the distance component of a turn couldn't be parsed.
    #[error(transparent)]
    InvalidDistance(#[from] std::num::ParseIntError),

    /// Indicates that a turn's direction is not recognized.
    #[error("invalid direction; expected 'L' or 'R', got '{0}'")]
    InvalidDirection(String),
}

impl FromStr for Turn {
    type Err = ParseTurnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("L", distance) => Ok(Turn::L(distance.parse()?)),
            ("R", distance) => Ok(Turn::R(distance.parse()?)),
            (direction, _distance) => Err(ParseTurnError::InvalidDirection(direction.into())),
        }
    }
}

/// Stats about dial position.
#[derive(Default, Debug)]
pub struct DialStats {
    pub landed_on_min: u16,
    pub touched_min: u64,
}

/// A safe dial. When turning past `self.max`, `self.current` will overflow starting back at
/// `self.min`. Similarly, when turning past `self.min`, `self.current` will underflow starting
/// back at `self.max`.
pub struct Dial {
    /// Current position of the dial. Must be between `self.min` and `self.max`.
    pub current: u64,

    /// Minimum value on the dial. A left turn that goes past this number will wrap around and
    /// decrease beginning from `self.max`.
    pub min: u64,

    /// Maximum value on the dial. A right turn that goes past this number will wrap around and
    /// increase beginning from `self.min`.
    pub max: u64,

    /// Stats about the dial's position as turns are applied.
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

    /// Apply a [`Turn`] to this dial to update the dial position.
    pub fn turn(&mut self, turn: Turn) -> u64 {
        let old_current = self.current;
        match turn {
            Turn::R(distance) => self.add(distance.into()),
            Turn::L(distance) => self.sub(distance.into()),
        };
        tracing::debug!("Pos {} + {} = Pos {}", old_current, turn, self.current);
        tracing::debug!("{:?}", self.stats);
        self.current
    }

    #[inline]
    fn rollover(&self, i: u64) -> (u64, u64) {
        let range = self.max - self.min + 1;
        let full_rotations = i / range;
        let remainder = i % range;
        tracing::trace!(
            "Distance {i} is {full_rotations} full rotations and {remainder} additional ticks"
        );
        (full_rotations, remainder)
    }

    fn add(&mut self, i: u64) -> u64 {
        let (full_rotations, remainder) = self.rollover(i);
        let space = self.max - self.current;

        // If we turn 350 ticks, no matter what we will touch min 3 times
        self.stats.touched_min += full_rotations;

        if remainder <= space {
            self.current += remainder;
        } else {
            tracing::debug!("Overflow, will definitely pass {}", self.min);
            self.stats.touched_min += 1;
            self.current = self.min + (remainder - space - 1);
        }

        if self.current == self.min {
            tracing::debug!("Landed on {}", self.min);
            self.stats.landed_on_min += 1;
        }

        self.current
    }

    fn sub(&mut self, i: u64) -> u64 {
        let (full_rotations, remainder) = self.rollover(i);
        let space = self.current - self.min;

        // If we turn 350 ticks, no matter what we will touch min 3 times
        self.stats.touched_min += full_rotations;

        // If we land on the min (remainder == space) or overflow (remainder > space) we
        // will touch min once. However, if we are starting from min, we've already counted
        // this instance so we should skip it.
        if remainder >= space && self.current != self.min {
            self.stats.touched_min += 1;
        }

        if remainder <= space {
            self.current -= remainder;
        } else {
            tracing::debug!("Underflow");
            self.current = self.max - (remainder - space - 1);
        }

        if self.current == self.min {
            tracing::debug!("Landed on {}", self.min);
            self.stats.landed_on_min += 1;
        }

        self.current
    }
}
