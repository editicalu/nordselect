//! Data structures and methods to interact with the NordVPN servers.
use oping;
use reqwest;
use serde_json;
use std;

use filters::Filter;
use sorters::Sorter;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
/// The categories a Server can be in.
pub enum ServerCategory {
    /// A standard VPN server
    Standard,
    /// A VPN server with P2P services allowed.
    P2P,
    /// A VPN server with a obfuscated IP (i.e. floating IP).
    Obfuscated,
    /// A VPN server with a dedicated IP, which is used only by one VPN user at a time.
    Dedicated,
    /// A VPN server with Tor/Onion funcitonality
    Tor,
    /// A VPN server that can be used to connect to another NordVPN server.
    Double,
    /// A VPN server that has a category that is not recognised.
    UnknownServer,
}

impl From<String> for ServerCategory {
    fn from(input: String) -> ServerCategory {
        match input.as_ref() {
            "Standard VPN servers" => ServerCategory::Standard,
            "P2P" => ServerCategory::P2P,
            "Double VPN" => ServerCategory::Double,
            "Onion Over VPN" => ServerCategory::Tor,
            "Obfuscated Servers" => ServerCategory::Obfuscated,
            "Dedicated IP" => ServerCategory::Dedicated,
            _ => ServerCategory::UnknownServer,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
/// The struct used to identify categories, used in the API.
struct ApiCategory {
    /// The name of the category (converted into a type)
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
/// All protocols and other features a Server can have.
pub struct Features {
    /// Support for IKEv2 protocol.
    pub ikev2: bool,
    /// Support for udp over openvpn
    pub openvpn_udp: bool,
    /// Support for tcp over openvpn
    pub openvpn_tcp: bool,
    /// Support for the SOCKS protocol.
    pub socks: bool,
    /// This server can be used as a proxy
    pub proxy: bool,
    /// Support for the older Point-to-Point Tunneling Protocol
    ///
    /// **Warning**: this protocol is considered unsafe. Usage is discouraged.
    ///
    /// From the NordVPN site:
    /// > Although technically you can use the L2TP/PPTP protocol, it has serious security flaws.
    /// > Whenever possible, we recommend choosing OpenVPN or IKEv2/IPSec instead.
    pub pptp: bool,
    /// Support for the Layer 2 Tunneling Protocol
    ///
    /// **Warning**: this protocol is considered unsafe. Usage is discouraged.
    ///
    /// From the NordVPN site:
    /// > Although technically you can use the L2TP/PPTP protocol, it has serious security flaws.
    /// > Whenever possible, we recommend choosing OpenVPN or IKEv2/IPSec instead.
    pub l2tp: bool,
    /// Support for udp over openvpn with xor obfuscation
    pub openvpn_xor_udp: bool,
    /// Support for tcp over openvpn with xor obfuscation
    pub openvpn_xor_tcp: bool,
    /// Support for a proxy with cybersec
    pub proxy_cybersec: bool,
    /// Support for a proxy with SSL
    pub proxy_ssl: bool,
    /// Support for a proxy with cybersec and SSL
    pub proxy_ssl_cybersec: bool,
}

#[derive(Debug, Deserialize)]
/// The way servers are represented in the API response.
struct ApiServer {
    /// The country this server is located in.
    pub flag: String,
    /// The domain of this server.
    pub domain: String,
    /// The current load on this server.
    pub load: u8,
    /// Categories this server is in.
    pub categories: Vec<ApiCategory>,
    /// Features of the server
    pub features: Features,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A server by NordVPN.
pub struct Server {
    /// The country this server is located in.
    pub flag: String,
    /// The domain of this server.
    pub domain: String,
    /// The current load on this server.
    pub load: u8,
    /// Categories this server is in.
    pub categories: Vec<ServerCategory>,
    /// Features of the server
    pub features: Features,
    /// Result of the ping test.
    #[deprecated(
        since = "0.3.3",
        note = "Ping tests no longer belong in the Server. Please use `LoadSorter` instead."
    )]
    pub ping: Option<usize>,
}

impl Hash for Server {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.domain.hash(hasher);
    }
}

impl From<ApiServer> for Server {
    fn from(api_server: ApiServer) -> Server {
        Server {
            flag: api_server.flag,
            domain: api_server.domain,
            load: api_server.load,
            categories: Vec::from_iter(
                api_server
                    .categories
                    .into_iter()
                    .map(|server_type| ServerCategory::from(server_type.name)),
            ),
            features: api_server.features,
            ping: None,
        }
    }
}

/// Ping operations
impl Server {
    /// Ping per instance.
    #[deprecated(
        since = "0.3.3",
        note = "Ping tests no longer belong in the Server. Please use `LoadSorter` instead."
    )]
    fn ping_single(&mut self, tries: usize) -> Result<(), Box<std::error::Error>> {
        let sum: usize = {
            let mut sum = 0usize;
            for _ in 0..tries {
                let mut pingr = oping::Ping::new();
                pingr.add_host(&self.domain)?;
                sum = sum + pingr.send()?.next().unwrap().latency_ms as usize;
            }
            sum
        };
        self.ping = Some(sum / tries as usize);
        Ok(())
    }
}

/// Non-ping operations.
impl Server {
    /// Returns the unique identifier of the server, without returning the full domain.
    ///
    /// This name is extracted from the `Server` everytime the function is called.
    pub fn name(&self) -> Option<&str> {
        use regex::Regex;
        let re = Regex::new(r"(.+)\.nordvpn.com").unwrap();
        let caps = match re.captures(&self.domain) {
            Some(caps) => caps,
            None => {
                return None;
            }
        };
        match caps.get(1) {
            Some(matches) => Some(matches.as_str()),
            None => None,
        }
    }
}

