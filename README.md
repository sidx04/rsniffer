# rsniffer

A simple CLI tool written in Rust that checks network ports for one of three possible statuses – open, closed, or filtered; and "sniffs" out the open ports only.

## Usage

The script provides three CLI arguments that are to be specified during execution. To view the arguments, run `cargo run -- -help`, which prompts the following:

```bash
Usage: rsniffer [-a=ARG] [-s=ARG] [-e=ARG]

Available options:
    -a, --address=ARG  valid IPv4 address that is to be sniffed, falls back to `IPFALLBACK`
    -s, --start=ARG    start port for the sniffer
    -e, --end=ARG      end port for the sniffer
    -h, --help         Prints help information
```

For example,

```bash
cargo run 8.8.8.8 -s 50 -e 100
```

or,

```bash
cargo run 192.168.1.1
```

A vector of open IP ports will be returned.

## Running Tests

A pre-loaded test module has been added to the rust script. To execute the tests, simply run:

```bash
  cargo test
```

## Acknowledgements

This project is largely inspired by v2 of [Tensor Programming's Port Sniffer](https://youtu.be/RhFZxkxkeIc?si=kfRy9cStwcYr-bBI).
