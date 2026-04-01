use crate::republican::month::RepublicanMonth;
use std::fmt;

/// A date in the Republican calendar.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RepublicanDate {
    year: u32,
    month: RepublicanMonth,
    day: u8,
}

impl fmt::Display for RepublicanDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Nous sommes le {} de {} de l'an {}",
            self.get_day_name(),
            self.month,
            self.year
        )
    }
}

impl RepublicanDate {
    /// Creates a new RepublicanDate with the given year, month, and day. The method validates the input parameters to ensure they represent a valid date in the Republican calendar. If the parameters are valid, it returns a new instance of RepublicanDate; otherwise, it returns an error message indicating the reason for the failure (e.g., invalid day, month, or year).
    pub fn new(year: u32, month: u8, day: u8) -> Result<Self, String> {
        if year < 1 {
            return Err(format!("Invalid year: {}. Year must be at least 1.", year));
        }

        if !(1..=30).contains(&day) {
            return Err(format!(
                "Invalid day: {}. Day must be between 1 and 30.",
                day
            ));
        }

        let month_enum = RepublicanMonth::try_from(month)
            .map_err(|_| format!("Invalid month: {}. Month must be between 1 and 13.", month))?;

        if month_enum == RepublicanMonth::Sansculottides {
            if (year.is_multiple_of(4) && !year.is_multiple_of(100)) || (year.is_multiple_of(400)) {
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
            month: month_enum,
            day,
        })
    }

    /// Returns the year of the Republican date.
    pub fn get_year(&self) -> u32 {
        self.year
    }
    
    /// Returns the month of the Republican date.
    pub fn get_month(&self) -> RepublicanMonth {
        self.month
    }
    
    /// Returns the day of the Republican date.
    pub fn get_day(&self) -> u8 {
        self.day
    }

