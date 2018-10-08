extern crate clap;
extern crate nordselect;

use nordselect::filters;
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

fn parse_filters(cli_filters: clap::Values, data: &Servers) -> PossibleFilters {
    // Parse which countries are in the data
    let flags = data.flags();

    let mut parsed_filters = PossibleFilters::new();

    for filter in cli_filters.into_iter() {
        match filter {
            "p2p" => parsed_filters.p2p_filter = true,
            "standard" => parsed_filters.standard_filter = true,
            "double" => parsed_filters.double_filter = true,
            "dedicated" => parsed_filters.dedicated_filter = true,
            "tor" => parsed_filters.tor_filter = true,
            "obfuscated" => parsed_filters.obfuscated_filter = true,
            "tcp" => parsed_filters.tcp_filter = true,
            "udp" => parsed_filters.udp_filter = true,
            "pptp" => parsed_filters.pptp_filter = true,
            "l2tp" => parsed_filters.l2tp = true,
            "tcp_xor" => parsed_filters.xor_tcp = true,
            "udp_xor" => parsed_filters.xor_udp = true,
            "socks" => parsed_filters.socks = true,
            "cybersecproxy" => parsed_filters.cy_pr = true,
            "sslproxy" => parsed_filters.ssl_pr = true,
            "cybersecsslproxy" => parsed_filters.cyssl_pr = true,
            "proxy" => parsed_filters.pr = true,
            _ => {
                let upper = filter.to_uppercase();
                if flags.contains(&upper.as_ref()) {
                    if parsed_filters.country_filter.is_none() {
                        parsed_filters.country_filter = Some(HashSet::with_capacity(1));
                    }
                    parsed_filters
                        .country_filter
                        .as_mut()
                        .unwrap()
                        .insert(upper);
                } else if let Some(region_countries) =
                    nordselect::filters::Region::from_str(&filter.to_uppercase())
                {
                    if parsed_filters.country_filter.is_none() {
                        parsed_filters.country_filter = Some(HashSet::new());
                    }
                    region_countries.countries().into_iter().for_each(|flag| {
                        parsed_filters
                            .country_filter
                            .as_mut()
                            .unwrap()
                            .insert(String::from(flag));
                        ()
                    });
                } else {
                    if let Ok(binary) = std::env::current_exe()
                        .unwrap()
                        .into_os_string()
                        .into_string()
                    {
                        eprintln!("Error: unknown filter: \"{}\". Run `{} --filters` to list all available filters.", filter, binary);
                    } else {
                        eprintln!("Error: unknown filter: \"{}\". Use `--filters` to list all available filters.", filter);
                    }
                    std::process::exit(1);
                }
            }
        };
    }
    parsed_filters
}

fn apply_filters(filters_to_apply: &PossibleFilters, data: &mut Servers) {
    // Filtering countries
    if let Some(ref countries) = filters_to_apply.country_filter {
        data.filter(&filters::CountriesFilter::from(countries.clone()));
    };

    // Filtering Standard
    if filters_to_apply.standard_filter {
        data.filter(&filters::CategoryFilter::from(ServerCategory::Standard));
    };

    // Filtering P2P
    if filters_to_apply.p2p_filter {
        data.filter(&filters::CategoryFilter::from(ServerCategory::P2P));
    };

    // Filtering Tor/Onion
    if filters_to_apply.tor_filter {
        data.filter(&filters::CategoryFilter::from(ServerCategory::Tor));
    };

    // Filtering Double
    if filters_to_apply.double_filter {
        data.filter(&filters::CategoryFilter::from(ServerCategory::Double));
    };

    // Filtering Obfuscated servers
    if filters_to_apply.obfuscated_filter {
        data.filter(&filters::CategoryFilter::from(ServerCategory::Obfuscated));
    };

    // Filtering Dedicated
    if filters_to_apply.dedicated_filter {
        data.filter(&filters::CategoryFilter::from(ServerCategory::Dedicated));
    };

    // Filtering servers on protocol
    if filters_to_apply.tcp_filter {
        data.filter(&filters::ProtocolFilter::from(Protocol::Tcp));
    }
    if filters_to_apply.udp_filter {
        data.filter(&filters::ProtocolFilter::from(Protocol::Udp));
    }
    if filters_to_apply.pptp_filter {
        data.filter(&filters::ProtocolFilter::from(Protocol::Pptp));
    }
    if filters_to_apply.l2tp {
        data.filter(&filters::ProtocolFilter::from(Protocol::L2tp));
    }
    if filters_to_apply.xor_tcp {
        data.filter(&filters::ProtocolFilter::from(Protocol::OpenVPNXTcp));
    }
    if filters_to_apply.xor_udp {
        data.filter(&filters::ProtocolFilter::from(Protocol::OpenVPNXUdp));
    }
    if filters_to_apply.socks {
        data.filter(&filters::ProtocolFilter::from(Protocol::Socks));
    }
    if filters_to_apply.cy_pr {
        data.filter(&filters::ProtocolFilter::from(Protocol::CyberSecProxy));
    }
    if filters_to_apply.ssl_pr {
        data.filter(&filters::ProtocolFilter::from(Protocol::SslProxy));
    }
    if filters_to_apply.cyssl_pr {
        data.filter(&filters::ProtocolFilter::from(Protocol::CyberSecSslProxy));
    }
    if filters_to_apply.pr {
        data.filter(&filters::ProtocolFilter::from(Protocol::Proxy));
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

struct PossibleFilters {
    country_filter: Option<HashSet<String>>,
    standard_filter: bool,
    p2p_filter: bool,
    double_filter: bool,
    dedicated_filter: bool,
    tor_filter: bool,
    obfuscated_filter: bool,
    tcp_filter: bool,
    udp_filter: bool,
    pptp_filter: bool,
    l2tp: bool,
    xor_tcp: bool,
    xor_udp: bool,
    socks: bool,
    cy_pr: bool,
    ssl_pr: bool,
    cyssl_pr: bool,
    pr: bool,
}

impl PossibleFilters {
    fn new() -> PossibleFilters {
        PossibleFilters {
            country_filter: None,
            standard_filter: false,
            p2p_filter: false,
            double_filter: false,
            dedicated_filter: false,
            tor_filter: false,
            obfuscated_filter: false,
            tcp_filter: false,
            udp_filter: false,
            pptp_filter: false,
            l2tp: false,
            xor_tcp: false,
            xor_udp: false,
            socks: false,
            cy_pr: false,
            ssl_pr: false,
            cyssl_pr: false,
            pr: false,
        }
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
    apply_filters(&filters_to_apply, &mut data);

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
