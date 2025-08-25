use time::{PrimitiveDateTime as DateTime, ext::NumericalDuration};

const GIGASECOND: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + GIGASECOND.seconds()
}
