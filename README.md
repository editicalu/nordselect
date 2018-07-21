# NordSelect

A fast CLI to find the perfect NordVPN server to connect to, based on given data.

# Usage

    nordselect <options>

Possible options are:
- A country (in [ISO 3166-1 alpha-2](//en.wikipedia.org/wiki/ISO_3166-1_alpha-2) format)
- A protocol (`tcp`, `udp`)
- A servertype (`standard`, `p2p`, `tor`, `double`, `obfuscated`, `dedicated`)

The default is any server in the world.

## Examples

    # I don't care, just pick a server
    nordselect
    # A server in Latvia with P2P that supports tcp.
    nordselect lv tcp p2p
    # A server that supports both Tor and double VPN

# Selection method

To select a server without waiting to long, we use the following method:

- Take all possible servers with the given filters.
- Take the one with the least load.

If you think you have a better selection procedure, please let me know by opening an issue. I'm thinking about adding ping tests to the ones with the least load.
