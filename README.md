# NordSelect

> **This package is passively maintained**. As I don't have a NordVPN account anymore, I'm no longer developing NordSelect. If someone finds a bug, open an issue and I'll do my best to fix it. If you feel like a feature is missing, feel free to open a PR. I'll be happy to have a look at it and merge.
> 
> \- Ward Segers, maintainer of NordSelect

[![Build Status](https://travis-ci.com/editicalu/nordselect.svg?branch=master)](https://travis-ci.com/editicalu/nordselect)
[![dependency status](https://deps.rs/repo/github/editicalu/nordselect/status.svg)](https://deps.rs/repo/github/editicalu/nordselect)

A fast CLI and Rust crate to find the perfect NordVPN server to connect to, based on given filters.

# Installation

## Arch Linux

Nordselect is available for Arch Linux as the [`nordselect` AUR package](https://aur.archlinux.org/packages/nordselect).

If you don't want to compile this program (which can take a while if you're compiling from scratch), you can use my [custom repository](https://ear.wardsegers.be) and install a precompiled binary. The repo only supports x86_64.

## Using Cargo

To install the `nordselect` CLI using [Cargo (Rust package manager)](https://www.rust-lang.org/en-US/install.html), enter `cargo install nordselect`.

## Pinging

Because of the pinging functionality, you have to execute the following command if you want to use the ping feature:

    # allow binary to send ping packets
    sudo setcap cap_net_raw+ep ~/.cargo/bin/nordselect

We might add extra installation options in the future.

# CLI Usage

    nordselect [FLAGS] [OPTIONS] [filter ..]

For a full list of options and flags, run `nordselect -h`.

## Filters

Possible filters are:
- A country (in [ISO 3166-1 alpha-2](//en.wikipedia.org/wiki/ISO_3166-1_alpha-2) format)
- A protocol (`tcp`, `udp`)
- A servertype (`standard`, `p2p`, `tor`, `double`, `obfuscated`, `dedicated`)

To see all filters, use `nordselect --filters`

## Examples

    # I don't care, just pick a server
    nordselect
    
    # A server in Latvia with P2P that supports p2p over tcp.
    nordselect lv tcp p2p

    # A server that supports both Tor and double VPN.
    # At the moment of writing, no such server is available.
    nordselect tor double

    # Use case: in combination with the official NordVPN CLI:
    # https://nordvpn.com/download/linux/
    nordvpn connect `nordselect ua`

# Library Usage

The documentation of the library can be found at [docs.rs](https://docs.rs/nordselect/).

# Development

The application is in development. If you encouter a bug, please open an issue describing how the bug occured or open a PR.

New features are not planned for now, but feel free to open issues to discuss them.

**Warning**: before running `cargo test`, use `dummydata.sh` to generate some dummy data.

# Selection method

To select a server without waiting too long, we use the following method to find your preferred server.

1. Download the list with all servers using the NordVPN API.
2. Apply your filters on the received data.
3. Sort the data on load.
4. Pick the best one.

When using `-p` or `-s`, it will take the 10 servers with the least load and compare their ping values.
If you think you have a better selection procedure, please let me know by opening an issue.

# License

This project is licensed under the very permissive [MIT License](https://opensource.org/licenses/MIT).
