use super::prelude::*;

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
        server.load.cmp(&self.load) != std::cmp::Ordering::Greater
    }
}
