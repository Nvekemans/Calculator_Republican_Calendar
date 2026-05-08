use crate::republican::{RepublicanDay, month::RepublicanMonth};
use time::Duration;

/// A date in the Republican calendar.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RepublicanDate {
    year: u32,
    month: RepublicanMonth,
    day: u8,
}

impl RepublicanDate {

    /// The minimum valid date in the Republican calendar, which is the 1st of Vendémiaire of year 1. This represents the starting point of the calendar.
    pub const MIN: Self = Self {
        year: 1,
        month: RepublicanMonth::Vendémiaire,
        day: 1,
    };

    /// The maximum valid date in the Republican calendar, which is the 5th of Sansculottides of the maximum year (u32::MAX). This represents the end point of the calendar. Note that the maximum year is not a leap year, so there are only 5 Sansculottides.
    pub const MAX: Self = Self {
        year: u32::MAX,
        month: RepublicanMonth::Sansculottides,
        day: 5, // Max day in Sansculottides for a non-leap year, which is the case for the maximum year as it is a odd number and thus not a leap year.
    };

    /// Creates a new RepublicanDate with the given year, month, and day. The method validates the input parameters to ensure they represent a valid date in the Republican calendar. If the parameters are valid, it returns a new instance of RepublicanDate; otherwise, it returns an error message indicating the reason for the failure (e.g., invalid day, month, or year).
    pub fn from_calendar_date(year: u32, month: RepublicanMonth, day: u8) -> Result<Self, String> {
        if year < 1 {
            return Err(format!("Invalid year: {}. Year must be at least 1.", year));
        }

        if !(1..=30).contains(&day) {
            return Err(format!(
                "Invalid day: {}. Day must be between 1 and 30.",
                day
            ));
        }

        if month == RepublicanMonth::Sansculottides {
            if Self::is_leap_year(year) {
                // Leap year, so there are 6 Sansculottides
                if day > 6 {
                    return Err(format!(
                        "Invalid day: {}. In a leap year, Sansculottides can have up to 6 days.",
                        day
                    ));
                }
            } else {
                // Non-leap year, so there are only 5 Sansculottides
                if day > 5 {
                    return Err(format!(
                        "Invalid day: {}. In a non-leap year, Sansculottides can have up to 5 days.",
                        day
                    ));
                }
            }
        }
        Ok(RepublicanDate {
            year,
            month,
            day,
        })
    }

    /// Creates a new RepublicanDate from the given year and ordinal day of the year. The method validates the input parameters to ensure they represent a valid date in the Republican calendar. If the parameters are valid, it calculates the corresponding month and day based on the ordinal day and returns a new instance of RepublicanDate; otherwise, it returns an error message indicating the reason for the failure (e.g., invalid year or ordinal day).
    pub fn from_ordinal_date(year: u32, ordinal_day: u16) -> Result<Self, String> {
        if year < 1 {
            return Err(format!("Invalid year: {}. Year must be at least 1.", year));
        }

                
        if ordinal_day < 1 || ordinal_day > 366 {
            return Err(format!(
                "Invalid ordinal day: {}. Ordinal day must be between 1 and 366.",
                ordinal_day
            ));
        }

        if ordinal_day == 366 && !Self::is_leap_year(year) {
            return Err(format!(
                "Invalid ordinal day: {}. Year {} is not a leap year, so it has only 365 days.",
                ordinal_day, year
            ));
        }

        let month = ((ordinal_day - 1) / 30 + 1) as u8;
        let day = ((ordinal_day - 1) % 30 + 1) as u8;

        let month_enum = RepublicanMonth::try_from(month).unwrap(); // This should never fail because we've already validated the ordinal day range

        Ok(RepublicanDate { year, month: month_enum, day })
    }

    /// Returns the year of the Republican date.
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Returns the month of the Republican date.
    pub fn month(&self) -> RepublicanMonth {
        self.month
    }

    /// Returns the day of the Republican date.
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Returns the name of the day in the Republican calendar.
    pub fn day_of_decade(&self) -> RepublicanDay {
        RepublicanDay::try_from(self.day).expect("Invalid day number, should be between 1 and 30")
    }

