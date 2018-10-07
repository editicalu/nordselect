//! The filters module consists of the Filter trait (used to implement filters) and several common inplementations of it.

use super::{Protocol, Server, ServerCategory};
use std::collections::HashSet;
use std::iter::FromIterator;

/// Way to reduce the amount of available servers.
pub trait Filter {
    /// Returns whether this server fullfills the needs of the Filter. When false, the given server
    /// should be removed from the set.
    fn filter(&self, &Server) -> bool;
}

/// Filter to only use servers from one specific country.
///
/// # Example
///
/// ```
/// use nordselect::Servers;
/// use nordselect::filters::CountryFilter;
///
/// let mut data = Servers::dummy_data();
/// data.filter(&CountryFilter::from("BE"));
///
/// assert_eq!(data.perfect_server().unwrap().flag, "BE");
/// ```
pub struct CountryFilter {
    /// The country on which to filter, noted according to
    /// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    country: String,
}

/// Ways to construct a CountryFilter.
impl CountryFilter {
    /// Creates a CountryFilter from the given country. The countrycode should be an
    /// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    #[deprecated(
        since = "1.0.0",
        note = "Inefficient, use the From-trait implementation instead"
    )]
    pub fn from_code(countrycode: String) -> CountryFilter {
        CountryFilter {
            country: countrycode.to_ascii_uppercase(),
        }
    }
}

impl Filter for CountryFilter {
    fn filter(&self, server: &Server) -> bool {
        self.country == server.flag
    }
}

impl<'a> From<&'a str> for CountryFilter {
    fn from(countrycode: &str) -> CountryFilter {
        CountryFilter {
            country: countrycode.to_ascii_uppercase(),
        }
    }
}

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
/// use nordselect::filters::CountriesFilter;
///
/// let mut data = Servers::dummy_data();
///
/// // Countries of the European Union.
/// data.filter(&CountriesFilter::from_region("EU").unwrap());
///
/// // The country will be one of the EU.
/// assert!(
///     CountriesFilter::region_countries("EU").unwrap()
///         .contains(&data.perfect_server().unwrap().flag.as_ref()));
/// ```
pub struct CountriesFilter {
    /// Countries which are allowed.
    countries: HashSet<String>,
}

/// Region operations
impl CountriesFilter {
    /// Builds a CountriesFilter from one of the provided regions. Regions should be given in the
    /// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) format, but can be
    /// uppercase or lowercase.
    ///
    /// When calling this with one of the `[available_regions](method.available_regions)` will
    /// always return `Some(CountriesFilter)`.
    #[deprecated(
        since = "1.1.0",
        note = "Use the Region object instead. It has more regions and works better."
    )]
    #[allow(deprecated)]
    pub fn from_region(region: &str) -> Option<CountriesFilter> {
        match region.to_lowercase().as_ref() {
            "eu" | "ею" => Some(CountriesFilter {
                countries: HashSet::from_iter(
                    Self::region_countries("EU")
                        .unwrap()
                        .iter()
                        .map(|s| String::from(*s)),
                ),
            }),
            _ => None,
        }
    }

    /// Returns regions that can be used.
    ///
    /// When calling [from_region](method.from_region) with one of the values in the returned slice
    /// should always give a `Some`-value.
    #[deprecated(
        since = "1.1.0",
        note = "Use the Region object instead. It has more regions and works better."
    )]
    pub fn available_regions() -> &'static [&'static str] {
        &["EU", "ЕЮ"]
    }

    /// Returns the countries that are represented by the given region. Regions should be in
    /// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) format.
    #[deprecated(
        since = "1.1.0",
        note = "Use the Region object instead. It has more regions and works better."
    )]
    pub fn region_countries(region: &str) -> Option<&'static [&'static str]> {
        match region.as_ref() {
            "EU" | "ЕЮ" => Some(&[
                "AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IE",
                "IT", "LV", "LT", "LU", "MT", "NL", "PL", "PT", "RO", "SK", "SI", "ES", "SE",
            ]),
            _ => None,
        }
    }
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

