
use std::{collections::HashMap, fs};
#[derive(Clone, Copy, PartialEq, Eq)]
enum RepublicanMonth {
    Vendémiaire,
    Brumaire,
    Frimaire,
    Nivose,
    Pluviose,
    Ventôse,
    Germinal,
    Floreal,
    Prairial,
    Messidor,
    Thermidor,
    Fructidor,
    Sansculottides,
}

impl RepublicanMonth {
    pub fn as_str(&self) -> &'static str {
        match self {
            RepublicanMonth::Vendémiaire => "Vendémiaire",
            RepublicanMonth::Brumaire => "Brumaire",
            RepublicanMonth::Frimaire => "Frimaire",
            RepublicanMonth::Nivose => "Nivose",
            RepublicanMonth::Pluviose => "Pluviose",
            RepublicanMonth::Ventôse => "Ventôse",
            RepublicanMonth::Germinal => "Germinal",
            RepublicanMonth::Floreal => "Floreal",
            RepublicanMonth::Prairial => "Prairial",
            RepublicanMonth::Messidor => "Messidor",
            RepublicanMonth::Thermidor => "Thermidor",
            RepublicanMonth::Fructidor => "Fructidor",
            RepublicanMonth::Sansculottides => "Sansculottides",
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

        let month_enum = match month {
            1 => RepublicanMonth::Vendémiaire,
            2 => RepublicanMonth::Brumaire,
            3 => RepublicanMonth::Frimaire,
            4 => RepublicanMonth::Nivose,
            5 => RepublicanMonth::Pluviose,
            6 => RepublicanMonth::Ventôse,
            7 => RepublicanMonth::Germinal,
            8 => RepublicanMonth::Floreal,
            9 => RepublicanMonth::Prairial,
            10 => RepublicanMonth::Messidor,
            11 => RepublicanMonth::Thermidor,
            12 => RepublicanMonth::Fructidor,
            13 => RepublicanMonth::Sansculottides,
            _ => panic!("Invalid month for the Republican calendar: {}", month),
        };

        if month == 13 && day > 6 {
            panic!("Invalid day for Sansculottides: {}", day);
        }
        Self { year, month: month_enum, day }
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

    pub fn get_month_name(&self) -> &'static str {
        self.month.as_str()
    }

    pub fn get_day_name(&self) -> &'static str {
        // Decadi is the 10th day of the decade, it is first in the list because 10 % 10 = 0. This way, the day names repeat every 10 days.
        let day_names = [
            "Décadi", "Primidi", "Duodi", "Tridi", "Quartidi", "Quintidi",
            "Sextidi", "Septidi", "Octidi", "Nonidi",
        ];
        day_names[(self.day % 10) as usize]
    }

    pub fn get_season(&self) -> &'static str {
        match self.month {
            RepublicanMonth::Vendémiaire | RepublicanMonth::Brumaire | RepublicanMonth::Frimaire => "Automne",
            RepublicanMonth::Nivose | RepublicanMonth::Pluviose | RepublicanMonth::Ventôse => "Hiver",
            RepublicanMonth::Germinal | RepublicanMonth::Floreal | RepublicanMonth::Prairial => "Printemps",
            RepublicanMonth::Messidor | RepublicanMonth::Thermidor | RepublicanMonth::Fructidor | RepublicanMonth::Sansculottides => "Été",
        }
    }

    pub fn get_object(&self) -> String {

        // Read the data from the JSON file
        let file_to_data = format!("src/data.json");
        let data = fs::read_to_string(file_to_data).expect("Unable to read data file");

        // Parse the JSON data into a HashMap
        let json: HashMap<String, HashMap<String, HashMap<String, String>>> = serde_json::from_str(&data).expect("Unable to parse JSON");

        // Look up the object based on the season, month, and day
        json.get(self.get_season()).expect("No match for the season")
            .get(self.get_month_name()).expect("No match for the month")
            .get(&self.day.to_string())
            .cloned()
            .expect("No match for the day")
    }

    pub fn get_date_string(&self) -> String {
        if self.month == RepublicanMonth::Sansculottides {
            return format!("{} jour complémentaire de l'an {}", self.get_day(), self.get_year());
        }
        format!("{} de {} de l'an {}", self.get_day_name(), self.get_month_name(), self.get_year())

    }

    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_republican_date() {
        let date = RepublicanDate::new(1, 1, 1);
        assert_eq!(date.get_year(), 1);
        assert_eq!(date.get_month_name(), "Vendémiaire");
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
}