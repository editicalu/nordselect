//! The filters module consists of the Filter trait (used to implement filters) and several common inplementations of it.

use super::Server;

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
        CountryFilter { country }
    }
}

impl Filter for CountryFilter {
    fn filter(&self, server: &Server) -> bool {
        self.country == server.flag
    }
}
