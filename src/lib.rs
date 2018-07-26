//! NordSelect is a small library to find the best NordVPN servers for your needs.
//!
//! Included is a small CLI that uses most of the functionality. Usage of that can be found
//! [here](https://editicalu.github.io/nordselect)
//!
//!
//! # Example
//! ```
//! use nordselect::{Protocol, Servers};
//!
//! fn main() {
//!     // Get data    
//!     let mut servers = Servers::from_api().unwrap();
//!
//!     // Filter: only servers in Canada
//!     servers.filter_country("CA");
//!     // Filter: only TCP compatible servers
//!     servers.filter_protocol(nordselect::Protocol::Tcp);
//!
//!     // Sort the servers on load.
//!     servers.sort_load();
//!
//!     assert!(servers.get_perfect_server().is_some());
//! }
//! ```

extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate oping;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Deserialize, PartialEq, Clone)]
/// The categories a Server can be in.
pub enum CategoryType {
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

impl From<String> for CategoryType {
    fn from(input: String) -> CategoryType {
        match input.as_ref() {
            "Standard VPN servers" => CategoryType::Standard,
            "P2P" => CategoryType::P2P,
            "Double VPN" => CategoryType::Double,
            "Onion Over VPN" => CategoryType::Tor,
            "Obfuscated Servers" => CategoryType::Obfuscated,
            "Dedicated IP servers" => CategoryType::Dedicated,
            server_type => {
                eprintln!("Warning: unknown server type: {}", server_type);
                eprintln!("Please report an issue at https://github.com/editicalu/nordselect");
                CategoryType::UnknownServer
            }
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
/// The struct used to identify categories.
struct Category {
    /// The name of the category (converted into a type)
    pub name: CategoryType,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Deserialize, PartialEq, Clone)]
/// A server by NordVPN.
pub struct Server {
    /// The country this server is located in.
    flag: String,
    /// The domain of this server.
    pub domain: String,
    /// The current load on this server.
    load: u8,
    /// Categories this server is in.
    categories: Vec<Category>,
    /// Features of the server
    features: Features,
    ping: Option<usize>,
}

/// Ping operations
impl Server {
    /// Ping per instance.
    fn ping_single(&mut self, tries: usize) -> Result<(), Box<std::error::Error>> {
        let sum: usize = {
            let mut sum = 0usize;
            for _ in 0..tries {
                let mut pingr = oping::Ping::new();
                pingr.add_host(&self.domain)?;
                sum = sum + pingr.send().unwrap().next().unwrap().latency_ms as usize;
            }
            sum
        };
        self.ping = Some(sum / tries as usize);
        eprintln!("{} pinged {}", self.domain, self.ping.unwrap());
        Ok(())
    }
}

/// A list of individual servers.
pub struct Servers {
    /// The actual servers
    servers: Vec<Server>,
}

/// Functions to build and read data from the Servers.
impl Servers {
    /// Downloads the list of servers from the API.
    pub fn from_api() -> Result<Servers, Box<std::error::Error>> {
        let mut data = reqwest::get("https://api.nordvpn.com/server")?;
        let text = data.text()?;

        Ok(Servers {
            servers: serde_json::from_str(
                // TODO: find a better solution to these expensive replacements.
                &text.replace("Standard VPN servers", "Standard")
                    .replace("Obfuscated Servers", "Obfuscated")
                    .replace("Double VPN", "Double")
                    .replace("Onion Over VPN", "Tor")
                    .replace("Dedicated IP servers", "Dedicated"),
            )?,
        })
    }

    /// Returns the perfect server. This should be called when the filters are applied.
    pub fn get_perfect_server(&self) -> Option<Server> {
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
    pub fn filter_category(&mut self, category: CategoryType) {
        let category_struct = Category { name: category };
        (&mut self.servers).retain(|server| server.categories.contains(&category_struct));
    }

    /// Filters the servers on a certain protocol.
    pub fn filter_protocol(&mut self, protocol: Protocol) {
        match protocol {
            Protocol::Tcp => (&mut self.servers).retain(|server| server.features.openvpn_tcp),
            Protocol::Udp => (&mut self.servers).retain(|server| server.features.openvpn_udp),
        };
    }

    /// Filters the servers on a certain country.
    pub fn filter_country(&mut self, country: &str) {
        (&mut self.servers).retain(|server| server.flag == country)
    }

    /// Sorts the servers on their load.
    pub fn sort_load(&mut self) {
        (&mut self.servers).sort_unstable_by(|x, y| x.load.cmp(&y.load));
    }

    /// Sorts servers on ping result. Should only be called when all servers were able to ping.
    fn sort_ping(&mut self) {
        (&mut self.servers).sort_unstable_by(|x, y| x.ping.unwrap().cmp(&y.ping.unwrap()));
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
    pub fn benchmark_ping(
        &mut self,
        servers: usize,
        tries: usize,
        parallel: bool,
    ) -> Result<(), Box<std::error::Error>> {
        // Omit other servers
        self.cut(servers);

        if parallel {
            // TODO
        } else {
            self.servers
                .iter_mut()
                .for_each(|mut x| (&mut x).ping_single(tries).unwrap());
        };

        // No errors -> sort
        self.sort_ping();

        Ok(())
    }
}
