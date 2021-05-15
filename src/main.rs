use std::env;
mod util;

fn main() {
  // START: Argument collection
  let args: Vec<String> = env::args().collect();
  let bank_balance = args[1].parse::<u32>();
  let wallet_balance = args[2].parse::<u32>();
  let payment_balance = args[3].parse::<u32>();
  let savings_balance = args[4].parse::<u32>();
  // END: Argument collection

  let remaining_money = bank_balance.unwrap() + wallet_balance.unwrap();
  let actual_money = remaining_money - (payment_balance.unwrap() + savings_balance.unwrap());
  let daily_budget = actual_money / util::get_days_till_next_salary();

  println!("Your Current Daily Budget is: {}", daily_budget);
}
