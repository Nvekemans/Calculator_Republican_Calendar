use revolutionary_calendar_calculator::republican;
use time::OffsetDateTime;

fn main() {
    let today = OffsetDateTime::now_utc().date();
    let republican_date = republican::date_calculator_romme(today);
    println!("{}", republican_date);
    println!(
        "L'objet du jour est : {}",
        republican::DataCalendar::new("src/data.json")
            .get_object(republican_date)
            .unwrap()
    );
}
