use chrono::prelude::*;
use chrono::Duration;

pub fn get_days_till_next_salary() -> u32 {
  let mut count: u32 = 1; // @note start with 1 to include today
  let days_till_next_salary = loop {
    let dayoffset = Local::today() + Duration::days(count.into());
    println!("{}, {}, {}", count, Local::today().day(), dayoffset.day());
    /* @todo
     * make salary dates dynamic by making including it in the arguments
     */
    if dayoffset.day() == 5 || dayoffset.day() == 20 {
      break count + 1;
    } else { count += 1; }
  };
  days_till_next_salary
}
