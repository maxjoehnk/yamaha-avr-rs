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
            (@arg value: "Set the Mute State")
        )
        (@subcommand inputs =>
            (about: "Get available Inputs")
        )
        (@subcommand select =>
            (about: "Select Input")
            (@arg input: +required "The Input to select")
        )
        (@subcommand volume =>
            (@setting AllowNegativeNumbers)
            (about: "Get/set the volume")
            (@arg value: "The Volume to set to")
        )
    ).get_matches();
    let ip: String = matches.value_of("ip").unwrap_or("192.168.2.102").to_owned();
    let mut avr = yamaha_avr::connect(ip);
    if let Some(matches) = matches.subcommand_matches("power") {
        match matches.value_of("value") {
            Some(value) => {
                let parsed_value = parse_bool_state(value);
                if parsed_value.is_some() {
                    avr.set_power(parsed_value.unwrap()).unwrap();
                }else {
                    println!("Invalid value {}", value);
                }
            },
            None => {
                let power = avr.get_power().unwrap();
                println!("Power: {}", transform_bool_state(power));
            }
        }
    }
    if let Some(matches) = matches.subcommand_matches("mute") {
        match matches.value_of("value") {
            Some(value) => {
                let parsed_value = parse_bool_state(value);
                if parsed_value.is_some() {
                    avr.set_mute(parsed_value.unwrap()).unwrap();
                }else {
                    println!("Invalid value {}", value);
                }
            },
            None => {
                let muted = avr.get_mute().unwrap();
                println!("Mute: {}", transform_bool_state(muted));
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
    if let Some(matches) = matches.subcommand_matches("volume") {
        match matches.value_of("value") {
            Some(value) => {
                let parsed_value: Result<i32, std::num::ParseIntError> = value.parse();
                if parsed_value.is_ok() {
                    avr.set_volume(parsed_value.unwrap()).unwrap();
                }else {
                    println!("Invalid value {}", value);
                }
            },
            None => {
                let volume = avr.get_volume().unwrap();
                println!("Volume: {:?}", volume);
            }
        }
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

fn transform_bool_state(input: bool) -> &'static str {
    if input { "On" } else { "Off" }
}