extern crate clap;
extern crate nordselect;

use nordselect::{CategoryType, Protocol, Servers};
use std::collections::HashSet;

fn main() {
    // Parse CLI args
    use clap::{App, Arg};
    let matches = App::new("NordSelect")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("ping")
                .short("p")
                .long("ping")
                .help("Use ping to find the best server")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("tries")
                .short("t")
                .long("tries")
                .value_name("TRIES")
                .default_value("2")
                .help("Ping every server TRIES times")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("amount")
                .short("a")
                .long("amount")
                .value_name("AMOUNT")
                .default_value("10")
                .help("Ping only to the least AMOUNT ones loaded")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .help("Print the full domain instead of the short identifier (us1.nordvpn.com instead of us1)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("list_filters")
                .long("filters")
                .help("Show all available filters")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("filter")
                .required(false)
                .multiple(true)
                .index(1)
                .help("Any restriction put on the server. This can be a country ('us'), a protocol ('tcp;) or a type of server ('p2p'). See --filters"),
        )
        .get_matches();

    let mut data = match Servers::from_api() {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Could not download data: {}", x);
            std::process::exit(1);
        }
    };

    if matches.is_present("list_filters") {
        // Show protocols
        println!("PROTOCOLS:\ttcp, udp");
        // Show server types
        println!("SERVERS:\tstandard, dedicated, double, obfuscated, p2p, tor");

        // Show countries
        let mut flags: Vec<String> = data.flags().iter().map(|&x| x.to_lowercase()).collect();
        flags.sort_unstable();
        let flags = flags;

        let mut iter = flags.iter();
        if let Some(flag) = iter.next() {
            print!("COUNTRIES:\t{}", flag.to_lowercase());
            iter.for_each(|flag| print!(", {}", flag.to_lowercase()));
        }
        println!();

        // Show regions
        println!("REGIONS:\teu");
        std::process::exit(0);
    }

    // Check whether filters were applied
    // Detect applied filters
    let mut country_filter: Option<HashSet<String>> = None;
    let mut standard_filter = false;
    let mut p2p_filter = false;
    let mut double_filter = false;
    let mut dedicated_filter = false;
    let mut tor_filter = false;
    let mut obfuscated_filter = false;
    let mut tcp_filter = false;
    let mut udp_filter = false;
    {
        // Parse which countries are in the data
        let flags = data.flags();

        for filter in matches
            .values_of("filter")
            .unwrap_or(clap::Values::default())
        {
            match filter {
                "p2p" => p2p_filter = true,
                "standard" => standard_filter = true,
                "double" => double_filter = true,
                "dedicated" => dedicated_filter = true,
                "tor" => tor_filter = true,
                "obfuscated" => obfuscated_filter = true,
                "tcp" => tcp_filter = true,
                "udp" => udp_filter = true,
                "eu" => {
                    if country_filter.is_none() {
                        country_filter = Some(HashSet::with_capacity(nordselect::EU.len()));
                    }
                    for &country in nordselect::EU.iter() {
                        country_filter
                            .as_mut()
                            .unwrap()
                            .insert(String::from(country));
                    }
                }
                _ => {
                    let upper = filter.to_uppercase();
                    if flags.contains(upper.as_ref() as &str) {
                        if country_filter.is_none() {
                            country_filter = Some(HashSet::with_capacity(1));
                        }
                        country_filter.as_mut().unwrap().insert(upper);
                    } else {
                        eprintln!("Error: unknown filter: \"{}\"", filter);
                        std::process::exit(1);
                    }
                }
            };
        }
    }

    // Filter servers that are not required.

    // Filtering countries
    if country_filter.is_some() {
        data.filter_countries(&country_filter.unwrap());
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

    // Perform ping test if required
    if matches.is_present("ping") {
        // TODO: avoid crash when no integer
        let tries: usize = matches
            .value_of("tries")
            .unwrap()
            .parse()
            .expect("No valid integer");

        // TODO: avoid crash when no integer
        let amount: usize = matches
            .value_of("amount")
            .unwrap()
            .parse()
            .expect("No valid integer");

        if let Err(x) = data.benchmark_ping(amount, tries, false) {
            eprintln!("An error occured when pinging: {}", x);
            eprintln!("Results will not include ping results");

            match x.to_string().as_str() {
                "oping::PingError::LibOpingError: Operation not permitted" => {
                    eprintln!("");
                    eprintln!(
                        "This error means that you did not give permission to nordselect to ping."
                    );
                    eprintln!("More details can be found at https://github.com/cfallin/rust-oping");
                    eprintln!("Hint: to solve this, execute the following command (as root):");
                    eprintln!(
                        "\tsetcap cap_net_raw+ep {}",
                        std::env::args().next().unwrap()
                    );
                }
                _ => {}
            }

            eprintln!("");
        }
    }

    // Print the ideal server, if found.
    if let Some(server) = data.perfect_server() {
        println!(
            "{}",
            match matches.is_present("domain") {
                true => &server.domain,
                false => server.name().unwrap_or(&server.domain),
            }
        );
    } else {
        eprintln!("No server found");
        std::process::exit(1);
    }
}
