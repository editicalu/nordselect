use super::prelude::*;
use crate::servers::DOMAIN_REGEX;
use std::collections::HashSet;
use std::error::Error;

use std::io::BufRead;

/// Reads a list of servers from a file.
async fn read_servers_from_file(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut servers = HashSet::new();

    let path = std::path::Path::new(path);
    let file = std::fs::File::open(path)?;

    for line in std::io::BufReader::new(file).lines() {
        let line = line?;
        if line.len() != 0 && DOMAIN_REGEX.captures(&line).is_some() {
            servers.insert(line);
        }
    }
    Ok(servers)
}

/// Reads a list of servers from a url.
async fn read_servers_from_url(url: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let reqwest_part_1 = reqwest::get(url).await?.text().await?;
    let reqwest_part_2 = reqwest_part_1.lines();

    let expected_amount = reqwest_part_2.size_hint().1.unwrap_or(2000);
    let mut servers: HashSet<String> = HashSet::with_capacity(expected_amount);

    for server in reqwest_part_2
        .filter(|line| line.len() != 0)
        .filter(|line| DOMAIN_REGEX.captures(line).is_some())
    {
        servers.insert(String::from(server));
    }
    Ok(servers)
}

/// Filter that uses a whitelist to indicate whether a server can be passed or not. It will allow servers that appear on the blacklist.
///
/// Assumes the blacklist consists of full domain names of servers.
pub struct WhiteListFilter {
    whitelist: HashSet<String>,
}

impl Default for WhiteListFilter {
    fn default() -> Self {
        Self {
            whitelist: HashSet::new(),
        }
    }
}

impl WhiteListFilter {
    /// Downloads a whitelist and reads it in.
    pub async fn from_url(url: &str) -> Result<Self, Box<dyn Error>> {
        read_servers_from_url(url).await.map(|server_list| Self {
            whitelist: server_list,
        })
    }

    /// Read a Whitelist from a file.
    pub async fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        read_servers_from_file(path).await.map(|server_list| Self {
            whitelist: server_list,
        })
    }
}

impl Filter for WhiteListFilter {
    fn filter(&self, server: &Server) -> bool {
        self.whitelist.contains(&server.domain)
    }
}

/// Filter that uses a whitelist to indicate whether a server can be passed or not. It will allow servers that appear on the blacklist.
///
/// Assumes the blacklist consists of full domain names of servers.
pub struct BlackListFilter {
    blacklist: HashSet<String>,
}

impl Default for BlackListFilter {
    fn default() -> Self {
        Self {
            blacklist: HashSet::with_capacity(0),
        }
    }
}

impl BlackListFilter {
    /// Downloads a whitelist and reads it in.
    pub async fn from_url(url: &str) -> Result<Self, Box<dyn Error>> {
        read_servers_from_url(url).await.map(|server_list| Self {
            blacklist: server_list,
        })
    }

    /// Read a Whitelist from a file.
    pub async fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        read_servers_from_file(path).await.map(|server_list| Self {
            blacklist: server_list,
        })
    }
}

impl Filter for BlackListFilter {
    fn filter(&self, server: &Server) -> bool {
        !self.blacklist.contains(&server.domain)
    }
}
