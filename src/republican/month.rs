use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

impl From<RepublicanMonth> for u8 {
    fn from(month: RepublicanMonth) -> Self {
        match month {
            RepublicanMonth::Vendémiaire => 1,
            RepublicanMonth::Brumaire => 2,
            RepublicanMonth::Frimaire => 3,
            RepublicanMonth::Nivôse => 4,
            RepublicanMonth::Pluviôse => 5,
            RepublicanMonth::Ventôse => 6,
            RepublicanMonth::Germinal => 7,
            RepublicanMonth::Floréal => 8,
            RepublicanMonth::Prairial => 9,
            RepublicanMonth::Messidor => 10,
            RepublicanMonth::Thermidor => 11,
            RepublicanMonth::Fructidor => 12,
            RepublicanMonth::Sansculottides => 13,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_month_number() {
        assert_eq!(u8::from(RepublicanMonth::Vendémiaire), 1);
        assert_eq!(u8::from(RepublicanMonth::Brumaire), 2);
        assert_eq!(u8::from(RepublicanMonth::Frimaire), 3);
        assert_eq!(u8::from(RepublicanMonth::Nivôse), 4);
        assert_eq!(u8::from(RepublicanMonth::Pluviôse), 5);
        assert_eq!(u8::from(RepublicanMonth::Ventôse), 6);
        assert_eq!(u8::from(RepublicanMonth::Germinal), 7);
        assert_eq!(u8::from(RepublicanMonth::Floréal), 8);
        assert_eq!(u8::from(RepublicanMonth::Prairial), 9);
        assert_eq!(u8::from(RepublicanMonth::Messidor), 10);
        assert_eq!(u8::from(RepublicanMonth::Thermidor), 11);
        assert_eq!(u8::from(RepublicanMonth::Fructidor), 12);
        assert_eq!(u8::from(RepublicanMonth::Sansculottides), 13);
    }

    #[test]
    fn test_get_season() {
        assert_eq!(RepublicanMonth::Vendémiaire.get_season(), "Automne");
        assert_eq!(RepublicanMonth::Brumaire.get_season(), "Automne");
        assert_eq!(RepublicanMonth::Frimaire.get_season(), "Automne");
        assert_eq!(RepublicanMonth::Nivôse.get_season(), "Hiver");
        assert_eq!(RepublicanMonth::Pluviôse.get_season(), "Hiver");
        assert_eq!(RepublicanMonth::Ventôse.get_season(), "Hiver");
        assert_eq!(RepublicanMonth::Germinal.get_season(), "Printemps");
        assert_eq!(RepublicanMonth::Floréal.get_season(), "Printemps");
        assert_eq!(RepublicanMonth::Prairial.get_season(), "Printemps");
        assert_eq!(RepublicanMonth::Messidor.get_season(), "Été");
        assert_eq!(RepublicanMonth::Thermidor.get_season(), "Été");
        assert_eq!(RepublicanMonth::Fructidor.get_season(), "Été");
        assert_eq!(RepublicanMonth::Sansculottides.get_season(), "Été");
    }

    #[test]
    fn test_try_from_valid() {
        assert_eq!(RepublicanMonth::try_from(1), Ok(RepublicanMonth::Vendémiaire));
        assert_eq!(RepublicanMonth::try_from(2), Ok(RepublicanMonth::Brumaire));
        assert_eq!(RepublicanMonth::try_from(3), Ok(RepublicanMonth::Frimaire));
        assert_eq!(RepublicanMonth::try_from(4), Ok(RepublicanMonth::Nivôse));
        assert_eq!(RepublicanMonth::try_from(5), Ok(RepublicanMonth::Pluviôse));
        assert_eq!(RepublicanMonth::try_from(6), Ok(RepublicanMonth::Ventôse));
        assert_eq!(RepublicanMonth::try_from(7), Ok(RepublicanMonth::Germinal));
        assert_eq!(RepublicanMonth::try_from(8), Ok(RepublicanMonth::Floréal));
        assert_eq!(RepublicanMonth::try_from(9), Ok(RepublicanMonth::Prairial));
        assert_eq!(RepublicanMonth::try_from(10), Ok(RepublicanMonth::Messidor));
        assert_eq!(RepublicanMonth::try_from(11), Ok(RepublicanMonth::Thermidor));
        assert_eq!(RepublicanMonth::try_from(12), Ok(RepublicanMonth::Fructidor));
        assert_eq!(RepublicanMonth::try_from(13), Ok(RepublicanMonth::Sansculottides));
    }

    #[test]
    fn test_try_from_invalid() {
        assert_eq!(RepublicanMonth::try_from(0), Err("Invalid month number: 0".into()));
        assert_eq!(RepublicanMonth::try_from(14), Err("Invalid month number: 14".into()));
        assert_eq!(RepublicanMonth::try_from(255), Err("Invalid month number: 255".into()));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", RepublicanMonth::Vendémiaire), "Vendémiaire");
        assert_eq!(format!("{}", RepublicanMonth::Brumaire), "Brumaire");
        assert_eq!(format!("{}", RepublicanMonth::Pluviôse), "Pluviôse");
        assert_eq!(format!("{}", RepublicanMonth::Ventôse), "Ventôse");
        assert_eq!(format!("{}", RepublicanMonth::Floréal), "Floréal");
        assert_eq!(format!("{}", RepublicanMonth::Messidor), "Messidor");
        assert_eq!(format!("{}", RepublicanMonth::Thermidor), "Thermidor");
        assert_eq!(format!("{}", RepublicanMonth::Fructidor), "Fructidor");
        assert_eq!(format!("{}", RepublicanMonth::Sansculottides), "Sansculottides");
    }

    #[test]
    fn test_roundtrip_number_conversion() {
        for month_number in 1..=13 {
            let month = RepublicanMonth::try_from(month_number).unwrap();
            assert_eq!(u8::from(month), month_number);
        }
    }

}