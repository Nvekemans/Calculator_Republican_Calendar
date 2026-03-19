use Revolutionary_Calendar_Calculator::date_calculator_romme;
use time::{Date, OffsetDateTime};

fn main() {
    let today = OffsetDateTime::now_utc().date();
    let republican_date = date_calculator_romme(today);
    println!("{}", republican_date.get_date_string());
    println!("Jour du {}", republican_date.get_object());
    println!("Nous sommes en {}", republican_date.get_season());
}
