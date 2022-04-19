use std::fmt;
use std::fmt::Formatter;

const HOUR: i32 = 60;
const DAY: i32 = 24 * HOUR;


#[derive(Debug, PartialEq, Eq)]
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