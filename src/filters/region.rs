use super::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub enum Region {
    /// The [European Union](https://en.wikipedia.org/wiki/European_Union), consisting of 27 countries.
    ///
    /// Because of the Brexit, the United Kingdom is not included in this region
    EuropeanUnion,
    /// The European Economic Area, consisting of the European Union, Norway, Lichtenstein and Iceland.
    EuropeanEconomicArea,
    /// The Benelux consists of Belgium, The Netherlands and Luxembourgh
    Benelux,
    /// [5 eyes programme countries](https://en.wikipedia.org/wiki/Five_Eyes)
    FiveEyes,
    /// [6 eyes programme countries.](https://en.wikipedia.org/wiki/Five_Eyes#Other_international_cooperatives)
    SixEyes,
    /// [9 eyes programme countries.](https://en.wikipedia.org/wiki/Five_Eyes#Other_international_cooperatives)
    NineEyes,
    /// [14 eyes programme countries.](https://en.wikipedia.org/wiki/Five_Eyes#Other_international_cooperatives)
    FourteenEyes,
}

impl Region {
    /// Tries to create a Region from a string slice. Returns a Region if there's one represented
    /// by your str slice. Returns None otherwise.
    ///
    /// The provided str slice should be **uppercase**!
    pub fn from_str(region_short: &str) -> Option<Region> {
        match region_short {
            "EU" | "ЕЮ" => Some(Region::EuropeanUnion),
            "EEA" => Some(Region::EuropeanEconomicArea),
            "BENELUX" => Some(Region::Benelux),
            "5E" => Some(Region::FiveEyes),
            "6E" => Some(Region::SixEyes),
            "9E" => Some(Region::NineEyes),
            "14E" => Some(Region::FourteenEyes),
            _ => None,
        }
    }

    /// Returns all possible region codes with their respective meanings in human readable form.
    /// Useful to provide lists to your users to choose from.
    ///
    /// Using a value from index 0 of the tuple will guaranteed give a Some when calling `[from_str](#method_from_str)`
    pub fn from_str_options() -> [(&'static str, &'static str); 8] {
        [
            ("EU", "The European Union"),
            ("ЕЮ", "The European Union (Cyrillic notation)"),
            ("EEA", "The European Economic Area"),
            ("BENELUX", "Countries of the Benelux"),
            ("5E", "Countries involved in the Five Eyes programme."),
            ("6E", "Countries involved in the Six Eyes programme."),
            ("9E", "Countries involved in the Nine Eyes programme."),
            ("14E", "Countries involved in the Fourteen Eyes programme."),
        ]
    }

    /// Returns the main short notation for a given Region.
    pub fn short(&self) -> &'static str {
        match self {
            Region::EuropeanUnion => "EU",
            Region::EuropeanEconomicArea => "EEA",
            Region::Benelux => "BENELUX",
            Region::FiveEyes => "5E",
            Region::SixEyes => "6E",
            Region::NineEyes => "9E",
            Region::FourteenEyes => "14E",
        }
    }

    pub fn countries(&self) -> Vec<&str> {
        match self {
            Region::EuropeanEconomicArea => vec![
                "AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IE",
                "IT", "LV", "LT", "LU", "MT", "NL", "PL", "PT", "RO", "SK", "SI", "ES", "SE", "NO",
                "LI", "IS",
            ],
            Region::EuropeanUnion => vec![
                "AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IE",
                "IT", "LV", "LT", "LU", "MT", "NL", "PL", "PT", "RO", "SK", "SI", "ES", "SE",
            ],
            Region::Benelux => vec!["BE", "LU", "NL"],
            Region::FiveEyes => vec!["AU", "CA", "NZ", "GB", "US"],
            Region::SixEyes => vec!["AU", "CA", "FR", "NZ", "GB", "US"],
            Region::NineEyes => vec!["AU", "CA", "DK", "FR", "NL", "NO", "NZ", "GB", "US"],
            Region::FourteenEyes => vec![
                "AU", "BE", "CA", "DE", "DK", "ES", "FR", "IT", "NL", "NO", "NZ", "GB", "SE", "US",
            ],
        }
    }
}

/// Filter that keeps servers from any of the provided countries.
///
/// This struct can be build from your own list of countries, or it can be used with one of the
/// provided regions. To see the available regions, use [CountriesFilter::available_regions()](#method.available_regions)
///
/// # Examples
/// ```
/// use nordselect::Servers;
/// use nordselect::filters::{CountriesFilter, Region};
///
/// let mut data = Servers::dummy_data();
///
/// // Countries of the European Union.
/// data.filter(&CountriesFilter::from(Region::EuropeanUnion));
///
/// // The country will be one of the EU.
/// assert!(
///     Region::EuropeanUnion.countries()
///         .contains(&data.perfect_server().unwrap().flag.as_ref()));
/// ```
pub struct CountriesFilter {
    /// Countries which are allowed.
    countries: HashSet<String>,
}

impl From<Region> for CountriesFilter {
    fn from(region: Region) -> CountriesFilter {
        CountriesFilter {
            countries: HashSet::from_iter(
                region
                    .countries()
                    .into_iter()
                    .map(|str_slice| String::from(str_slice)),
            ),
        }
    }
}

impl From<HashSet<String>> for CountriesFilter {
    fn from(countries: HashSet<String>) -> CountriesFilter {
        CountriesFilter { countries }
    }
}

impl Filter for CountriesFilter {
    fn filter(&self, server: &Server) -> bool {
        self.countries.contains(&server.flag)
    }
}