    /// Returns the name of the day in the Republican calendar. The day names repeat every 10 days, so the method uses the modulo operator to determine the correct name based on the day number.
    pub fn get_day_name(&self) -> &'static str {
        // Decadi is the 10th day of the decade, it is first in the list because 10 % 10 = 0. This way, the day names repeat every 10 days.
        let day_names = [
            "Décadi", "Primidi", "Duodi", "Tridi", "Quartidi", "Quintidi", "Sextidi", "Septidi",
            "Octidi", "Nonidi",
        ];
        day_names[(self.day % 10) as usize]
    }

    /// Returns the season of the Republican date based on the month. The method delegates the retrieval of the season to the month, which has a method to determine its corresponding season.
    pub fn get_season(&self) -> &'static str {
        self.month.get_season()
    }

    /// Returns whether the Republican date falls in a leap year. The leap year rules for the Republican calendar are the same as those for the Gregorian calendar, but with a different starting point. A year is a leap year if it is divisible by 4, except for years that are divisible by 100 but not by 400.
    pub fn is_leap_year(&self) -> bool {
        // The leap years in the Republican calendar are every 4 years, except for years divisible by 100 but not by 400. This is the same rule as the Gregorian calendar, but with a different starting point.
        (self.year.is_multiple_of(4) && !self.year.is_multiple_of(100))
            || (self.year.is_multiple_of(400))
    }

    pub fn get_tomorrow(&self) -> Self {
        let mut day = self.day + 1;
        let mut month = u8::from(self.month);
        let mut year = self.year;

        if month == 13 {
            if self.is_leap_year() && day > 6 || !self.is_leap_year() && day > 5 {
                day = 1;
                month = 1;
                year += 1;
            }
        } else if day > 30 {
            day = 1;
            month += 1;
        }

        Self::new(year, month, day).expect("Impossible date generated in get_tomorrow")
    }

    pub fn get_yesterday(&self) -> Self {
        let mut day = self.day - 1;
        let mut month = u8::from(self.month);
        let mut year = self.year;

        if month == 1 && day < 1 {
            month = 13;
            if self.is_leap_year() {
                day = 6;
            } else {
                day = 5;
            }
            year -= 1;
        } else if day < 1 {
            month -= 1;
            day = 30;
        }

        Self::new(year, month, day).expect("Impossible date generated in get_yesterday")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::republican::month::RepublicanMonth;

    // Valid date creation
    #[test]
    fn test_new_valid_date() {
        let date = RepublicanDate::new(89, 1, 1).unwrap();
        assert_eq!(date.get_year(), 89);
        assert_eq!(date.get_month(), RepublicanMonth::Vendémiaire);
        assert_eq!(date.get_day(), 1);

        // Mid-year date
        let date = RepublicanDate::new(2025, 5, 15).unwrap();
        assert_eq!(date.get_year(), 2025);
        assert_eq!(date.get_month(), RepublicanMonth::Pluviôse);
        assert_eq!(date.get_day(), 15);
    }

    // Invalid day
    #[test]
    fn test_new_invalid_day() {
        let err = RepublicanDate::new(232, 1, 31).unwrap_err();
        assert_eq!(err, "Invalid day: 31. Day must be between 1 and 30.");

        let err = RepublicanDate::new(23, 5, 0).unwrap_err();
        assert_eq!(err, "Invalid day: 0. Day must be between 1 and 30.");
    }

    // Invalid month
    #[test]
    fn test_new_invalid_month() {
        let err = RepublicanDate::new(32, 14, 1).unwrap_err();
        assert_eq!(err, "Invalid month: 14. Month must be between 1 and 13.");
        let err = RepublicanDate::new(2025, 0, 1).unwrap_err();
        assert_eq!(err, "Invalid month: 0. Month must be between 1 and 13.");
    }

    // Invalid year
    #[test]
    fn test_new_invalid_year() {
        let err = RepublicanDate::new(0, 1, 1).unwrap_err();
        assert_eq!(err, "Invalid year: 0. Year must be at least 1.");
    }

    // Leap year detection
    #[test]
    fn test_leap_year() {
        let d = RepublicanDate::new(1796, 1, 1).unwrap(); // leap year
        assert!(d.is_leap_year());

        let d = RepublicanDate::new(27, 1, 1).unwrap(); // non-leap
        assert!(!d.is_leap_year());

        let d = RepublicanDate::new(1200, 1, 1).unwrap(); // leap year
        assert!(d.is_leap_year());

        let d = RepublicanDate::new(200, 1, 1).unwrap(); // non-leap
        assert!(!d.is_leap_year());
    }

    // Leap year & Sansculottides
    #[test]
    fn test_sansculottides_leap_year() {
        // Leap year → 6 Sansculottides
        let date = RepublicanDate::new(16, 13, 6).unwrap();
        assert_eq!(date.get_month(), RepublicanMonth::Sansculottides);
        assert_eq!(date.get_day(), 6);

        let err = RepublicanDate::new(40, 13, 7).unwrap_err();
        assert_eq!(
            err,
            "Invalid day: 7. In a leap year, Sansculottides can have up to 6 days."
        );
    }

    #[test]
    fn test_sansculottides_non_leap_year() {
        // Non-leap year → 5 Sansculottides
        let date = RepublicanDate::new(1797, 13, 5).unwrap();
        assert_eq!(date.get_month(), RepublicanMonth::Sansculottides);
        assert_eq!(date.get_day(), 5);

        let err = RepublicanDate::new(1797, 13, 6).unwrap_err();
        assert_eq!(
            err,
            "Invalid day: 6. In a non-leap year, Sansculottides can have up to 5 days."
        );
    }

    // Day names
    #[test]
    fn test_day_names() {
        let names = [
            "Primidi", "Duodi", "Tridi", "Quartidi", "Quintidi", "Sextidi", "Septidi", "Octidi",
            "Nonidi", "Décadi",
        ];

        for i in 1..=10 {
            let date = RepublicanDate::new(27, 1, i).unwrap();
            assert_eq!(date.get_day_name(), names[i as usize - 1]);
        }

        // 11th day should cycle back to Primidi
        let date = RepublicanDate::new(27, 1, 11).unwrap();
        assert_eq!(date.get_day_name(), "Primidi");

        // 27th day should be Septidi
        let date = RepublicanDate::new(27, 1, 27).unwrap();
        assert_eq!(date.get_day_name(), "Septidi");
    }

    // -------------------------------
    // 6️⃣ Seasons
    // -------------------------------
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

    // -------------------------------
    // 8️⃣ Tomorrow / Yesterday arithmetic
    // -------------------------------
    #[test]
    fn test_tomorrow_yesterday() {
        let d = RepublicanDate::new(38, 1, 30).unwrap();
        let tomorrow = d.get_tomorrow();
        assert_eq!(tomorrow.get_day(), 1);
        assert_eq!(tomorrow.get_month(), RepublicanMonth::Brumaire);

        let d = RepublicanDate::new(38, 2, 1).unwrap();
        let yesterday = d.get_yesterday();
        assert_eq!(yesterday.get_day(), 30);
        assert_eq!(yesterday.get_month(), RepublicanMonth::Vendémiaire);
    }

    #[test]
    fn test_tomorrow_yesterday_sansculottides() {
        // Non-leap year
        let d = RepublicanDate::new(1797, 13, 5).unwrap();
        let tomorrow = d.get_tomorrow();
        assert_eq!(tomorrow.get_day(), 1);
        assert_eq!(tomorrow.get_month(), RepublicanMonth::Vendémiaire);
        assert_eq!(tomorrow.get_year(), 1798);

        // Leap year
        let d = RepublicanDate::new(1796, 13, 6).unwrap();
        let tomorrow = d.get_tomorrow();
        assert_eq!(tomorrow.get_day(), 1);
        assert_eq!(tomorrow.get_month(), RepublicanMonth::Vendémiaire);
        assert_eq!(tomorrow.get_year(), 1797);
    }

    // Round-trip consistency
    #[test]
    fn test_round_trip() {
        let d = RepublicanDate::new(1792, 1, 1).unwrap();
        let tomorrow = d.get_tomorrow();
        let yesterday = tomorrow.get_yesterday();
        assert_eq!(d, yesterday);
    }
}
