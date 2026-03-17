use time::Date;

use crate::republicanDate::RepublicanDate;

mod republicanDate;

// TO DO : MORE TESTS
// TO DO : Recuperation of today's date and conversion to republican date
// To DO : Nice way to display the date, maybe with emojis for the seasons and the objects of the day



pub fn date_calculator_romme(date : Date) -> RepublicanDate {
    // Implementation of the Republican calendar date calculation using the Romme method
    
    let start_date = Date::from_calendar_date(1792, time::Month::September, 22).unwrap();
    let mut days_since_start = (date - start_date).whole_days() + 1; // +1 because the start date is the first day of the Republican calendar
    let (year, day_of_year) = get_year(days_since_start as i32);

    let (month, day_of_month) = get_month(day_of_year as i32);

    return RepublicanDate::new(year, month, day_of_month);

}

fn get_year(mut days_since_start: i32) -> (i32, i32) {
    let mut year = 1;
    while days_since_start >= 365 {
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            println!(
                "Year {} is a leap year, days since start: {}",
                year, days_since_start
            );
            if days_since_start >= 366 {
                days_since_start -= 366;
                year += 1;
            } else {
                break;
            }
        } else {
            days_since_start -= 365;
            year += 1;
        }
    }

    return (year, days_since_start);
}

fn get_month(mut days_since_start: i32) -> (i32, i32) {
    let mut month = 1;
    while days_since_start > 30 {
        days_since_start -= 30;
        month += 1;
    }

    return (month, days_since_start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_year() {
        let (year, day_of_year) = get_year(76350);
        assert_eq!(year, 210);
        assert_eq!(day_of_year, 15);


        let (year, day_of_year) = get_year(3);
        assert_eq!(year, 1);
        assert_eq!(day_of_year, 3);

        let (year, day_of_year) = get_year(366);
        assert_eq!(year, 2);
        assert_eq!(day_of_year, 1);
    }

    #[test]
    fn test_get_month() {
        let (month, day_of_month) = get_month(30);
        assert_eq!(month, 1);
        assert_eq!(day_of_month, 30);

        let (month, day_of_month) = get_month(31);
        assert_eq!(month, 2);
        assert_eq!(day_of_month, 1);
    }

    #[test]
    fn test_date_calculator_romme() {
        let date = Date::from_calendar_date(2026, time::Month::March, 17).unwrap();
        let republican_date = date_calculator_romme(date);
        println!("{}", republican_date.get_day());
        assert_eq!(republican_date.get_date_string(), "Septidi de Ventôse de l'an 234");
    }
}