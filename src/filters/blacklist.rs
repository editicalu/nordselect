use super::prelude::*;
use std::collections::HashSet;

/// Filter that uses a blacklist to indicate whether a server can be passed or not.
/// TODO: implement
pub struct BlackListFilter {
    blacklist: HashSet<String>,
}

impl BlackListFilter {}

impl Filter for BlackListFilter {
    fn filter(&self, server: &Server) -> bool {
        self.blacklist.contains(&server.domain)
    }
}
