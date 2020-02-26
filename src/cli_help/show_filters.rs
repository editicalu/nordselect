use nordselect::Servers;

pub fn show_available_filters(data: &Servers) {
    // Show protocols
    println!("PROTOCOLS:\ttcp, udp, pptp, l2tp, tcp_xor, udp_xor, socks, cybersecproxy, sslproxy, cybersecsslproxy, proxy, wg_udp");
    // Show server types
    println!("SERVERS:\tstandard, dedicated, double, obfuscated, p2p, tor");

    // Show countries
    let mut flags: Vec<&str> = data.flags().into_iter().collect();
    flags.sort_unstable();
    println!("COUNTRIES:\t{}", flags.join(", ").to_lowercase());

    println!();
    println!();

    // Show regions
    println!("REGIONS:");
    for flag in nordselect::filters::Region::from_str_options().iter() {
        println!("{}\t{}", flag.0.to_lowercase(), flag.1);
    }
    println!();
    println!("Any filter can be inverted using !");
}
