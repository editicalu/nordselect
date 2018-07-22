extern crate nordselect;

use nordselect::{CategoryType, Protocol, Servers};

fn main() {
    let mut data = Servers::from_api().unwrap();

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
            // TODO: enhance this
            _ => country_filter = Some(filter),
        };
    }

    // Filter servers that are not required.

    // Filtering countries
    if country_filter.is_some() {
        let country: String = country_filter.unwrap().to_uppercase();
        data.filter_country(&country);
    };

    // Filtering Standard
    if standard_filter {
        data.filter_category(CategoryType::Standard);
    };

    // Filtering P2P
    if p2p_filter {
        data.filter_category(CategoryType::P2P);
    };

    // Filtering Tor/Onion
    if tor_filter {
        data.filter_category(CategoryType::Tor);
    };

    // Filtering Double
    if double_filter {
        data.filter_category(CategoryType::Double);
    };

    // Filtering Obfuscated servers
    if obfuscated_filter {
        data.filter_category(CategoryType::Obfuscated);
    };

    // Filtering Dedicated
    if dedicated_filter {
        data.filter_category(CategoryType::Dedicated);
    };

    // Filtering servers with TCP capacity
    if tcp_filter {
        data.filter_protocol(Protocol::Tcp);
    }

    // Filtering servers with UDP capacity
    if udp_filter {
        data.filter_protocol(Protocol::Udp);
    }

    // Sort the data on load
    data.sort_load();

    // Print the ideal server, if found.
    if let Some(server) = data.get_perfect_server() {
        println!("{}", server.domain);
    } else {
        eprintln!("No server found");
        std::process::exit(1);
    }
}
