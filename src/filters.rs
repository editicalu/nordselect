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
/// data.filter(&CountryFilter::from_code("BE".to_string()));
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
    #[deprecated(since = "1.0.0", note = "Inefficient, use the From-trait implementation instead")]
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
    pub fn available_regions() -> &'static [&'static str] {
        &["EU", "ЕЮ"]
    }

    /// Returns the countries that are represented by the given region. Regions should be in
    /// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) format.
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
    fn country_filter_simple() {
        let mut data = Servers::dummy_data();

        data.filter(&CountryFilter::from_code("sg"));

        let server_opt = data.perfect_server();

        assert!(server_opt.is_some());
        assert_eq!(server_opt.unwrap().flag, "SG");
    }

    #[test]
    fn country_filter_advanced() {
        let mut data = Servers::dummy_data();

        data.filter(&CountryFilter::from_code("Sg"));

        let server_opt = data.perfect_server();

        assert!(server_opt.is_some());
        assert_eq!(server_opt.unwrap().flag, "SG");
    }

    #[test]
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
}
