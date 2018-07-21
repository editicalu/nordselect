extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Category {
    pub name: CategoryType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Features {
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
pub struct Server {
    /// The country this server is located in.
    pub flag: String,
    /// The domain of this server.
    pub domain: String,
    /// The current load on this server.
    pub load: u8,
    /// Categories this server is in.
    pub categories: Vec<Category>,
    /// Features of the server
    pub features: Features,
}

impl Server {
    pub fn from_api() -> Result<Vec<Server>, Box<std::error::Error>> {
        let mut data = reqwest::get("https://api.nordvpn.com/server")?;
        let text = data.text()?;

        Ok(serde_json::from_str(
            // TODO: find a better solution to these expensive replacements.
            &text.replace("Standard VPN servers", "Standard")
                .replace("Obfuscated Servers", "Obfuscated")
                .replace("Double VPN", "Double")
                .replace("Onion Over VPN", "Tor")
                .replace("Dedicated IP servers", "Dedicated"),
        )?)
    }
}
