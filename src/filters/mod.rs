//! The filters module consists of the Filter trait (used to implement filters) and several common inplementations of it.

use super::servers::{Server, ServerCategory};

/// Way to reduce the amount of available servers.
pub trait Filter {
    /// Returns whether this server fullfills the needs of the Filter. When false, the given server
    /// should be removed from the set.
    fn filter(&self, server: &Server) -> bool;
}

mod prelude;

mod blacklist;
mod country;
mod load;
mod protocol;
mod region;

pub use self::blacklist::BlackListFilter;
pub use self::country::CountryFilter;
pub use self::load::LoadFilter;
pub use self::protocol::ProtocolFilter;
pub use self::region::{RegionFilter, Region};

// Will be deleted in 3.0.0
//#[deprecated(since="2.0.0")]
//type CountriesFilter = RegionFilter;

/// Filter that contains multiple Filter instances. This could be more efficient, as only servers
/// fullfilling all requirements are kept.
///
/// Logically, this should be viewed as an AND-gate, as every `Filter` should allow the server to
/// be kept.
pub struct CombinedFilter {
    // The actual filters
    filters: Vec<Box<dyn Filter>>,
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

impl From<Vec<Box<dyn Filter>>> for CombinedFilter {
    fn from(filters: Vec<Box<dyn Filter>>) -> CombinedFilter {
        CombinedFilter { filters }
    }
}

impl CombinedFilter {
    /// Adds a new filter
    pub fn add_filter(&mut self, filter: Box<dyn Filter>) {
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

/// Filter that negates the results of a given filter.
///
/// # Example
///
/// ```
/// use nordselect::Servers;
/// use nordselect::filters::{CountryFilter, NegatingFilter};
///
/// let mut data = Servers::dummy_data();
/// data.filter(&NegatingFilter::new(CountryFilter::from("BE")));
///
/// assert_ne!(data.perfect_server().unwrap().flag, "BE");
/// ```
pub struct NegatingFilter(Box<dyn Filter>);

impl NegatingFilter {
    pub fn new(filter: impl Filter + 'static) -> Self {
        Self(Box::new(filter))
    }
}

impl From<Box<dyn Filter + 'static>> for NegatingFilter {
    fn from(filter: Box<dyn Filter + 'static>) -> Self {
        Self(filter)
    }
}

impl Filter for NegatingFilter {
    fn filter(&self, server: &Server) -> bool {
        !self.0.filter(server)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Servers;
    use super::*;

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
    fn countries_filter_empty() {
        use std::collections::HashSet;
        let mut data = Servers::dummy_data();

        data.filter(&RegionFilter::from(HashSet::with_capacity(0)));

        let server_opt = data.perfect_server();

        assert_eq!(server_opt, None);
    }

    #[test]
    fn countries_filter_simple() {
        use std::collections::HashSet;
        use std::iter::FromIterator;

        let mut data = Servers::dummy_data();
        let vec = vec!["AE", "AL", "AR"];

        data.filter(&RegionFilter::from(HashSet::from_iter(
            vec.iter().map(|x| x.to_string()),
        )));

        let server_opt = data.perfect_server();

        assert!(server_opt.is_some());
        assert!(vec.contains(&server_opt.unwrap().flag.as_str()));
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
        for (region, _) in Region::from_str_options().iter() {
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
