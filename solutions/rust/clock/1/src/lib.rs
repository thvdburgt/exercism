use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: u32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = hours.rem_euclid(24) as u32;
        let mut minutes = minutes.rem_euclid(24 * 60) as u32;

        minutes += hours * 60;
        minutes %= 24 * 60;

        Self { minutes }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Clock {
        Self::new(0, self.minutes as i32 + minutes)
    }

    pub fn hours(&self) -> u32 {
        self.minutes / 60
    }

    pub fn minutes(&self) -> u32 {
        self.minutes % 60
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
