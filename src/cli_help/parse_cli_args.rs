pub fn parse_cli_args<'a>() -> clap::ArgMatches<'a> {
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
                .help("Any restriction put on the server. \
                    This can be a country ('us'), a protocol ('tcp') or a type \
                    of server ('p2p'). \
                    Any filter can be inverted by prepending '!' to it ('!us'). \
                    See --filters"),
        )
        .get_matches()
}
