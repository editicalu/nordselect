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

// Used to parse some data out of a string.
extern crate regex;
// Used to easily send GET requests.
extern crate reqwest;
/// Used to parse JSON data from the API.
#[macro_use]
extern crate serde_derive;
/// Used for ping functionality.
extern crate oping;
/// Used to parse JSON data from the API.
extern crate serde;
/// Used to parse JSON data from the API.
extern crate serde_json;

pub mod bench;
pub mod filters;
pub mod servers;
#[deprecated(since = "2.0.0", note = "Use the new bench module instead.")]
pub mod sorters;

pub use servers::Protocol;
pub use servers::Server;
pub use servers::ServerCategory;
pub use servers::Servers;
