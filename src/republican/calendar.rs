use crate::republican::date::RepublicanDate;
use time::Date;

pub fn date_calculator_romme(date: Date) -> RepublicanDate {
    let start_date = Date::from_calendar_date(1792, time::Month::September, 22).unwrap(); // The start date of the Republican calendar is September 22, 1792
    let days_since_start = (date - start_date).whole_days() + 1; // +1 because the start date is the first day of the Republican calendar

    let (year, day_of_year) = get_year_number(days_since_start as u64);

    let (month, day_of_month) = get_month_number(day_of_year);

    RepublicanDate::new(year, month, day_of_month).expect("Error in the date conversion")
}

//** Returns the year and day of the year for a given number of days since the start date. */
fn get_year_number(mut days_since_start: u64) -> (u32, u64) {
    let mut year = 1;
    while days_since_start > 365 {
        // A year has 365 days, but we need to check for leap years
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            // The year is a leap year if it is divisible by 4, but not by 100 unless it is also divisible by 400
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

    (year, days_since_start)
}

//** Returns the month and day of the month for a given number of days since the start of the year. */
fn get_month_number(mut days_since_start: u64) -> (u8, u8) {
    let mut month = 1;
    while days_since_start > 30 {
        // A month has always 30 days in the Republican calendar. The last 5 or 6 days of the year are the "Sans-culottides" and are here considered as the the 13th month.
        month += 1;
        days_since_start -= 30;
    }

    (month, days_since_start as u8)
}

#[cfg(test)]
mod tests {
    use crate::republican::month::RepublicanMonth;

    use super::*;
    use time::Date;

    #[test]
    fn test_date_calculator_romme() {
        let date = Date::from_calendar_date(1792, time::Month::September, 22).unwrap();
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_year(), 1);
        assert_eq!(republican_date.get_month(), RepublicanMonth::Vendémiaire);
        assert_eq!(republican_date.get_day(), 1);

        let date = Date::from_calendar_date(1793, time::Month::October, 5).unwrap();
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_year(), 2);
        assert_eq!(republican_date.get_month(), RepublicanMonth::Vendémiaire);
        assert_eq!(republican_date.get_day(), 14);

        let date = Date::from_calendar_date(1794, time::Month::November, 10).unwrap();
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_year(), 3);
        assert_eq!(republican_date.get_month(), RepublicanMonth::Brumaire);
        assert_eq!(republican_date.get_day(), 20);

        let date = Date::from_calendar_date(1795, time::Month::December, 25).unwrap();
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_year(), 4);
        assert_eq!(republican_date.get_month(), RepublicanMonth::Nivôse);
        assert_eq!(republican_date.get_day(), 5);

        let date = Date::from_calendar_date(2026, time::Month::March, 26).unwrap();
        let republican_date = date_calculator_romme(date);
        assert_eq!(republican_date.get_year(), 234);
        assert_eq!(republican_date.get_month(), RepublicanMonth::Germinal);
        assert_eq!(republican_date.get_day(), 6);
    }
}
