#[macro_use]
extern crate clap;
extern crate yamaha_avr;

use clap::App;

fn main() {
    let mut matches = clap_app!(@app(App::new("Yamaha AVR Remote"))
        (version: "0.0.1")
        (author: "Max Jöhnk <maxjoehnk@gmail.com>")
        (@arg ip: --ip +takes_value "Set the AVR Ip")
        (@subcommand power =>
            (about: "Set Power")
            (@arg value: +required "Set the Power state")
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
        let value: bool = matches.value_of("value").unwrap().parse().unwrap();
        avr.power(value).unwrap();
    }
    if let Some(matches) = matches.subcommand_matches("mute") {
        let value: bool = matches.value_of("value").unwrap().parse().unwrap();
        avr.mute(value).unwrap();
    }
    if let Some(matches) = matches.subcommand_matches("inputs") {
        let inputs = avr.get_inputs().unwrap();
        for input in inputs {
            println!("{}", input.name);
        }
    }
    if let Some(matches) = matches.subcommand_matches("select") {
        let input = matches.value_of("input").unwrap().to_owned();
        avr.select_input(input).unwrap();
    }
}
