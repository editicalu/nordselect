extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Deserialize, PartialEq)]
/// The categories a Server can be in.
enum CategoryType {
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

#[derive(Debug, Deserialize, PartialEq)]
struct Category {
    name: CategoryType,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Features {
    pub ikev2: bool,
    pub openvpn_udp: bool,
    pub openvpn_tcp: bool,
    pub socks: bool,
    pub proxy: bool,
    pub pptp: bool,
    pub l2tp: bool,
    pub openvpn_xor_udp: bool,
    pub openvpn_xor_tcp: bool,
    pub proxy_cybersec: bool,
    pub proxy_ssl: bool,
    pub proxy_ssl_cybersec: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
/// A server by NordVPN.
struct Server {
    /// The country this server is located in.
    flag: String,
    /// The domain of this server.
    domain: String,
    /// The current load on this server.
    pub load: u8,
    /// Categories this server is in.
    categories: Vec<Category>,
    /// Features of the server
    features: Features,
}

fn main() {
    let mut data: Vec<Server> = serde_json::from_str(
        &reqwest::get("https://api.nordvpn.com/server")
        .unwrap()
        .text()
        .unwrap()
        // TODO: find a better solution to these expensive replacements.
        .replace("Standard VPN servers", "Standard")
        .replace("Obfuscated Servers", "Obfuscated")
        .replace("Double VPN", "Double")
        .replace("Onion Over VPN", "Tor")
        .replace("Dedicated IP servers", "Dedicated"),
    ).unwrap();

    // Check whether filters were applied
    // Detect applied filters
    let mut country_filter: Option<String> = None;
    let mut standard_filter = false;
    let mut p2p_filter = false;
    let mut double_filter = false;
    let mut dedicated_filter = false;
    let mut tor_filter = false;
    let mut obfuscated_filter = false;
    let mut tcp_filter = false;
    let mut udp_filter = false;
    for filter in std::env::args().into_iter().skip(1) {
        match filter.as_ref() {
            "p2p" => p2p_filter = true,
            "standard" => standard_filter = true,
            "double" => double_filter = true,
            "dedicated" => dedicated_filter = true,
            "tor" => tor_filter = true,
            "obfuscated" => obfuscated_filter = true,
            "tcp" => tcp_filter = true,
            "udp" => udp_filter = true,
            _ => country_filter = Some(filter),
        };
    }

    // Filter servers that are not required.

    // Filtering countries
    if country_filter.is_some() {
        let country: String = country_filter.unwrap().to_uppercase();
        (&mut data).retain(|server| server.flag == country);
    };

    // Filtering Standard
    if standard_filter {
        (&mut data).retain(|server| {
            server.categories.contains(&Category {
                name: CategoryType::Standard,
            })
        });
    };

    // Filtering P2P
    if p2p_filter {
        (&mut data).retain(|server| {
            server.categories.contains(&Category {
                name: CategoryType::P2P,
            })
        });
    };

    // Filtering Tor/Onion
    if tor_filter {
        (&mut data).retain(|server| {
            server.categories.contains(&Category {
                name: CategoryType::Tor,
            })
        });
    };

    // Filtering Double
    if double_filter {
        (&mut data).retain(|server| {
            server.categories.contains(&Category {
                name: CategoryType::Double,
            })
        });
    };

    // Filtering Obfuscated servers
    if obfuscated_filter {
        (&mut data).retain(|server| {
            server.categories.contains(&Category {
                name: CategoryType::Obfuscated,
            })
        });
    };

    // Filtering Dedicated
    if dedicated_filter {
        (&mut data).retain(|server| {
            server.categories.contains(&Category {
                name: CategoryType::P2P,
            })
        });
    };

    if tcp_filter {
        (&mut data).retain(|server| server.features.openvpn_tcp);
    }

    if udp_filter {
        (&mut data).retain(|server| server.features.openvpn_udp);
    }

    // Sort the data on load
    data.sort_unstable_by(|x, y| x.load.cmp(&y.load));

    if data.len() != 0 {
        println!("{}", data[0].domain);
    } else {
        eprintln!("Could not find a server");
        std::process::exit(1);
    }
}
