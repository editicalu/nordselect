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
/// // Filter on 10-40% load.
/// data.filter(&LoadFilter::from((10, 40)));
/// 
/// assert!(data.perfect_server().load > 10);
/// ```
pub struct LoadFilter {
    /// minimum allowed load
    min_load: u8,
    /// maximum allowed load
    max_load: u8,
}

impl From<(u8,u8)> for LoadFilter {
    fn from(loads: (u8, u8)) -> LoadFilter {
        LoadFilter { min_load: loads.0, max_load: loads.1 }
    }
}

impl Filter for LoadFilter {
    /// A server's load has to be Greater than the min_load
    /// and Less than the max_load provided.
    fn filter(&self, server: &Server) -> bool {
        server.load.cmp(&self.min_load) == std::cmp::Ordering::Greater &&
        server.load.cmp(&self.max_load) == std::cmp::Ordering::Less
    }
}
