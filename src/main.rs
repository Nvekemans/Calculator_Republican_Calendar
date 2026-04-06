use revolutionary_calendar_calculator::republican;
use time::OffsetDateTime;

fn main() {
    let today = OffsetDateTime::now_utc().date();
    let republican_date = republican::date_calculator_romme(today);
    println!("{}", republican_date);
    print_object_of_the_day(&republican_date);
}

fn print_object_of_the_day(republican_date: &republican::date::RepublicanDate) {
    let object_of_the_day = republican::DataCalendar::new("src/data.json")
        .get_object(*republican_date)
        .unwrap();

    if republican_date.get_month() == republican::month::RepublicanMonth::Sansculottides {
        println!(
            "Aujourd'hui, c'est le jour de la fête de {} !",
            object_of_the_day
        );
    } else {
        match republican_date.get_day() {
            
            day if day.is_multiple_of(10) => println!(
                "L'outil du jour est : {} !",
                object_of_the_day
            ),

            day if day.is_multiple_of(5) => println!(
                "L'animal du jour est : {} !",
                object_of_the_day
            ),

            _ => println!("Nous célébrons {} aujourd'hui !", object_of_the_day),
        }
    }
}
