extern crate xml;
extern crate regex;

use std::io::Result;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SystemConfigAvailableFeatures {
    pub tuner: bool,
    pub hd_radio: bool,
    pub rhapsody: bool,
    pub sirius_ir: bool,
    pub pandora: bool,
    pub server: bool,
    pub net_radio: bool,
    pub usb: bool,
    pub ipod: bool,
    pub air_play: bool
}

#[derive(PartialEq, Debug, Clone)]
pub struct SystemConfig {
    pub model_name: Option<String>,
    pub inputs: Vec<Input>,
    pub available_zones: Vec<String>,
    pub available_features: SystemConfigAvailableFeatures
}

#[derive(PartialEq, Debug, Clone)]
pub struct Input {
    /// The Internal name, used for input selection
    pub name: String,
    /// A User set Display Name, None when the name is empty
    pub display_name: Option<String>
}

pub fn parse_system_config(xml: String) -> Result<SystemConfig> {
    let reader = xml::reader::EventReader::from_str(&xml);
    let mut inputs: Vec<Input> = Vec::new();
    let mut in_inputs = false;
    let mut current_input: Option<Input> = None;
    let mut in_model_name = false;
    let mut model_name: Option<String> = None;
    let mut in_feature_existence = false;
    let mut current_feature: Option<String> = None;
    let mut available_zones: Vec<String> = Vec::new();
    let mut available_features = SystemConfigAvailableFeatures {
        tuner: false,
        hd_radio: false,
        rhapsody: false,
        sirius_ir: false,
        pandora: false,
        server: false,
        net_radio: false,
        usb: false,
        ipod: false,
        air_play: false
    };
    for e in reader {
        match e {
            Ok(xml::reader::XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_str() {
                    "Input" => {
                        in_inputs = true;
                    }
                    "Model_Name" => {
                        in_model_name = true;
                    }
                    "Feature_Existence" => {
                        in_feature_existence = true;
                    }
                    _ => {
                        if in_inputs {
                            current_input = Some(Input {
                                name: name.local_name,
                                display_name: None
                            });
                        }else if in_feature_existence {
                            current_feature = Some(name.local_name);
                        }
                    }
                }
            }
            Ok(xml::reader::XmlEvent::EndElement { name, .. }) => {
                match name.local_name.as_str() {
                    "Input" => {
                        in_inputs = false
                    }
                    "Model_Name" => {
                        in_model_name = false
                    }
                    "Feature_Existence" => {
                        in_feature_existence = false;
                    }
                    _ => {
                        if in_inputs {
                            inputs.push(current_input.unwrap());
                            current_input = None;
                        }else if in_feature_existence {
                            current_feature = None;
                        }
                    }
                }
            }
            Ok(xml::reader::XmlEvent::Characters(s)) => {
                if in_model_name {
                    model_name = Some(s.trim().to_owned());
                }else if in_inputs && current_input.is_some() {
                    current_input = Some(Input {
                        name: current_input.unwrap().name,
                        display_name: Some(s.trim().to_owned())
                    });
                }else if in_feature_existence {
                    if s == "1" {
                        match current_feature.as_ref().map(String::as_ref) {
                            Some("Main_Zone") => available_zones.push("Main_Zone".to_owned()),
                            Some("Zone_2") => available_zones.push("Zone_2".to_owned()),
                            Some("Zone_3") => available_zones.push("Zone_3".to_owned()),
                            Some("Tuner") => available_features.tuner = true,
                            Some("Zone_4") => available_zones.push("Zone_4".to_owned()),
                            Some("HD_Radio") => available_features.hd_radio = true,
                            Some("Rhapsody") => available_features.rhapsody = true,
                            Some("SIRIUS_IR") => available_features.sirius_ir = true,
                            Some("Pandora") => available_features.pandora = true,
                            Some("SERVER") => available_features.server = true,
                            Some("NET_RADIO") => available_features.net_radio = true,
                            Some("USB") => available_features.usb = true,
                            Some("iPod_USB") => available_features.ipod = true,
                            Some("AirPlay") => available_features.air_play = true,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let regex = regex::Regex::new(r"^([A-Z]+)_([0-9]+)$").unwrap();

    let inputs: Vec<Input> = inputs.iter_mut()
        .map(|input| {
            let input = input.clone();
            let mut name: String = input.name;
            if name == "V_AUX" {
                name = "V-AUX".to_owned();
            }else {
                let input_name = name.clone();
                let name_parts = regex.captures(&input_name);
                if name_parts.is_some() {
                    let name_parts = name_parts.unwrap();
                    name = format!("{}{}", &name_parts[1], &name_parts[2]);
                }
            }
            Input {
                name,
                display_name: input.display_name
            }
        }).collect();

    Ok(SystemConfig {
        inputs,
        model_name,
        available_zones,
        available_features
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Input {
        fn new(name: &'static str, display_name: Option<&'static str>) -> Input {
            Input {
                name: String::from(name),
                display_name: display_name.map(String::from)
            }
        }
    }

    #[test]
    fn parse_system_config_should_parse_xml() {
        let input = String::from("<YAMAHA_AV rsp=\"GET\" RC=\"0\"><System><Config><Model_Name>RX-V473</Model_Name><System_ID>05852093</System_ID><Version>1.14/1.04</Version><Feature_Existence><Main_Zone>1</Main_Zone><Zone_2>0</Zone_2><Zone_3>0</Zone_3><Zone_4>0</Zone_4><Tuner>1</Tuner><HD_Radio>0</HD_Radio><Rhapsody>0</Rhapsody><SIRIUS_IR>0</SIRIUS_IR><Pandora>0</Pandora><SERVER>1</SERVER><NET_RADIO>1</NET_RADIO><USB>1</USB><iPod_USB>1</iPod_USB><AirPlay>1</AirPlay></Feature_Existence><Name><Input><HDMI_1>  Chrome </HDMI_1><HDMI_2>Raspberry</HDMI_2><HDMI_3>   PC    </HDMI_3><HDMI_4>  Game   </HDMI_4><AV_1>         </AV_1><AV_2>   PC    </AV_2><AV_3>   TV    </AV_3><AV_4>         </AV_4><AV_5>   Wii   </AV_5><AV_6>Turntable</AV_6><V_AUX>  V-AUX  </V_AUX><USB>   USB   </USB></Input></Name></Config></System></YAMAHA_AV>");
        assert_eq!(parse_system_config(input).unwrap(), SystemConfig {
            inputs: vec![
                Input::new("HDMI1", Some("Chrome")),
                Input::new("HDMI2", Some("Raspberry")),
                Input::new("HDMI3", Some("PC")),
                Input::new("HDMI4", Some("Game")),
                Input::new("AV1", None),
                Input::new("AV2", Some("PC")),
                Input::new("AV3", Some("TV")),
                Input::new("AV4", None),
                Input::new("AV5", Some("Wii")),
                Input::new("AV6", Some("Turntable")),
                Input::new("V-AUX", Some("V-AUX")),
                Input::new("USB", Some("USB"))
            ],
            model_name: Some(String::from("RX-V473")),
            available_zones: vec![String::from("Main_Zone")],
            available_features: SystemConfigAvailableFeatures {
                tuner: true,
                hd_radio: false,
                rhapsody: false,
                sirius_ir: false,
                pandora: false,
                server: true,
                net_radio: true,
                usb: true,
                ipod: true,
                air_play: true
            }
        });
    }
}