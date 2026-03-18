//! This library provides functions to convert a Gregorian date to a Republican date using the Romme method. 
//! It also includes a struct to represent the Republican date and methods to display it in a nice format.
use time::Date;

use crate::republicanDate::RepublicanDate;

mod republicanDate;
// TO DO : MORE TESTS
// TO DO : Recuperation of today's date and conversion to republican date
// To DO : Nice way to display the date, maybe with emojis for the seasons and the objects of the day



//** Calculates the Republican date using the Romme method. It takes a [time::Date] as input and returns a [RepublicanDate]. */
pub fn date_calculator_romme(date : Date) -> RepublicanDate {
    
    let start_date = Date::from_calendar_date(1792, time::Month::September, 22).unwrap();   // The start date of the Republican calendar is September 22, 1792
    let days_since_start = (date - start_date).whole_days() + 1; // +1 because the start date is the first day of the Republican calendar
    
    let (year, day_of_year) = get_year(days_since_start as i32);

    let (month, day_of_month) = get_month(day_of_year);

    return RepublicanDate::new(year, month, day_of_month);

}


//** Returns the year and day of the year for a given number of days since the start date. */
fn get_year(mut days_since_start: i32) -> (i32, i32) {

    let mut year = 1;
    while days_since_start > 365 { // A year has 365 days, but we need to check for leap years
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {  // The year is a leap year if it is divisible by 4, but not by 100 unless it is also divisible by 400
            if days_since_start > 366 {
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


//** Returns the month and day of the month for a given number of days since the start of the year. */
fn get_month(mut days_since_start: i32) -> (i32, i32) {
    let mut month = 1;
    while days_since_start > 30 {   // A month has always 30 days in the Republican calendar. The last 5 or 6 days of the year are the "Sans-culottides" and are here considered as the the 13th month.
        month += 1;
        days_since_start -= 30;
    }

    return (month, days_since_start);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_year() {

        let (year, day_of_year) = get_year(3);  // Test the third days of the first year
        assert_eq!(year, 1);
        assert_eq!(day_of_year, 3);

        let (year, day_of_year) = get_year(366);    // The first leap year is year 4, so the 366th day is the first day of year 2
        assert_eq!(year, 2);
        assert_eq!(day_of_year, 1);

        let (year, day_of_year) = get_year(365);    // The 365th day is the last day of year 1
        assert_eq!(year, 1);
        assert_eq!(day_of_year, 365);

        let (year, day_of_year) = get_year(365 * 4 + 1);    // The last day of year 4, which is a leap year
        assert_eq!(year, 4);
        assert_eq!(day_of_year, 366);

        let (year, day_of_year) = get_year(365 * 4 + 2);    // The first day of year 5, which because of the leap year, is the 366th day of year 4
        assert_eq!(year, 5);
        assert_eq!(day_of_year, 1);

        let (year, day_of_year) = get_year(85279);  // Test a random date, which is the 18th of March 2026, which is the 178th day of year 234
        assert_eq!(year, 234);
        assert_eq!(day_of_year, 178);
    }

    #[test]
    fn test_get_month() {
        let (month, day_of_month) = get_month(30);  // The 30th day of the year is the last day of the first month
        assert_eq!(month, 1);
        assert_eq!(day_of_month, 30);

        let (month, day_of_month) = get_month(31); // The 31st day of the year is the first day of the second month
        assert_eq!(month, 2);
        assert_eq!(day_of_month, 1);

        let (month, day_of_month) = get_month(60);
        assert_eq!(month, 2);
        assert_eq!(day_of_month, 30);

        let (month, day_of_month) = get_month(360); // The 360th day of the year is the last day of the 12th month
        assert_eq!(month, 12);
        assert_eq!(day_of_month, 30);

        let (month, day_of_month) = get_month(361); // The 361st day of the year is the first day of the 13th month, which is the "Sans-culottides"
        assert_eq!(month, 13);
        assert_eq!(day_of_month, 1);

    }

    #[test]
    fn test_date_calculator_romme() {
        let date = Date::from_calendar_date(2026, time::Month::March, 17).unwrap(); // Random date, which is the 17th of March 2026, which is the 177th day of year 234
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_date_string(), "Septidi de Ventôse de l'an 234");

        let date = Date::from_calendar_date(1792, time::Month::September, 22).unwrap(); // The start date of the Republican calendar, which is the 1st of Vendémiaire of the year 1
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_date_string(), "Primidi de Vendémiaire de l'an 1");

        let date = Date::from_calendar_date(1799, time::Month::September, 20).unwrap(); // The first anniversary of the start date, which is the 1st of Vendémiaire of the year 2
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_date_string(), "4 jour complémentaire de l'an 7");

        let date = Date::from_calendar_date(1796, time::Month::September, 21).unwrap(); // The second anniversary of the start date, which is the 1st of Vendémiaire of the year 3
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_date_string(), "6 jour complémentaire de l'an 4");


    }
}