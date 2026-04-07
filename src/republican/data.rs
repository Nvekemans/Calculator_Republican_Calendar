use std::{collections::HashMap, fs::File, io::BufReader};

use serde_json::Result;

use crate::republican::date::RepublicanDate;

/// This struct represents the data calendar, which contains the objects of the day for each day of the Republican calendar. The data is stored in a JSON file and loaded into a HashMap for easy access.
pub struct DataCalendar {
    object_json: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

impl DataCalendar {
    /// Creates a new DataCalendar by loading the data from the specified JSON file. The JSON file should have a specific structure that allows for easy retrieval of the object of the day based on the season, month, and day.
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Unable to open the data file");
        let reader = BufReader::new(file);
        let object_json: HashMap<String, HashMap<String, HashMap<String, String>>> =
            serde_json::from_reader(reader).expect("Unable to parse the data file");
        Self { object_json }
    }

    /// Retrieves the object of the day for a given Republican date. The method looks up the season, month, and day in the loaded JSON data and returns the corresponding object as a string. If any of the lookups fail (e.g., season, month, or day not found), it will panic with an appropriate error message.
    pub fn get_object(&self, date: RepublicanDate) -> Result<String> {
        Ok(self
            .object_json
            .get(date.season())
            .expect("Season not found in data file")
            .get(date.month().to_string().as_str())
            .expect("Month not found in data file")
            .get(date.day().to_string().as_str())
            .expect("Day not found in data file")
            .clone())
    }
}

#[cfg(test)]
mod tests {

    use crate::republican::RepublicanMonth;

    use super::*;

    #[test]
    fn test_get_object() {
        let data = DataCalendar::new("src/data.json");
        let date = RepublicanDate::from_calendar_date(1, RepublicanMonth::Fructidor, 1).unwrap();
        let object = data.get_object(date).unwrap();
        assert_eq!(object, "Prune");
    }
}
