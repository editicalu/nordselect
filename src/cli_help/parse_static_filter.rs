use nordselect::filters::{self, Filter};
use nordselect::servers::Protocol;
use nordselect::ServerCategory;

pub fn parse_static_filter(filter: &str) -> Option<(Box<dyn Filter>, bool)> {
    let mut is_category_filter = false;
    let lib_filter = {
        let mut category_filter = |category: ServerCategory| -> Box<dyn Filter> {
            is_category_filter = true;
            Box::new(filters::CategoryFilter::from(category))
        };
        let protocol_filter = |protocol: Protocol| -> Box<dyn Filter> {
            Box::new(filters::ProtocolFilter::from(protocol))
        };

        match filter {
            "p2p" => category_filter(ServerCategory::P2P),
            "standard" => category_filter(ServerCategory::Standard),
            "double" => category_filter(ServerCategory::Double),
            "dedicated" => category_filter(ServerCategory::Dedicated),
            "tor" => category_filter(ServerCategory::Tor),
            "obfuscated" => category_filter(ServerCategory::Obfuscated),
            "tcp" => protocol_filter(Protocol::Tcp),
            "udp" => protocol_filter(Protocol::Udp),
            "pptp" => protocol_filter(Protocol::Pptp),
            "l2tp" => protocol_filter(Protocol::L2tp),
            "tcp_xor" => protocol_filter(Protocol::OpenVPNXTcp),
            "udp_xor" => protocol_filter(Protocol::OpenVPNXUdp),
            "socks" => protocol_filter(Protocol::Socks),
            "cybersecproxy" => protocol_filter(Protocol::CyberSecProxy),
            "sslproxy" => protocol_filter(Protocol::SslProxy),
            "cybersecsslproxy" => protocol_filter(Protocol::CyberSecSslProxy),
            "proxy" => protocol_filter(Protocol::Proxy),
            "wg_udp" => protocol_filter(Protocol::WireGuardUdp),
            _ => return None,
        }
    };
    Some((lib_filter, is_category_filter))
}
