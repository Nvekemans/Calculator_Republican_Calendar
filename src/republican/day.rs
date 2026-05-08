use core::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RepublicanDay {
    Primidi,
    Duodi,
    Tridi,
    Quartidi,
    Quintidi,
    Sextidi,
    Septidi,
    Octidi,
    Nonidi,
    Décadi,
}

impl RepublicanDay {
    
    pub fn previous(&self) -> Self {
        match self {
            RepublicanDay::Primidi => RepublicanDay::Décadi,
            RepublicanDay::Duodi => RepublicanDay::Primidi,
            RepublicanDay::Tridi => RepublicanDay::Duodi,
            RepublicanDay::Quartidi => RepublicanDay::Tridi,
            RepublicanDay::Quintidi => RepublicanDay::Quartidi,
            RepublicanDay::Sextidi => RepublicanDay::Quintidi,
            RepublicanDay::Septidi => RepublicanDay::Sextidi,
            RepublicanDay::Octidi => RepublicanDay::Septidi,
            RepublicanDay::Nonidi => RepublicanDay::Octidi,
            RepublicanDay::Décadi => RepublicanDay::Nonidi,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            RepublicanDay::Primidi => RepublicanDay::Duodi,
            RepublicanDay::Duodi => RepublicanDay::Tridi,
            RepublicanDay::Tridi => RepublicanDay::Quartidi,
            RepublicanDay::Quartidi => RepublicanDay::Quintidi,
            RepublicanDay::Quintidi => RepublicanDay::Sextidi,
            RepublicanDay::Sextidi => RepublicanDay::Septidi,
            RepublicanDay::Septidi => RepublicanDay::Octidi,
            RepublicanDay::Octidi => RepublicanDay::Nonidi,
            RepublicanDay::Nonidi => RepublicanDay::Décadi,
            RepublicanDay::Décadi => RepublicanDay::Primidi,
        }

}}

impl fmt::Display for RepublicanDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RepublicanDay::Primidi => "Primidi",
            RepublicanDay::Duodi => "Duodi",
            RepublicanDay::Tridi => "Tridi",
            RepublicanDay::Quartidi => "Quartidi",
            RepublicanDay::Quintidi => "Quintidi",
            RepublicanDay::Sextidi => "Sextidi",
            RepublicanDay::Septidi => "Septidi",
            RepublicanDay::Octidi => "Octidi",
            RepublicanDay::Nonidi => "Nonidi",
            RepublicanDay::Décadi => "Décadi",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<u8> for RepublicanDay {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 | 11 | 21 => Ok(RepublicanDay::Primidi),
            2 | 12 | 22 => Ok(RepublicanDay::Duodi),
            3 | 13 | 23 => Ok(RepublicanDay::Tridi),
            4 | 14 | 24 => Ok(RepublicanDay::Quartidi),
            5 | 15 | 25 => Ok(RepublicanDay::Quintidi),
            6 | 16 | 26 => Ok(RepublicanDay::Sextidi),
            7 | 17 | 27 => Ok(RepublicanDay::Septidi),
            8 | 18 | 28 => Ok(RepublicanDay::Octidi),
            9 | 19 | 29 => Ok(RepublicanDay::Nonidi),
            10 | 20 | 30 => Ok(RepublicanDay::Décadi),
            _ => Err(format!("Invalid day number: {}. Day number must be between 1 and 30.", value)),
        }
    }
}

impl FromStr for RepublicanDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Primidi" => Ok(RepublicanDay::Primidi),
            "Duodi" => Ok(RepublicanDay::Duodi),
            "Tridi" => Ok(RepublicanDay::Tridi),
            "Quartidi" => Ok(RepublicanDay::Quartidi),
            "Quintidi" => Ok(RepublicanDay::Quintidi),
            "Sextidi" => Ok(RepublicanDay::Sextidi),
            "Septidi" => Ok(RepublicanDay::Septidi),
            "Octidi" => Ok(RepublicanDay::Octidi),
            "Nonidi" => Ok(RepublicanDay::Nonidi),
            "Décadi" => Ok(RepublicanDay::Décadi),
            _ => Err(format!("Invalid day name: {}. Day name must be one of: Primidi, Duodi, Tridi, Quartidi, Quintidi, Sextidi, Septidi, Octidi, Nonidi, Décadi.", s)),
        }
    }
}
