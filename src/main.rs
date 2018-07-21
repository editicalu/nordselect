extern crate nordselect;

use nordselect::{Category, CategoryType, Server};

fn main() {
    let mut data = Server::from_api().unwrap();

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
