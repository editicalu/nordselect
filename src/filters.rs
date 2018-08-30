//! The filters module consists of the Filter trait (used to implement filters) and several common inplementations of it.

use super::{Protocol, Server};

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
    countries: Vec<String>,
}

/// Region operations
impl CountriesFilter {
    /// Builds a CountriesFilter from one of the provided regions.
    pub fn from_region(region: &str) -> Option<CountriesFilter> {
        match region.to_lowercase().as_ref() {
            "eu" | "ею" => Some(CountriesFilter {
                countries: vec![
                    String::from("AT"),
                    String::from("BE"),
                    String::from("BG"),
                    String::from("HR"),
                    String::from("CY"),
                    String::from("CZ"),
                    String::from("DK"),
                    String::from("EE"),
                    String::from("FI"),
                    String::from("FR"),
                    String::from("DE"),
                    String::from("GR"),
                    String::from("HU"),
                    String::from("IE"),
                    String::from("IT"),
                    String::from("LV"),
                    String::from("LT"),
                    String::from("LU"),
                    String::from("MT"),
                    String::from("NL"),
                    String::from("PL"),
                    String::from("PT"),
                    String::from("RO"),
                    String::from("SK"),
                    String::from("SI"),
                    String::from("ES"),
                    String::from("SE"),
                ],
            }),
            _ => None,
        }
    }

    /// Returns regions that can be used
    pub fn available_regions() -> Vec<&'static str> {
        vec!["eu", "ею"]
    }
}

impl From<Vec<String>> for CountriesFilter {
    fn from(countries: Vec<String>) -> CountriesFilter {
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
