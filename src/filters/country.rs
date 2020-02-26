use super::prelude::*;

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
