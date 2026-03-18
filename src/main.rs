use Revolutionary_Calendar_Calculator::date_calculator_romme;
use time::Date;

fn main() {
    let date = Date::from_calendar_date(2026, time::Month::March, 18).unwrap();
    let start_date = Date::from_calendar_date(1792, time::Month::September, 22).unwrap();
    let days_since_start = (date - start_date).whole_days() + 1;
    println!("Days since the start of the Republican calendar: {}", days_since_start);
    let republican_date = date_calculator_romme(date);
    println!("{}", republican_date.get_date_string());
}
