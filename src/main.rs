extern crate clap;
extern crate nordselect;

use nordselect::filters::{self, Filter};
use nordselect::{Protocol, ServerCategory, Servers};
use std::collections::HashSet;

fn parse_cli_args<'a>() -> clap::ArgMatches<'a> {
    use clap::{App, Arg};
    App::new("NordSelect")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("multi_ping")
                .short("p")
                .long("ping")
                .help("Use ping tests with simultaneous pings")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("single_ping")
                .short("s")
                .long("sping")
                .help("Use ping tests and execute pings linear")
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
                .help("Any restriction put on the server. This can be a country ('us'), a protocol ('tcp') or a type of server ('p2p'). See --filters"),
        )
        .get_matches()
}

fn show_available_filters(data: &Servers) {
    // Show protocols
    println!("PROTOCOLS:\ttcp, udp, pptp, l2tp, tcp_xor, udp_xor, socks, cybersecproxy, sslproxy, cybersecsslproxy, proxy");
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
    println!();

    // Show regions
    println!("REGIONS:");
    let iter = nordselect::filters::Region::from_str_options();
    let mut iter = iter.into_iter();
    if let Some(flag) = iter.next() {
        println!("{}\t{}", flag.0.to_lowercase(), flag.1);
        iter.for_each(|flag| println!("{}\t{}", flag.0.to_lowercase(), flag.1));
        println!();
    }
}

fn parse_static_filter(filter: &str) -> Option<(Box<dyn Filter>, bool)> {
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
            _ => return None,
        }
    };
    Some((lib_filter, is_category_filter))
}

fn consider_negating_filter<'a>(filter: &'a str) -> (&'a str, bool) {
    if filter.len() > 0 && &filter[..1] == "!" {
        return (&filter[1..], true);
    }
    (filter.into(), false)
}

#[test]
fn consider_negating_filter_test() {
    assert_eq!(consider_negating_filter("qwe"), ("qwe", false));
    assert_eq!(consider_negating_filter("!qwe"), ("qwe", true));
    assert_eq!(consider_negating_filter(""), ("", false));
}

fn parse_filters(cli_filters: clap::Values, data: &Servers) -> Vec<Box<dyn Filter>> {
    // Parse which countries are in the data
    let flags = data.flags();

    let mut lib_filters: Vec<Box<dyn Filter>> = Vec::new();
    let mut category_filter_added = false;
    let mut included_countries = HashSet::new();
    let mut excluded_countries = HashSet::new();

    for original_filter in cli_filters.into_iter() {
        let (filter, is_negating) = consider_negating_filter(original_filter);

        if let Some((lib_filter, is_category_filter)) = parse_static_filter(filter) {
            lib_filters.push(if is_negating {
                Box::new(filters::NegatingFilter::from(lib_filter))
            } else {
                lib_filter
            });
            if is_category_filter {
                category_filter_added = true;
            }
            continue;
        }

        let filter_upper = filter.to_uppercase();
        let contries_to_modify = if is_negating {
            &mut excluded_countries
        } else {
            &mut included_countries
        };

        if flags.contains(filter_upper.as_str()) {
            contries_to_modify.insert(filter_upper);
            continue;
        }

        if let Some(region_countries) = filters::Region::from_str(&filter_upper) {
            region_countries.countries().into_iter().for_each(|flag| {
                contries_to_modify.insert(flag.into());
                ()
            });
            continue;
        }

        if let Ok(binary) = std::env::current_exe()
            .unwrap()
            .into_os_string()
            .into_string()
        {
            eprintln!(
                "Error: unknown filter: \"{}\". Run `{} --filters` to list all available filters.",
                original_filter, binary
            );
        } else {
            eprintln!(
                "Error: unknown filter: \"{}\". Use `--filters` to list all available filters.",
                original_filter
            );
        }
        std::process::exit(1);
    }

    // Use a Standard server if no special server is requested.
    if !category_filter_added {
        lib_filters.push(Box::new(filters::CategoryFilter::from(
            ServerCategory::Standard,
        )));
    }

    // Add countries filters.
    if !included_countries.is_empty() {
        lib_filters.push(Box::new(filters::CountriesFilter::from(included_countries)));
    }
    if !excluded_countries.is_empty() {
        lib_filters.push(Box::new(filters::NegatingFilter::new(
            filters::CountriesFilter::from(excluded_countries),
        )));
    }

    lib_filters
}

fn apply_filters(filters_to_apply: Vec<Box<dyn Filter>>, data: &mut Servers) {
    for filter in filters_to_apply.iter() {
        data.filter(filter.as_ref())
    }
}

fn sort(data: &mut Servers, matches: &clap::ArgMatches) {
    let mut should_sort = true;

    // Perform ping test if required
    let s_ping = matches.is_present("single_ping");
    let m_ping = matches.is_present("multi_ping");
    if s_ping || m_ping {
        let tries_opt = matches.value_of("tries").unwrap().parse();
        if let Err(err) = tries_opt {
            eprintln!("Could not read tries of pings: {}", err);

            std::process::exit(1);
        }

        let amount_opt = matches.value_of("amount").unwrap().parse();
        if let Err(err) = amount_opt {
            eprintln!("Could not read amount of pings: {}", err);

            std::process::exit(1);
        }

        let amount = amount_opt.unwrap();
        let tries = tries_opt.unwrap();

        data.cut(amount);

        match {
            if s_ping {
                nordselect::sorters::PingSorter::ping_single(&data, tries)
            } else {
                nordselect::sorters::PingSorter::ping_multi(&data, tries)
            }
        } {
            Ok(sorter) => {
                data.sort(&sorter);
                should_sort = false;
            }
            Err(error) => {
                eprintln!("An error occured when pinging: {}", error);
                eprintln!("Results will not include ping results");

                match error.to_string().as_str() {
                    "oping::PingError::LibOpingError: Operation not permitted" => {
                        eprintln!("");
                        eprintln!(
                            "This error means that you did not give permission to nordselect to ping."
                        );
                        eprintln!(
                            "More details can be found at https://github.com/cfallin/rust-oping"
                        );
                        if let Ok(exe) = std::env::current_exe() {
                            if cfg!(unix) {
                                eprintln!("Hint: to solve this on Linux, execute the following command (as root):");
                                eprintln!("\tsetcap cap_net_raw+ep {:#?}", exe);
                            } else if cfg!(windows) {
                                eprintln!("Hint: ping has not been tested on Windows. Consider using something else.");
                            }
                        }
                    }
                    _ => {}
                }

                eprintln!("");

                should_sort = true;
            }
        }
    }

    if should_sort {
        data.sort(&nordselect::sorters::LoadSorter);
    }
}

fn main() {
    // Parse CLI args
    let matches = parse_cli_args();

    // Get API data
    let mut data = match Servers::from_api() {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Could not download data: {}", x);
            std::process::exit(1);
        }
    };

    // Should we only show the available filters?
    if matches.is_present("list_filters") {
        show_available_filters(&data);
        std::process::exit(0);
    }

    // Detect filters
    let filters_to_apply = parse_filters(
        matches
            .values_of("filter")
            .unwrap_or(clap::Values::default()),
        &data,
    );

    // Filter servers that are not required.
    apply_filters(filters_to_apply, &mut data);

    // Sort the servers
    sort(&mut data, &matches);

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