/// A list of individual servers.
pub struct Servers {
    /// The actual servers
    pub servers: Vec<Server>,
}

/// Functions to build and read data from the Servers.
impl Servers {
    fn from_txt(txt: &str) -> Result<Servers, Box<std::error::Error>> {
        let api_servers: Vec<ApiServer> = serde_json::from_str(&txt)?;

        Ok(Servers {
            servers: Vec::from_iter(
                api_servers
                    .into_iter()
                    .map(|api_server| Server::from(api_server)),
            ),
        })
    }

    /// Downloads the list of servers from the API.
    pub fn from_api() -> Result<Servers, Box<std::error::Error>> {
        let mut data = reqwest::get("https://api.nordvpn.com/server")?;
        let text = data.text()?;

        Self::from_txt(&text)
    }

    /// Returns the Servers from the API call on Sept. 8th 17:00 UTC. Use this only in benchmarks
    /// and examples in documentation.
    pub fn dummy_data() -> Servers {
        let text = std::fs::read_to_string("dummydata").unwrap();
        Self::from_txt(&text).unwrap()
    }

    #[deprecated(since = "0.3.2", note = "please use `flags` instead")]
    pub fn get_flags(&self) -> HashSet<&str> {
        self.flags()
    }

    pub fn flags(&self) -> HashSet<&str> {
        HashSet::from_iter(self.servers.iter().map(|server| server.flag.as_ref()))
    }

    #[deprecated(since = "0.3.2", note = "please use `perfect_server` instead")]
    pub fn get_perfect_server(&self) -> Option<Server> {
        self.perfect_server()
    }

    /// Returns the perfect server. This should be called when the filters are applied.
    pub fn perfect_server(&self) -> Option<Server> {
        match self.servers.get(0) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
}

#[derive(PartialEq)]
/// A protocol to connect to the VPN server.
pub enum Protocol {
    /// The [User Datagram Protocol](https://en.wikipedia.org/wiki/User_Datagram_Protocol)
    Udp,
    /// The [Transmission Control Protocol](https://en.wikipedia.org/wiki/Transmission_Control_Protocol)
    Tcp,
}

/// All filters that can be applied.
impl Servers {
    /// Filters the servers on a certain category.
    #[deprecated(since = "0.3.3", note = "please use `CategoryFilter` instead")]
    pub fn filter_category(&mut self, category: ServerCategory) {
        (&mut self.servers).retain(|server| server.categories.contains(&category));
    }

    /// Filters the servers on a certain protocol.
    #[deprecated(since = "0.3.3", note = "please use `ProtocolFilter` instead")]
    pub fn filter_protocol(&mut self, protocol: Protocol) {
        match protocol {
            Protocol::Tcp => (&mut self.servers).retain(|server| server.features.openvpn_tcp),
            Protocol::Udp => (&mut self.servers).retain(|server| server.features.openvpn_udp),
        };
    }

    /// Filters the servers on a certain country.
    #[deprecated(since = "0.3.3", note = "please use `CountryFilter` instead")]
    pub fn filter_country(&mut self, country: &str) {
        (&mut self.servers).retain(|server| server.flag == country)
    }

    /// Filters the servers on a set of countries. It retains servers from all these countries.
    #[deprecated(since = "0.3.3", note = "please use `CountriesFilter` instead")]
    pub fn filter_countries(&mut self, countries: &HashSet<String>) {
        (&mut self.servers).retain(|server| countries.contains(&server.flag))
    }

    /// Applies the given filter on this serverlist.
    pub fn filter(&mut self, filter: &Filter) {
        (&mut self.servers).retain(|server| filter.filter(&server))
    }

    #[deprecated(since = "0.3.3", note = "please use `LoadSorter` instead")]
    /// Sorts the servers on their load.
    pub fn sort_load(&mut self) {
        (&mut self.servers).sort_unstable_by(|x, y| x.load.cmp(&y.load));
    }

    #[deprecated(since = "0.3.3", note = "please use `PingSorter` instead")]
    /// Sorts servers on ping result. Should only be called when all servers were able to ping.
    fn sort_ping(&mut self) {
        (&mut self.servers).sort_unstable_by(|x, y| x.ping.unwrap().cmp(&y.ping.unwrap()));
    }

    /// Sorts the servers using a Sorter. The sort is unstable.
    pub fn sort(&mut self, sorter: &Sorter) {
        (&mut self.servers).sort_unstable_by(|x, y| sorter.sort(x, y));
    }

    /// Removes all but the `max` best servers at the moment. Does nothing if there are less
    /// servers.
    pub fn cut(&mut self, max: usize) {
        self.servers.truncate(max);
    }

    /// Benchmark the given amount of first servers in the list based upon their ping latency.
    /// Omits other servers.
    ///
    /// Returns `Ok(())` when succeeded. Returns an `Box<Error>` otherwise.
    ///
    /// - `servers`: amount of best servers that should be tested.
    /// - `tries`: how many tries should be done. The average ping is taken.
    /// - `parallel`: whether the tests should be run in parallel. This will make tests go faster,
    ///   but less accurate.
    #[deprecated(since = "0.3.3", note = "please use `PingSorter` instead")]
    pub fn benchmark_ping(
        &mut self,
        servers: usize,
        tries: usize,
        _parallel: bool,
    ) -> Result<(), Box<std::error::Error>> {
        // Omit other servers
        self.cut(servers);
        for mut server in &mut self.servers {
            (&mut server).ping_single(tries)?;
        }

        // No errors -> sort
        self.sort_ping();

        Ok(())
    }
}
