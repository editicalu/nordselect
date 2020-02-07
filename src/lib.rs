//! NordSelect is a small library to find the best NordVPN servers for your needs.
//!
//! This crate has a small CLI included that uses most of the functionality. Documentation of that
//! can be found [here](https://editicalu.github.io/nordselect) or in the README.
//!
//! # Example
//! ```
//! use nordselect::{ServerCategory, Protocol, Servers};
//! use nordselect::filters;
//! use nordselect::sorters;
//!
//! fn main() {
//!     // Get data
//!     let mut servers = Servers::from_api().unwrap();
//!
//!     // Filter: only servers in Canada
//!     servers.filter(&filters::CountryFilter::from_code("CA".to_string()));
//!     // Filter: only TCP compatible servers
//!     servers.filter(&filters::ProtocolFilter::from(Protocol::Tcp));
//!     // Filter: only standard servers
//!     servers.filter(&filters::CategoryFilter::from(ServerCategory::Standard));
//!
//!     // Sort the servers on load.
//!     servers.sort(&sorters::LoadSorter);
//!
//!     assert!(servers.perfect_server().is_some());
//! }
//! ```

pub mod filters;
pub mod servers;
pub mod sorters;

pub use crate::servers::Protocol;
pub use crate::servers::Server;
pub use crate::servers::ServerCategory;
pub use crate::servers::Servers;
