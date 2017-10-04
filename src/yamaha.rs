extern crate xml;
extern crate regex;

use std::io::Result;

use http::exec;

pub struct YamahaAvr {
    ip: String
}

#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub inputs: Vec<Input>
}

#[derive(Debug, Clone)]
pub struct Input {
    pub name: String,
    pub display_name: Option<String>
}

fn parse_system_config(xml: String) -> Result<SystemConfig> {
    let mut reader = xml::reader::EventReader::from_str(&xml);
    let mut inputs: Vec<Input> = Vec::new();
    let mut in_inputs = false;
    let mut current_input: Option<Input> = None;
    for e in reader {
        match e {
            Ok(xml::reader::XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "Input" {
                    in_inputs = true;
                }else if in_inputs {
                    current_input = Some(Input {
                        name: name.local_name,
                        display_name: None
                    });
                }
            }
            Ok(xml::reader::XmlEvent::EndElement { name, .. }) => {
                if in_inputs {
                    if name.local_name == "Input" {
                        in_inputs = false;
                    }else {
                        inputs.push(current_input.unwrap());
                        current_input = None;
                    }
                }
            }
            Ok(xml::reader::XmlEvent::Characters(s)) => {
                if in_inputs && current_input.is_some() {
                    current_input = Some(Input {
                        name: current_input.unwrap().name,
                        display_name: Some(s.trim().to_owned())
                    });
                }
            }
            _ => {}
        }
    }

    let regex = regex::Regex::new(r"^([A-Z]+)_([0-9]+)$").unwrap();

    let inputs: Vec<Input> = inputs.iter_mut()
        .map(|input| {
            let i = input.clone();
            let mut name: String = i.name;
            if input.name == "V_AUX" {
                name = "V-AUX".to_owned();
            }else {
                let iname = name.clone();
                let cap = regex.captures(&iname);
                if cap.is_some() {
                    let cap = cap.unwrap();
                    name = format!("{}{}", &cap[1], &cap[2])
                }
            }
            Input {
                name,
                display_name: i.display_name
            }
        }).collect();

    Ok(SystemConfig { inputs })
}

impl YamahaAvr {
    pub fn new(ip: String) -> YamahaAvr {
        YamahaAvr {
            ip
        }
    }

    fn exec(&mut self, xml: String) -> Result<()> {
        exec(self.ip.clone(), xml)
    }

    pub fn power(&mut self, value: bool) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><Main_Zone><Power_Control><Power>{}</Power></Power_Control></Main_Zone></YAMAHA_AV>", if value { "On" } else { "Standby" });
        self.exec(cmd).unwrap();
        Ok(())
    }

    pub fn mute(&mut self, value: bool) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><Main_Zone><Volume><Mute>{}</Mute></Volume></Main_Zone></YAMAHA_AV>", if value { "On" } else { "Off" });
        self.exec(cmd).unwrap();
        Ok(())
    }

    pub fn select_input(&mut self, input: String) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><Main_Zone><Input><Input_Sel>{}</Input_Sel></Input></Main_Zone></YAMAHA_AV>", input);
        self.exec(cmd).unwrap();
        Ok(())
    }

    pub fn get_inputs(&mut self) -> Result<Vec<Input>> {
        let config = self.get_system_config()?;
        Ok(config.inputs)
    }

    fn get_system_config(&mut self) -> Result<SystemConfig> {
        let cmd = "<YAMAHA_AV cmd=\"GET\"><System><Config>GetParam</Config></System></YAMAHA_AV>".to_owned();
        let res = "<YAMAHA_AV rsp=\"GET\" RC=\"0\"><System><Config><Model_Name>RX-V473</Model_Name><System_ID>05852093</System_ID><Version>1.14/1.04</Version><Feature_Existence><Main_Zone>1</Main_Zone><Zone_2>0</Zone_2><Zone_3>0</Zone_3><Zone_4>0</Zone_4><Tuner>1</Tuner><HD_Radio>0</HD_Radio><Rhapsody>0</Rhapsody><SIRIUS_IR>0</SIRIUS_IR><Pandora>0</Pandora><SERVER>1</SERVER><NET_RADIO>1</NET_RADIO><USB>1</USB><iPod_USB>1</iPod_USB><AirPlay>1</AirPlay></Feature_Existence><Name><Input><HDMI_1>  Chrome </HDMI_1><HDMI_2>Raspberry</HDMI_2><HDMI_3>   PC    </HDMI_3><HDMI_4>  Game   </HDMI_4><AV_1>         </AV_1><AV_2>   PC    </AV_2><AV_3>   TV    </AV_3><AV_4>         </AV_4><AV_5>   Wii   </AV_5><AV_6>Turntable</AV_6><V_AUX>  V-AUX  </V_AUX><USB>   USB   </USB></Input></Name></Config></System></YAMAHA_AV>".to_owned();
        parse_system_config(res)
    }
}