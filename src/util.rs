use chrono::prelude::*;
use chrono::Duration;

pub fn get_days_till_next_salary() -> u32 {
  let mut count: u32 = 0;
  let days_till_next_salary = loop {
    let dayoffset = Local::today() + Duration::days(count.into());

    if dayoffset.day() == 5 || dayoffset.day() == 20 { // temporary values 5 and 20
      break count + 1; // @note include the day today
    } else { count += 1; }
  };
  days_till_next_salary
}