//! NordSelect is a small library to find the best NordVPN servers for your needs.
//!
//! Included is a small CLI that uses most of the functionality. Usage of that can be found
//! [here](https://editicalu.github.io/nordselect)
//!
//!
//! # Example
//! ```
//! use nordselect::{CategoryType, Protocol, Servers};
//!
//! fn main() {
//!     // Get data
//!     let mut servers = Servers::from_api().unwrap();
//!
//!     // Filter: only servers in Canada
//!     servers.filter_country("CA");
//!     // Filter: only TCP compatible servers
//!     servers.filter_protocol(nordselect::Protocol::Tcp);
//!     // Filter: only standard servers
//!     servers.filter_category(CategoryType::Standard);
//!
//!     // Sort the servers on load.
//!     servers.sort_load();
//!
//!     assert!(servers.perfect_server().is_some());
//! }
//! ```

extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate oping;
extern crate serde;
extern crate serde_json;

pub mod filters;
pub mod servers;
pub mod sorters;

pub use servers::Protocol;
pub use servers::Server;
pub use servers::ServerCategory;
pub use servers::Servers;
