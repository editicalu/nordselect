//! The filters module consists of the Filter trait (used to implement filters) and several common inplementations of it.

use super::Server;

/// Way to minify the amount of available servers.
pub trait Filter {
    /// Returns whether this server fullfills the needs of the Filter.
    fn filter(&self, &Server) -> bool;
}
