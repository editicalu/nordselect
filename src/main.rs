extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct Category {
    name: CategoryType,
}

#[derive(Debug, Deserialize)]
/// A server by NordVPN.
struct Server {
    /// The country this server is located in.
    flag: String,
    /// The domain of this server.
    domain: String,
    /// The current load on this server.
    pub load: u8,
    // Categories this server is in.
    categories: Vec<Category>,
}

fn main() {
    let data: Vec<Server> = serde_json::from_str(&reqwest::get("https://api.nordvpn.com/server")
        .unwrap()
        .text()
        .unwrap()
        // TODO: find a better solution to these expensive replacements.
        .replace("Standard VPN servers", "Standard")
        .replace("Obfuscated Servers", "Obfuscated")
        .replace("Double VPN", "Double")
        .replace("Onion Over VPN", "Tor")
        .replace("Dedicated IP servers", "Dedicated"))
        .unwrap();

    let mut country_filter: Option<String> = None;
    let mut standard_filter = false;
    let mut p2p_filter = false;
    let mut double_filter = false;
    let mut dedicated_filter = false;
    let mut tor_filter = false;
    let mut obfuscated_filter = false;

    for filter in std::env::args() {
        match filter.as_ref() {
            "p2p" => p2p_filter = true,
            "standard" => standard_filter = true,
            "double" => double_filter = true,
            "dedicated" => dedicated_filter = true,
            "tor" => tor_filter = true,
            "obfuscated" => obfuscated_filter = true,
            _ => country_filter = Some(filter),
        };
    }
    // Filter servers that are not required.

    // Filtering countries
    let data: Vec<Server> = if country_filter.is_some() {
        // TODO: filter countries
        data
    } else {
        data
    };

    let data: Vec<Server> = if standard_filter {
        // TODO: filter standard
        data
    } else {
        data
    };

    let data: Vec<Server> = if p2p_filter {
        // TODO: filter P2P
        data
    } else {
        data
    };

    let data: Vec<Server> = if tor_filter {
        // TODO: filter Tor
        data
    } else {
        data
    };

    let data: Vec<Server> = if double_filter {
        // TODO: filter double
        data
    } else {
        data
    };

    let data: Vec<Server> = if obfuscated_filter {
        // TODO: filter obfuscated
        data
    } else {
        data
    };

    let mut data: Vec<Server> = if dedicated_filter {
        // TODO: filter dedicated
        data
    } else {
        data
    };

    // Sort the data on load
    data.sort_unstable_by(|ref x, ref y| x.load.cmp(&y.load));

    println!("{:?}", data);
}
