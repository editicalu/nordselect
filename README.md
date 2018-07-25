# NordSelect

A fast library/CLI to find the perfect NordVPN server to connect to, based on given filters.

# Installation

To install the `nordselect` CLI using [Cargo (Rust package manager)](https://www.rust-lang.org/en-US/install.html), enter `cargo install nordselect`. Because of the pinging functionality, you have to execute the following command:
    sudo setcap cap_net_raw+ep ~/.cargo/bin/nordselect    # allow binary to send ping packets

You might require the following packages:
    # Ubuntu
    apt install autoconf automake libtool gcc

Official binaries (PPA for Ubuntu and AUR package for Arch Linux) will be available when nordselect is stabilized.

# CLI Usage

    nordselect [FILTER ..]

Possible filters are:
- A country (in [ISO 3166-1 alpha-2](//en.wikipedia.org/wiki/ISO_3166-1_alpha-2) format)
- A protocol (`tcp`, `udp`)
- A servertype (`standard`, `p2p`, `tor`, `double`, `obfuscated`, `dedicated`)

## Examples

    # I don't care, just pick a server
    nordselect
    # A server in Latvia with P2P that supports p2p over tcp.
    nordselect lv tcp p2p
    # A server that supports both Tor and double VPN
    nordselect tor double

## Library Usage

The documentation of the library can be found at [docs.rs](https://docs.rs/crate/nordselect/).

# Selection method

To select a server without waiting too long, we use the following method:

- Take all possible servers with the given filters.
- Ping the 10 ones with the least load twice.
- Take the best one.

If you think you have a better selection procedure, please let me know by opening an issue.

# License

This project is licensed under the very permissive [MIT License](https://opensource.org/licenses/MIT).
