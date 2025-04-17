use std::{fmt::Display, i32};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: u32,
    minutes: u32,
}

fn compute_hours(hours: i32) -> u32 {
    eprintln!("Computing hours for {hours:?}");
    let real_hours = match hours {
        i32::MIN..=-1 => 24 - (hours.abs() % 24) as u32,
        0..=23 => hours as u32,
        24..=i32::MAX => (hours % 24) as u32,
    };
    if real_hours == 24 { 0 } else { real_hours }
}

fn compute_minutes(minutes: i32) -> (u32, i32) {
    let diff_hours = minutes as f32 / 60_f32;

    eprintln!("first diff hours: {diff_hours:?}");

    let diff_hours: i32 = diff_hours.floor() as i32;

    let real_minutes = if minutes < 0 {
        (minutes + (60 * diff_hours.abs())) as u32
    } else {
        (minutes % 60) as u32
    };

    eprintln!(
        "Computing minutes for {minutes:?}: diff_hours: {diff_hours:?}, real_minutes: {real_minutes:?}"
    );

    (real_minutes, diff_hours)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (real_minutes, diff_hours) = compute_minutes(minutes);
        Clock {
            hours: compute_hours(compute_hours(hours) as i32 + diff_hours),
            minutes: real_minutes,
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        eprintln!("adding {minutes} minutes");
        if minutes != 0 {
            let (real_minutes, diff_hours) = compute_minutes(self.minutes as i32 + minutes);
            self.minutes = real_minutes;
            self.hours = compute_hours(self.hours as i32 + diff_hours);
        }

        self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
