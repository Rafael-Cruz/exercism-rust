use std::{fmt::Display, i32};

const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn compute_minutes(minutes: i32) -> (i32, i32) {
    let rem = minutes.rem_euclid(MINUTES_IN_HOUR);

    eprintln!("minutes: {minutes}, rem: {rem}");
    
    let hours = (minutes.div_euclid(MINUTES_IN_HOUR)).rem_euclid(HOURS_IN_DAY);
    let minutes = minutes.rem_euclid(MINUTES_IN_HOUR);

    (hours, minutes)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = compute_minutes(MINUTES_IN_HOUR * hours + minutes);
        Clock { hours, minutes }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        if minutes != 0 {
            let (hours, minutes) =
                compute_minutes((MINUTES_IN_HOUR * self.hours) + (self.minutes + minutes));
            self.hours = hours;
            self.minutes = minutes;
        }

        self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
