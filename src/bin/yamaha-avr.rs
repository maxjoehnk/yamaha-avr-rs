#[macro_use]
extern crate clap;
extern crate yamaha_avr;

use clap::App;

fn main() {
    let matches = clap_app!(@app(App::new("Yamaha AVR Remote"))
        (version: "0.1.0")
        (author: "Max Jöhnk <maxjoehnk@gmail.com>")
        (@arg ip: --ip +takes_value "Set the AVR Ip")
        (@subcommand power =>
            (about: "Get/Set Power")
            (@arg value: "Get/Set the Power state")
        )
        (@subcommand mute =>
            (about: "Mute/Unmute")
            (@arg value: +required "Set the Mute State")
        )
        (@subcommand inputs =>
            (about: "Get available Inputs")
        )
        (@subcommand select =>
            (about: "Select Input")
            (@arg input: +required "The Input to select")
        )
    ).get_matches();
    let ip: String = matches.value_of("ip").unwrap_or("192.168.2.102").to_owned();
    let mut avr = yamaha_avr::connect(ip);
    if let Some(matches) = matches.subcommand_matches("power") {
        match matches.value_of("value") {
            Some(value) => {
                let parsed_value = parse_bool_state(value);
                if parsed_value.is_some() {
                    avr.set_power(parsed_value.unwrap());
                }else {
                    println!("Invalid value {}", value);
                }
            },
            None => {
                unimplemented!();
            }
        }
    }
    if let Some(matches) = matches.subcommand_matches("mute") {
        match matches.value_of("value") {
            Some(value) => {
                let parsed_value = parse_bool_state(value);
                if parsed_value.is_some() {
                    avr.set_mute(parsed_value.unwrap());
                }else {
                    println!("Invalid value {}", value);
                }
            },
            None => {
                unimplemented!();
            }
        }
    }
    if matches.subcommand_matches("inputs").is_some() {
        let inputs = avr.get_inputs().unwrap();
        for input in inputs {
            println!("{}", input.name);
        }
    }
    if let Some(matches) = matches.subcommand_matches("select") {
        let input = matches.value_of("input").unwrap().to_owned();
        avr.select_input(input, None).unwrap();
    }
}

fn parse_bool_state(input: &str) -> Option<bool> {
    if input == "on" {
        return Some(true);
    }else if input == "off" {
        return Some(false);
    }
    None
}