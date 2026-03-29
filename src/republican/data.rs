use std::{collections::HashMap, fs::File, io::BufReader};

use serde_json::Result;

use crate::republican::date::RepublicanDate;

pub struct DataCalendar {
    object_json : HashMap<String, HashMap<String, HashMap<String, String>>>
}

impl DataCalendar {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Unable to open the data file");
        let reader = BufReader::new(file);
        let object_json: HashMap<String, HashMap<String, HashMap<String, String>>> = serde_json::from_reader(reader).expect("Unable to parse the data file");
        Self { object_json }
    }

    pub fn get_object(&self, date: RepublicanDate) -> Result<String> {
        Ok(self.object_json.get(date.get_season()).expect("Season not found in data file")
            .get(date.get_month().to_string().as_str()).expect("Month not found in data file")
            .get(date.get_day().to_string().as_str()).expect("Day not found in data file")
            .clone())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_object() {
        let data = DataCalendar::new("src/data.json");
        let date = RepublicanDate::new(1, 12, 1).unwrap();
        let object = data.get_object(date).unwrap();
        assert_eq!(object, "Prune");
    }
}