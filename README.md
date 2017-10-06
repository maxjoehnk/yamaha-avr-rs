# yamaha-avr-rs

`yamaha-avr-rs` provides a Library and a CLI for interaction with Yamaha AVRs.

## Library
### Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
yamaha_avr = "0.2"
```

Next add this to your crate root:
```rust
extern crate yamaha_avr;
```

Now you are able to connect to your avr via
```rust
yamaha::connect("ip".to_owned()); // connect takes a String instead of a str
```

Soon you will also be able to discover your local AVRs via
```rust
yamaha::discover();
```
which will return a List of found Receivers.

## CLI

### Installation

#### Using cargo
Requires Rust

```
cargo install yamaha_avr
```

### Usage
```
USAGE:
    yamaha-avr [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --ip <ip>    Set the AVR Ip

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    inputs    Get available Inputs
    mute      Mute/Unmute
    power     Set Power
    select    Select Input
```