/// Filter that keeps only servers that accept a specific protocol.
///
/// # Example
///
/// ```
/// use nordselect::Servers;
/// use nordselect::Protocol;
/// use nordselect::filters::ProtocolFilter;
/// let mut data = Servers::dummy_data();
///
/// // Filter on the TCP protocol
/// data.filter(&ProtocolFilter::from(Protocol::Tcp));
///
/// assert!(data.perfect_server().is_some());
/// ```
pub struct ProtocolFilter {
    /// The protocol that should be filtered against.
    protocol: Protocol,
}

impl From<Protocol> for ProtocolFilter {
    fn from(protocol: Protocol) -> ProtocolFilter {
        ProtocolFilter { protocol }
    }
}

impl Filter for ProtocolFilter {
    fn filter(&self, server: &Server) -> bool {
        match self.protocol {
            Protocol::Tcp => server.features.openvpn_tcp,
            Protocol::Udp => server.features.openvpn_udp,
            Protocol::Pptp => server.features.pptp,
            Protocol::L2tp => server.features.l2tp,
        }
    }
}

/// Filter that keeps servers with less or equal load compared to a provided value.
///
/// # Example
///
/// ```
/// use nordselect::Servers;
/// use nordselect::filters::LoadFilter;
/// let mut data = Servers::dummy_data();
///
/// // Filter on 10% load or less.
/// data.filter(&LoadFilter::from(10));
///
/// assert!(data.perfect_server().is_some());
/// ```
pub struct LoadFilter {
    /// The maximal allowed load.
    load: u8,
}

impl From<u8> for LoadFilter {
    fn from(load: u8) -> LoadFilter {
        LoadFilter { load }
    }
}

impl Filter for LoadFilter {
    fn filter(&self, server: &Server) -> bool {
        use std;
        server.load.cmp(&self.load) != std::cmp::Ordering::Greater
    }
}

/// Filter that contains multiple Filter instances. This could be more efficient, as only servers
/// fullfilling all requirements are kept.
///
/// Logically, this should be viewed as an AND-gate, as every `Filter` should allow the server to
/// be kept.
pub struct CombinedFilter {
    // The actual filters
    filters: Vec<Box<Filter>>,
}

/// Ways to construct `CombinedFilters`.
impl CombinedFilter {
    /// Builds a new `CombinedFilter`.
    pub fn new() -> CombinedFilter {
        CombinedFilter {
            filters: Vec::new(),
        }
    }

    /// Builds a new `CombinedFilter` with the given capacity.
    pub fn with_capacity(capacity: usize) -> CombinedFilter {
        CombinedFilter {
            filters: Vec::with_capacity(capacity),
        }
    }
}

impl From<Vec<Box<Filter>>> for CombinedFilter {
    fn from(filters: Vec<Box<Filter>>) -> CombinedFilter {
        CombinedFilter { filters }
    }
}

impl CombinedFilter {
    /// Adds a new filter
    pub fn add_filter(&mut self, filter: Box<Filter>) {
        self.filters.push(filter);
    }
}

impl Filter for CombinedFilter {
    fn filter(&self, server: &Server) -> bool {
        self.filters
            .iter()
            // Sorry for the confusing line of Rust code.
            .filter(|filter| filter.filter(server))
            .next()
            .is_some()
    }
}

/// Filter the Servers using a given category.
///
/// # Example
///
/// ```
/// use nordselect::{Servers, ServerCategory};
/// use nordselect::filters::CategoryFilter;
/// let mut data = Servers::dummy_data();
///
/// // Filter on Standard servers.
/// data.filter(&CategoryFilter::from(ServerCategory::Standard));
///
/// assert!(data.perfect_server().is_some());
/// ```
pub struct CategoryFilter {
    category: ServerCategory,
}

impl From<ServerCategory> for CategoryFilter {
    fn from(category: ServerCategory) -> CategoryFilter {
        CategoryFilter { category }
    }
}

