use super::prelude::*;
use crate::servers::Protocol;

/// Filter that keeps only servers that accept a specific protocol.
///
/// # Example
///
/// ```
/// use nordselect::Servers;
/// use nordselect::Protocol;
/// use nordselect::filters::ProtocolFilter;
/// let mut data = Servers::dummy_data();
///
/// // Filter on the TCP protocol
/// data.filter(&ProtocolFilter::from(Protocol::Tcp));
///
/// assert!(data.perfect_server().is_some());
/// ```
pub struct ProtocolFilter {
    /// The protocol that should be filtered against.
    protocol: Protocol,
}

impl From<Protocol> for ProtocolFilter {
    fn from(protocol: Protocol) -> ProtocolFilter {
        ProtocolFilter { protocol }
    }
}

impl Filter for ProtocolFilter {
    fn filter(&self, server: &Server) -> bool {
        match self.protocol {
            Protocol::Tcp => server.features.openvpn_tcp,
            Protocol::Udp => server.features.openvpn_udp,
            Protocol::Pptp => server.features.pptp,
            Protocol::L2tp => server.features.l2tp,
            Protocol::OpenVPNXTcp => server.features.openvpn_tcp,
            Protocol::OpenVPNXUdp => server.features.openvpn_udp,
            Protocol::Socks => server.features.socks,
            Protocol::CyberSecProxy => server.features.proxy_cybersec,
            Protocol::SslProxy => server.features.proxy_ssl,
            Protocol::CyberSecSslProxy => server.features.proxy_ssl_cybersec,
            Protocol::Proxy => server.features.proxy,
            Protocol::WireGuardUdp => server.features.wireguard_udp,
        }
    }
}
