use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Sub};

const HOUR: i32 = 60;
const DAY: i32 = 24 * HOUR;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let tot_minutes = ((hours * HOUR + minutes) % DAY + DAY) % DAY;
        Clock {
            hours: tot_minutes / HOUR,
            minutes: tot_minutes % HOUR,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", &self.hours, &self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

/** BONUS PART **/
/* Clock + Clock */
impl Add<Clock> for Clock {
    type Output = Self;

    fn add(self, other: Clock) -> Self::Output {
        Clock::new(self.hours + other.hours, self.minutes + other.minutes)
    }
}

/* Clock - Clock */
impl Sub<Clock> for Clock {
    type Output = Self;

    fn sub(self, other: Clock) -> Self::Output {
        Clock::new(self.hours - other.hours, self.minutes - other.minutes)
    }
}

/* Clock + minutes: i32 */
impl Add<i32> for Clock {
    type Output = Self;

    fn add(self, other: i32) -> Self::Output {
        Clock::new(self.hours, self.minutes + other)
    }
}

/* Clock - minutes: i32 */
impl Sub<i32> for Clock {
    type Output = Self;

    fn sub(self, other: i32) -> Self::Output {
        Clock::new(self.hours, self.minutes - other)
    }
}