    /// Returns the season of the Republican date based on the month. The method delegates the retrieval of the season to the month, which has a method to determine its corresponding season.
    pub fn season(&self) -> &'static str {
        self.month.get_season()
    }

    /// Returns the ordinal day of the year for the Republican date. The method calculates the ordinal day by multiplying the month index (0-based) by 30 and adding the day of the month. This gives a unique number for each day of the year.
    pub fn ordinal(&self) -> u16 {
        let month_ordinal = (u8::from(self.month) - 1) as u16; // Convert month to 0-based index
        month_ordinal * 30 + self.day as u16
    }

    /// Returns whether the given year is a leap year in the Republican calendar. The leap year rules are the same as the Gregorian calendar, but with a different starting point. A year is a leap year if it is divisible by 4 but not by 100, or if it is divisible by 400.
    pub fn is_leap_year(year: u32) -> bool {
        (year.is_multiple_of(4) && !year.is_multiple_of(100)) || (year.is_multiple_of(400))
    }

    /// Returns whether the Republican date falls in a leap year.
    pub fn is_self_leap_year(&self) -> bool {
        Self::is_leap_year(self.year)
    }

    /// Converts the Republican date to a tuple containing the year, month, the decade and the day. This method provides a way to represent the date in a more traditional calendar format.
    pub fn to_calendar_date(&self) -> (u32, RepublicanMonth, u8, RepublicanDay) {
        (self.year, self.month, self.decade(),self.day_of_decade())
    }

    /// Converts the Republican date to a tuple containing the year and the ordinal day of the year.
    pub fn to_ordinal_date(&self) -> (u32, u16) {
        (self.year, self.ordinal())
    }

    /// Returns the Republican date for the next day. If the current date is the maximum valid date, it returns None to indicate that there is no tomorrow. 
     pub fn tomorrow(&self) -> Option<Self> {
        if self == &Self::MAX {
            return None; // No tomorrow if we are at the maximum date
        }
        let mut day = self.day + 1;
        let mut month = u8::from(self.month);
        let mut year = self.year;

        if month == 13 {
            if Self::is_leap_year(year) && day > 6 || !Self::is_leap_year(year) && day > 5 {
                day = 1;
                month = 1;
                year += 1;
            }
        } else if day > 30 {
            day = 1;
            month += 1;
        }

        Some(Self::from_calendar_date(year, RepublicanMonth::try_from(month).unwrap(), day).expect("Impossible date generated in get_tomorrow"))
    }

    /// Returns the Republican date for the previous day. If the current date is the minimum valid date, it returns None to indicate that there is no yesterday.
     pub fn yesterday(&self) -> Option<Self> {
        if self == &Self::MIN {
            return None; // No yesterday if we are at the minimum date
        }
        let mut day = self.day - 1;
        let mut month = u8::from(self.month);
        let mut year = self.year;

        if month == 1 && day < 1 {
            month = 13;
            if Self::is_leap_year(year) {
                day = 6;
            } else {
                day = 5;
            }
            year -= 1;
        } else if day < 1 {
            month -= 1;
            day = 30;
        }

        Some(Self::from_calendar_date(year, RepublicanMonth::try_from(month).unwrap(), day).expect("Impossible date generated in get_yesterday"))
    }

    /// Adds a duration to the Republican date, returning the resulting date if the operation is valid.
    pub fn checked_add(self, duration : Duration) -> Option<Self> {
        let total_days = duration.whole_days();
        if duration.is_negative() {
            return self.checked_sub(-duration);
        }
        let mut result = self;
        for _ in 0..total_days {
            result = result.tomorrow()?;
        }   
        Some(result)
    }

    /// Subtracts a duration from the Republican date, returning the resulting date if the operation is valid.
    pub fn checked_sub(self, duration : Duration) -> Option<Self> {
        let total_days = duration.whole_days();
        if duration.is_negative() {
            return self.checked_add(-duration);
        }
        let mut result = self;
        for _ in 0..total_days {
            result = result.yesterday()?;
        }   
        Some(result)
    }

    /// Replace the year. The month and day are kept the same. The method validates the new year along with the existing month and day to ensure that the resulting date is valid. If the new year is valid, it returns a new instance of RepublicanDate with the updated year; otherwise, it returns an error message indicating the reason for the failure (e.g., invalid year or resulting date).
    pub fn replace_year(&self, new_year: u32) -> Result<Self, String> {
        Self::from_calendar_date(new_year, self.month, self.day)
    }

    /// Replace the month. The year and day are kept the same. The method validates the new month along with the existing year and day to ensure that the resulting date is valid. If the new month is valid, it returns a new instance of RepublicanDate with the updated month; otherwise, it returns an error message indicating the reason for the failure (e.g., invalid month or resulting date).
    pub fn replace_month(&self, new_month: RepublicanMonth) -> Result<Self, String> {
        Self::from_calendar_date(self.year, new_month, self.day)
    }

    /// Replace the day. The year and month are kept the same. The method validates the new day along with the existing year and month to ensure that the resulting date is valid. If the new day is valid, it returns a new instance of RepublicanDate with the updated day; otherwise, it returns an error message indicating the reason for the failure (e.g., invalid day or resulting date).
    pub fn replace_day(&self, new_day: u8) -> Result<Self, String> {
        Self::from_calendar_date(self.year, self.month, new_day)
    }


    /// Gets the decade of the Republican date. The decade is calculated based on the day of the month, with each decade consisting of 10 days. The method returns a number between 1 and 3, indicating which decade the date falls into (1 for days 1-10, 2 for days 11-20, and 3 for days 21-30).
    pub fn decade(&self) -> u8 {
        (self.day - 1) / 10 + 1
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::republican::month::RepublicanMonth;

    // Valid date creation
    #[test]
    fn test_new_valid_date() {
        let date = RepublicanDate::from_calendar_date(89, RepublicanMonth::Vendémiaire, 1).unwrap();
        assert_eq!(date.year(), 89);
        assert_eq!(date.month(), RepublicanMonth::Vendémiaire);
        assert_eq!(date.day(), 1);

        // Mid-year date
        let date = RepublicanDate::from_calendar_date(2025, RepublicanMonth::try_from(5).unwrap(), 15).unwrap();
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), RepublicanMonth::Pluviôse);
        assert_eq!(date.day(), 15);
    }

    // Invalid day
    #[test]
    fn test_new_invalid_day() {
        let err = RepublicanDate::from_calendar_date(232, RepublicanMonth::Vendémiaire, 31).unwrap_err();
        assert_eq!(err, "Invalid day: 31. Day must be between 1 and 30.");

        let err = RepublicanDate::from_calendar_date(23, RepublicanMonth::Pluviôse, 0).unwrap_err();
        assert_eq!(err, "Invalid day: 0. Day must be between 1 and 30.");
    }

    // Invalid year
    #[test]
    fn test_new_invalid_year() {
        let err = RepublicanDate::from_calendar_date(0, RepublicanMonth::Vendémiaire, 1).unwrap_err();
        assert_eq!(err, "Invalid year: 0. Year must be at least 1.");
    }

    // Leap year detection
    #[test]
    fn test_leap_year() {
        let d = RepublicanDate::from_calendar_date(1796, RepublicanMonth::Vendémiaire, 1).unwrap(); // leap year
        assert!(d.is_self_leap_year());

        let d = RepublicanDate::from_calendar_date(27, RepublicanMonth::Vendémiaire, 1).unwrap(); // non-leap
        assert!(!d.is_self_leap_year());

        let d = RepublicanDate::from_calendar_date(1200, RepublicanMonth::Vendémiaire, 1).unwrap(); // leap year
        assert!(d.is_self_leap_year());

        let d = RepublicanDate::from_calendar_date(200, RepublicanMonth::Vendémiaire, 1).unwrap(); // non-leap
        assert!(!d.is_self_leap_year());
    }

    // Leap year & Sansculottides
    #[test]
    fn test_sansculottides_leap_year() {
        // Leap year → 6 Sansculottides
        let date = RepublicanDate::from_calendar_date(16, RepublicanMonth::Sansculottides, 6).unwrap();
        assert_eq!(date.month(), RepublicanMonth::Sansculottides);
        assert_eq!(date.day(), 6);

        let err = RepublicanDate::from_calendar_date(40, RepublicanMonth::Sansculottides, 7).unwrap_err();
        assert_eq!(
            err,
            "Invalid day: 7. In a leap year, Sansculottides can have up to 6 days."
        );
    }

    #[test]
    fn test_sansculottides_non_leap_year() {
        // Non-leap year → 5 Sansculottides
        let date = RepublicanDate::from_calendar_date(1797, RepublicanMonth::Sansculottides, 5).unwrap();
        assert_eq!(date.month(), RepublicanMonth::Sansculottides);
        assert_eq!(date.day(), 5);

        let err = RepublicanDate::from_calendar_date(1797, RepublicanMonth::Sansculottides, 6).unwrap_err();
        assert_eq!(
            err,
            "Invalid day: 6. In a non-leap year, Sansculottides can have up to 5 days."
        );
    }

    // Day names
    #[test]
    fn test_day_names() {

        for i in 1..=10 {
            let date = RepublicanDate::from_calendar_date(27, RepublicanMonth::Vendémiaire, i).unwrap();
            assert_eq!(date.day_of_decade(), RepublicanDay::try_from(i).unwrap());
        }

        // 11th day should cycle back to Primidi
        let date = RepublicanDate::from_calendar_date(27, RepublicanMonth::Vendémiaire, 11).unwrap();
        assert_eq!(date.day_of_decade(), RepublicanDay::try_from(1).unwrap());

        // 27th day should be Septidi
        let date = RepublicanDate::from_calendar_date(27, RepublicanMonth::Vendémiaire, 27).unwrap();
        assert_eq!(date.day_of_decade(), RepublicanDay::try_from(27).unwrap());
    }

    // Seasons
    #[test]
    fn test_season() {
        let autumn = RepublicanMonth::Vendémiaire.get_season();
        let winter = RepublicanMonth::Nivôse.get_season();
        let spring = RepublicanMonth::Germinal.get_season();
        let summer = RepublicanMonth::Messidor.get_season();
        let sc = RepublicanMonth::Sansculottides.get_season();

        assert_eq!(autumn, "Automne");
        assert_eq!(winter, "Hiver");
        assert_eq!(spring, "Printemps");
        assert_eq!(summer, "Été");
        assert_eq!(sc, "Été");
    }

    // Tomorrow / Yesterday arithmetic
    #[test]
    fn test_tomorrow_yesterday() {
        let d = RepublicanDate::from_calendar_date(38, RepublicanMonth::Vendémiaire, 30).unwrap();
        let tomorrow = d.tomorrow();
        assert_eq!(tomorrow.unwrap().day(), 1);
        assert_eq!(tomorrow.unwrap().month(), RepublicanMonth::Brumaire);

        let d = RepublicanDate::from_calendar_date(38, RepublicanMonth::Brumaire, 1).unwrap();
        let yesterday = d.yesterday();
        assert_eq!(yesterday.unwrap().day(), 30);
        assert_eq!(yesterday.unwrap().month(), RepublicanMonth::Vendémiaire);
    }

    #[test]
    fn test_tomorrow_yesterday_sansculottides() {
        // Non-leap year
        let d = RepublicanDate::from_calendar_date(1797, RepublicanMonth::Sansculottides, 5).unwrap();
        let tomorrow = d.tomorrow();
        assert_eq!(tomorrow.unwrap().day(), 1);
        assert_eq!(tomorrow.unwrap().month(), RepublicanMonth::Vendémiaire);
        assert_eq!(tomorrow.unwrap().year(), 1798);

        // Leap year
        let d = RepublicanDate::from_calendar_date(1796, RepublicanMonth::Sansculottides, 6).unwrap();
        let tomorrow = d.tomorrow();
        assert_eq!(tomorrow.unwrap().day(), 1);
        assert_eq!(tomorrow.unwrap().month(), RepublicanMonth::Vendémiaire);
        assert_eq!(tomorrow.unwrap().year(), 1797);
    }

    // Round-trip consistency
    #[test]
    fn test_round_trip() {
        let d = RepublicanDate::from_calendar_date(1792, RepublicanMonth::Vendémiaire, 1).unwrap();
        let tomorrow = d.tomorrow().unwrap();
        let yesterday = tomorrow.yesterday().unwrap();
        assert_eq!(d, yesterday);
    }
}