impl Filter for CategoryFilter {
    fn filter(&self, server: &Server) -> bool {
        server.categories.contains(&self.category)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Servers;
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn country_filter_simple_legacy() {
        let mut data = Servers::dummy_data();

        data.filter(&CountryFilter::from_code("sg".to_string()));

        let server_opt = data.perfect_server();

        assert!(server_opt.is_some());
        assert_eq!(server_opt.unwrap().flag, "SG");
    }

    #[test]
    #[allow(deprecated)]
    fn country_filter_advanced_legacy() {
        let mut data = Servers::dummy_data();

        data.filter(&CountryFilter::from_code("Sg".to_string()));

        let server_opt = data.perfect_server();

        assert!(server_opt.is_some());
        assert_eq!(server_opt.unwrap().flag, "SG");
    }

    #[test]
    fn country_filter_simple() {
        let mut data = Servers::dummy_data();

        data.filter(&CountryFilter::from("sg"));

        let server_opt = data.perfect_server();

        assert!(server_opt.is_some());
        assert_eq!(server_opt.unwrap().flag, "SG");
    }

    #[test]
    fn country_filter_advanced() {
        let mut data = Servers::dummy_data();

        data.filter(&CountryFilter::from("Sg"));

        let server_opt = data.perfect_server();

        assert!(server_opt.is_some());
        assert_eq!(server_opt.unwrap().flag, "SG");
    }

    #[test]
    #[allow(deprecated)]

    fn countries_filter_regions_give_some() {
        for region in CountriesFilter::available_regions() {
            assert!(CountriesFilter::from_region(region).is_some());
        }
    }

    #[test]
    fn countries_filter_empty() {
        let mut data = Servers::dummy_data();

        data.filter(&CountriesFilter::from(HashSet::with_capacity(0)));

        let server_opt = data.perfect_server();

        assert_eq!(server_opt, None);
    }

    #[test]
    fn countries_filter_simple() {
        let mut data = Servers::dummy_data();

        data.filter(&CountriesFilter::from(HashSet::from_iter(
            vec!["AX", "AY", "AZ"].into_iter().map(|x| x.to_string()),
        )));

        let server_opt = data.perfect_server();

        assert!(server_opt.is_some());
        assert_eq!(server_opt.unwrap().flag, "AZ");
    }

    #[test]
    fn valid_regions() {
        assert_eq!(
            Region::from_str("EU").unwrap().countries(),
            vec![
                "AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IE",
                "IT", "LV", "LT", "LU", "MT", "NL", "PL", "PT", "RO", "SK", "SI", "ES", "SE",
            ]
        );
        assert_eq!(
            Region::from_str("ЕЮ").unwrap().countries(),
            vec![
                "AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IE",
                "IT", "LV", "LT", "LU", "MT", "NL", "PL", "PT", "RO", "SK", "SI", "ES", "SE",
            ]
        );
        assert_eq!(
            Region::from_str("5E").unwrap().countries(),
            vec!["AU", "CA", "NZ", "GB", "US"]
        );
        assert_eq!(
            Region::from_str("6E").unwrap().countries(),
            vec!["AU", "CA", "FR", "NZ", "GB", "US"]
        );
        assert_eq!(
            Region::from_str("9E").unwrap().countries(),
            vec!["AU", "CA", "DK", "FR", "NL", "NO", "NZ", "GB", "US"]
        );
        assert_eq!(
            Region::from_str("14E").unwrap().countries(),
            vec![
                "AU", "BE", "CA", "DE", "DK", "ES", "FR", "IT", "NL", "NO", "NZ", "GB", "SE", "US",
            ],
        );

        // Make sure we do not forget a region
        for (region, _) in Region::from_str_options().into_iter() {
            assert!(Region::from_str(region).is_some());
        }
    }

    #[test]
    fn invalid_regions() {
        assert_eq!(Region::from_str("blablabla"), None);
        assert_eq!(Region::from_str(""), None);
        assert_eq!(Region::from_str("idk"), None);
        assert_eq!(Region::from_str("test"), None);
        assert_eq!(Region::from_str("12e"), None);
        assert_eq!(Region::from_str("15e"), None);
    }
}
