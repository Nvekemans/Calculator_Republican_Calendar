use core::fmt;
use std::{collections::HashMap, fs};
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RepublicanMonth {
    Vendémiaire,
    Brumaire,
    Frimaire,
    Nivôse,
    Pluviôse,
    Ventôse,
    Germinal,
    Floréal,
    Prairial,
    Messidor,
    Thermidor,
    Fructidor,
    Sansculottides,
}

impl RepublicanMonth {
    pub fn get_month_number(&self) -> u8 {
        match self {
            Self::Vendémiaire => 1,
            Self::Brumaire => 2,
            Self::Frimaire => 3,
            Self::Nivôse => 4,
            Self::Pluviôse => 5,
            Self::Ventôse => 6,
            Self::Germinal => 7,
            Self::Floréal => 8,
            Self::Prairial => 9,
            Self::Messidor => 10,
            Self::Thermidor => 11,
            Self::Fructidor => 12,
            Self::Sansculottides => 13,
        }
    }

    pub fn get_season(&self) -> &'static str {
        match self {
            Self::Vendémiaire | Self::Brumaire | Self::Frimaire => "Automne",
            Self::Nivôse | Self::Pluviôse | Self::Ventôse => "Hiver",
            Self::Germinal | Self::Floréal | Self::Prairial => "Printemps",
            Self::Messidor | Self::Thermidor | Self::Fructidor | Self::Sansculottides => "Été",
        }
    }
}

impl fmt::Display for RepublicanMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Vendémiaire => "Vendémiaire",
            Self::Brumaire => "Brumaire",
            Self::Frimaire => "Frimaire",
            Self::Nivôse => "Nivôse",
            Self::Pluviôse => "Pluviôse",
            Self::Ventôse => "Ventôse",
            Self::Germinal => "Germinal",
            Self::Floréal => "Floréal",
            Self::Prairial => "Prairial",
            Self::Messidor => "Messidor",
            Self::Thermidor => "Thermidor",
            Self::Fructidor => "Fructidor",
            Self::Sansculottides => "Sansculottides",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<u8> for RepublicanMonth {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Vendémiaire),
            2 => Ok(Self::Brumaire),
            3 => Ok(Self::Frimaire),
            4 => Ok(Self::Nivôse),
            5 => Ok(Self::Pluviôse),
            6 => Ok(Self::Ventôse),
            7 => Ok(Self::Germinal),
            8 => Ok(Self::Floréal),
            9 => Ok(Self::Prairial),
            10 => Ok(Self::Messidor),
            11 => Ok(Self::Thermidor),
            12 => Ok(Self::Fructidor),
            13 => Ok(Self::Sansculottides),
            _ => Err(format!("Invalid month number: {}", value)),
        }
    }
}

pub struct RepublicanDate {
    year: i32,
    month: RepublicanMonth,
    day: i32,
}

impl RepublicanDate {
    pub fn new(year: i32, month: i32, day: i32) -> Self {
        if day < 1 || day > 30 {
            panic!("Invalid day for the Republican calendar: {}", day);
        }

        let month_enum = RepublicanMonth::try_from(month as u8)
            .expect("Invalid month number for the Republican calendar");

        if month == 13 && day > 6 {
            panic!("Invalid day for Sansculottides: {}", day);
        }
        Self {
            year,
            month: month_enum,
            day,
        }
    }

    pub fn get_year(&self) -> i32 {
        self.year
    }

    pub fn get_month(&self) -> RepublicanMonth {
        self.month
    }

    pub fn get_day(&self) -> i32 {
        self.day
    }

    pub fn get_day_name(&self) -> &'static str {
        // Decadi is the 10th day of the decade, it is first in the list because 10 % 10 = 0. This way, the day names repeat every 10 days.
        let day_names = [
            "Décadi", "Primidi", "Duodi", "Tridi", "Quartidi", "Quintidi", "Sextidi", "Septidi",
            "Octidi", "Nonidi",
        ];
        day_names[(self.day % 10) as usize]
    }

    pub fn get_season(&self) -> &'static str {
        self.month.get_season()
    }

    pub fn get_object(&self) -> String {
        // Read the data from the JSON file
        let file_to_data = format!("src/data.json");
        let data = fs::read_to_string(file_to_data).expect("Unable to read data file");

        // Parse the JSON data into a HashMap
        let json: HashMap<String, HashMap<String, HashMap<String, String>>> =
            serde_json::from_str(&data).expect("Unable to parse JSON");

        // Look up the object based on the season, month, and day
        json.get(self.get_season())
            .expect("No match for the season")
            .get(&self.month.to_string())
            .expect("No match for the month")
            .get(&self.day.to_string())
            .cloned()
            .expect("No match for the day")
    }

    pub fn get_date_string(&self) -> String {
        if self.month == RepublicanMonth::Sansculottides {
            return format!(
                "{} jour complémentaire de l'an {}",
                self.get_day(),
                self.get_year()
            );
        }
        format!(
            "{} de {} de l'an {}",
            self.get_day_name(),
            self.get_month().to_string(),
            self.get_year()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_republican_date() {
        let date = RepublicanDate::new(1, 1, 1);
        assert_eq!(date.get_year(), 1);
        assert_eq!(date.get_month().to_string(), "Vendémiaire");
        assert_eq!(date.get_day(), 1);
        assert_eq!(date.get_day_name(), "Primidi");
        assert_eq!(date.get_season(), "Automne");
        assert_eq!(date.get_object(), "Raisin");
    }

    #[test]
    fn test_republicain_get_day_name() {
        let date = RepublicanDate::new(1, 1, 10);
        assert_eq!(date.get_day_name(), "Décadi");

        let date = RepublicanDate::new(1, 1, 11);
        assert_eq!(date.get_day_name(), "Primidi");

        let date = RepublicanDate::new(1, 1, 20);
        assert_eq!(date.get_day_name(), "Décadi");

        let date = RepublicanDate::new(1, 1, 21);
        assert_eq!(date.get_day_name(), "Primidi");
    }

    #[test]
    fn test_get_object() {
        let date = RepublicanDate::new(45, 1, 1);
        assert_eq!(date.get_object(), "Raisin");

        let date = RepublicanDate::new(1, 2, 1);
        assert_eq!(date.get_object(), "Pomme");

        let date = RepublicanDate::new(265, 3, 29);
        assert_eq!(date.get_object(), "Olive");

        let date = RepublicanDate::new(123, 4, 15);
        assert_eq!(date.get_object(), "Lapin");

        let date = RepublicanDate::new(789, 5, 30);
        assert_eq!(date.get_object(), "Traîneau");

        let date = RepublicanDate::new(342, 8, 17);
        assert_eq!(date.get_object(), "Pimprenelle");

        let date = RepublicanDate::new(432, 13, 1);
        assert_eq!(date.get_object(), "La Fête de la Vertu");
    }
}
