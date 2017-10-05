extern crate xml;
extern crate regex;

use hyper;
use std::result;
use std::io::Result;

use http::exec;

pub struct YamahaAvr {
    ip: String
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub model_name: Option<String>,
    pub inputs: Vec<Input>,
    pub available_zones: Vec<String>,
    pub available_features: SystemConfigAvailableFeatures
}

#[derive(Debug, Clone)]
pub struct Input {
    /// The Internal name, used for input selection
    pub name: String,
    /// A User set Display Name, None when the name is empty
    pub display_name: Option<String>
}

fn parse_system_config(xml: String) -> Result<SystemConfig> {
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

impl YamahaAvr {
    pub fn new(ip: String) -> YamahaAvr {
        YamahaAvr {
            ip
        }
    }

    fn exec(&mut self, xml: String) -> result::Result<String, hyper::Error> {
        exec(self.ip.clone(), xml)
    }

    pub fn set_power(&mut self, value: bool) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><Main_Zone><Power_Control><Power>{}</Power></Power_Control></Main_Zone></YAMAHA_AV>", if value { "On" } else { "Standby" });
        self.exec(cmd).unwrap();
        Ok(())
    }

    pub fn set_mute(&mut self, value: bool) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><Main_Zone><Volume><Mute>{}</Mute></Volume></Main_Zone></YAMAHA_AV>", if value { "On" } else { "Off" });
        self.exec(cmd).unwrap();
        Ok(())
    }

    /// Select a Input for the given Zone or Main_Zone when zone is None.
    ///
    /// Does nothing when Input is not available
    ///
    /// # Arguments
    ///
    /// * `input` - The Input to select
    /// * `zone` - The Zone in which the input should be selected. Defaults to Main_Zone
    ///
    pub fn select_input(&mut self, input: String, zone: Option<&str>) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><{zone}><Input><Input_Sel>{}</Input_Sel></Input></{zone}></YAMAHA_AV>", input, zone = zone.unwrap_or("Main_Zone"));
        self.exec(cmd).unwrap();
        Ok(())
    }

    pub fn get_inputs(&mut self) -> Result<Vec<Input>> {
        let config = self.get_system_config()?;
        Ok(config.inputs)
    }

    pub fn get_zones(&mut self) -> Result<Vec<String>> {
        let config = self.get_system_config()?;
        Ok(config.available_zones)
    }

    pub fn get_system_config(&mut self) -> Result<SystemConfig> {
        let cmd = "<YAMAHA_AV cmd=\"GET\"><System><Config>GetParam</Config></System></YAMAHA_AV>".to_owned();
        let res = self.exec(cmd).unwrap();
        parse_system_config(res)
    }
}