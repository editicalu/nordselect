# NordSelect

A fast library/CLI to find the perfect NordVPN server to connect to, based on given filters.

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

The documentation of the library can be found at [docs.rs](https://docs.rs/crate/nordselect/0.1.0).

# Selection method

To select a server without waiting too long, we use the following method:

- Take all possible servers with the given filters.
- Ping the 10 ones with the least load twice.
- Take the best one.

If you think you have a better selection procedure, please let me know by opening an issue.

# License

This project is licensed under the very permissive [MIT License](https://opensource.org/licenses/MIT).
