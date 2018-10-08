//! Data structures and methods to interact with the NordVPN servers.
use filters::Filter;
use reqwest;
use serde_json;
use sorters::Sorter;
use std;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
/// The categories a Server can be in, as used by NordVPN.
pub enum ServerCategory {
    /// A standard VPN server
    Standard,
    /// A VPN server with P2P services allowed.
    P2P,
    /// A VPN server with an obfuscated IP (i.e. floating IP).
    Obfuscated,
    /// A VPN server with a dedicated IP, which is used only by one VPN user at a time.
    Dedicated,
    /// A VPN server with [Tor](https://www.torproject.org) connections allowed.
    Tor,
    /// A VPN server that can be used to connect to another NordVPN server.
    Double,
    /// A VPN server that has a category that is not recognised by this library.\
    ///
    /// Should you ever encouter this in the API response, feel free to open an issue.
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
///
/// **Should only be used when parsing API data.**
struct ApiCategory {
    /// The name of the category (converted into a type)
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
/// All protocols and other features a Server can have.
pub struct Features {
    /// Support for IKEv2 protocol.
    pub ikev2: bool,
    /// Support for udp over OpenVPN
    pub openvpn_udp: bool,
    /// Support for tcp over OpenVPN
    pub openvpn_tcp: bool,
    /// Support for the SOCKS protocol.
    pub socks: bool,
    /// This server can be used as a proxy.
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
    /// Support for udp over OpenVPN with xor obfuscation
    pub openvpn_xor_udp: bool,
    /// Support for tcp over OpenVPN with xor obfuscation
    pub openvpn_xor_tcp: bool,
    /// Support for a proxy with CyberSec
    pub proxy_cybersec: bool,
    /// Support for a proxy with SSL
    pub proxy_ssl: bool,
    /// Support for a proxy with CyberSec and SSL
    pub proxy_ssl_cybersec: bool,
}

#[derive(Debug, Deserialize)]
/// The way servers are represented in the API response.
struct ApiServer {
    /// The country this server is located in.
    pub flag: String,
    /// The domain of this server.
    pub domain: String,
    /// The current load on this server, written as a percentage (%)
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
        }
    }
}

impl Server {
    /// Returns the unique identifier of the server, without returning the full domain.
    ///
    /// This name is extracted from the `Server` everytime the function is called. Use it only to
    /// create output.
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
    /// Creates a Servers by reading the given text.
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

    /// Downloads the list of servers from the API. Returns an error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// let data = nordselect::Servers::from_api();
    /// assert!(data.is_ok());
    /// ```
    pub fn from_api() -> Result<Servers, Box<std::error::Error>> {
        let mut data = reqwest::get("https://nordvpn.com/api/server")?;
        let text = data.text()?;

        Self::from_txt(&text)
    }

    /// Returns the data, fetched out of the `dummydata` file, generated using `dummydata.sh`.
    ///
    /// Use this only for debugging, testing and benchmarking.
    ///
    /// # Examples
    /// ```
    /// // If this fails, run `dummydata.sh` from the crate root.
    /// nordselect::Servers::dummy_data();
    /// ```
    pub fn dummy_data() -> Servers {
        let text = std::fs::read_to_string("dummydata").unwrap();
        Self::from_txt(&text).unwrap()
    }

    /// Returns a set with all the flags (countries) in this set.
    ///
    /// # Examples
    ///
    /// ```
    /// use nordselect::Servers;
    /// let data = Servers::dummy_data();
    ///
    /// assert!(data.flags().contains("BE"));
    /// assert!(data.flags().contains("US"));
    ///
    /// assert!(!data.flags().contains("XK")); // No servers in Kosovo
    /// assert!(!data.flags().contains("EU")); // The EU is not a country
    /// ```
    pub fn flags(&self) -> HashSet<&str> {
        HashSet::from_iter(self.servers.iter().map(|server| server.flag.as_ref()))
    }

    /// Returns the best server, according to the given values. This should be called after all the
    /// filters have been applied.
    ///
    /// Returns `None` if no `Server` fullfills all your needs.
    ///
    /// # Examples
    ///
    /// ```
    /// use nordselect::{Servers, filters};
    /// let mut data = Servers::dummy_data();
    ///
    /// data.filter(&filters::CountryFilter::from_code("".to_string()));
    /// assert_eq!(data.perfect_server(), None);
    ///
    /// let mut data = Servers::dummy_data();
    /// data.filter(&filters::CountryFilter::from_code("BE".to_string()));
    /// assert!(data.perfect_server().is_some());
    /// ```
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
    /// OpenVPN over the [User Datagram Protocol](https://en.wikipedia.org/wiki/User_Datagram_Protocol)
    Udp,
    /// OpenVPN over the [Transmission Control Protocol](https://en.wikipedia.org/wiki/Transmission_Control_Protocol)
    Tcp,
    /// The older Point-to-Point Tunneling Protocol
    ///
    /// **Warning**: this protocol is considered unsafe. Usage is discouraged.
    ///
    /// From the NordVPN site:
    /// > Although technically you can use the L2TP/PPTP protocol, it has serious security flaws.
    /// > Whenever possible, we recommend choosing OpenVPN or IKEv2/IPSec instead.
    Pptp,
    /// The Layer 2 Tunneling Protocol
    ///
    /// **Warning**: this protocol is considered unsafe. Usage is discouraged.
    ///
    /// From the NordVPN site:
    /// > Although technically you can use the L2TP/PPTP protocol, it has serious security flaws.
    /// > Whenever possible, we recommend choosing OpenVPN or IKEv2/IPSec instead.
    L2tp,
    /// OpenVPN over TCP with xor obfuscation
    OpenVPNXTcp,
    /// OpenVPN over UDP with xor obfuscation
    OpenVPNXUdp,
    /// Support for the SOCKS protocol.
    Socks,
    /// Support for a proxy with CyberSec
    CyberSecProxy,
    /// Support for a proxy with SSL
    SslProxy,
    /// Support for a proxy with CyberSec and SSL
    CyberSecSslProxy,
    /// Use the server as a proxy
    Proxy,
}

/// All manipulations that will alter the servers.
impl Servers {
    /// Applies the given filter on this serverlist.
    pub fn filter(&mut self, filter: &Filter) {
        (&mut self.servers).retain(|server| filter.filter(&server))
    }

    /// Sorts the servers using a Sorter. The sort is unstable.
    pub fn sort(&mut self, sorter: &Sorter) {
        (&mut self.servers).sort_unstable_by(|x, y| sorter.sort(x, y));
    }

    /// Removes all but the `max` best servers at the moment. Does nothing if there are less
    /// servers.
    ///
    /// 'Best' servers are defined by the filters and sorters that should have been applied before
    /// calling this function.
    pub fn cut(&mut self, max: usize) {
        self.servers.truncate(max);
    }
}
