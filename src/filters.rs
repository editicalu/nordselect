//! The filters module consists of the Filter trait (used to implement filters) and several common inplementations of it.

use super::{Protocol, Server, ServerCategory};
use std::collections::HashSet;
use std::iter::FromIterator;

/// Way to minify the amount of available servers.
pub trait Filter {
    /// Returns whether this server fullfills the needs of the Filter.
    fn filter(&self, &Server) -> bool;
}

/// Filter to only use servers from one specific country.
pub struct CountryFilter {
    country: String,
}

impl CountryFilter {
    pub fn from_code(country: String) -> CountryFilter {
        CountryFilter {
            country: country.to_ascii_uppercase(),
        }
    }
}

impl Filter for CountryFilter {
    fn filter(&self, server: &Server) -> bool {
        self.country == server.flag
    }
}

/// Filter that keeps servers from any of the provided countries.
pub struct CountriesFilter {
    countries: HashSet<String>,
}

/// Region operations
impl CountriesFilter {
    /// Builds a CountriesFilter from one of the provided regions.
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
    pub fn available_regions() -> Vec<&'static str> {
        vec!["EU", "ЕЮ"]
    }

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

/// Filter that keeps servers that accept a specific protocol.
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

/// Filter that keeps servers with less load than a provided value.
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

/// Filter that contains multiple Filter instances. This could be more efficient, as only servers fullfilling all requirements are kept.
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
    fn from(source: Vec<Box<Filter>>) -> CombinedFilter {
        CombinedFilter { filters: source }
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
            .filter(|filter| filter.filter(server))
            .next()
            .is_some()
    }
}

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
