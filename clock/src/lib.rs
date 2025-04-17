use std::{fmt::Display, i32};

const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn get_hours_in_day(hours: i32) -> i32 {
    match hours {
        // Acha o valor congruente com a quantidade de horas em mod 24 (para que caiba em um dia),
        // e subtrai este valor do total de um dia (por isso o 24 - (...)). Ao final, aplica
        // novamente o módulo 24 para o caso de o resultado ter sido 24
        i32::MIN..=-1 => (HOURS_IN_DAY - (hours.abs() % HOURS_IN_DAY)) % HOURS_IN_DAY,
        _ => hours % HOURS_IN_DAY,
    }
}

fn compute_minutes(minutes: i32) -> (i32, i32) {
    let hours = (minutes as f32 / MINUTES_IN_HOUR as f32).floor() as i32;

    let minutes = if minutes < 0 {
        minutes + (MINUTES_IN_HOUR * hours.abs())
    } else {
        minutes % MINUTES_IN_HOUR
    };

    (get_hours_in_day(hours), minutes)
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
