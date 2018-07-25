extern crate clap;
extern crate nordselect;

use nordselect::{CategoryType, Protocol, Servers};

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
            Arg::with_name("filter")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .get_matches();

    let mut data = match Servers::from_api() {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Could not download data: {}", x);
            std::process::exit(1);
        }
    };

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
    for filter in matches.values_of("filter").unwrap() {
        match filter {
            "p2p" => p2p_filter = true,
            "standard" => standard_filter = true,
            "double" => double_filter = true,
            "dedicated" => dedicated_filter = true,
            "tor" => tor_filter = true,
            "obfuscated" => obfuscated_filter = true,
            "tcp" => tcp_filter = true,
            "udp" => udp_filter = true,
            // TODO: enhance this
            _ => country_filter = Some(String::from(filter)),
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
        }
    }

    // Print the ideal server, if found.
    if let Some(server) = data.get_perfect_server() {
        println!("{}", server.domain);
    } else {
        eprintln!("No server found");
        std::process::exit(1);
    }
}
