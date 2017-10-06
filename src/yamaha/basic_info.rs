extern crate xml;

use xml::reader::XmlEvent;
use std::io::Result;

#[derive(PartialEq, Debug, Clone)]
pub struct BasicInfo {
    pub power: bool,
    pub volume: i32,
    pub mute: bool,
    pub input: String
}

pub fn parse_basic_info(xml: String) -> Result<BasicInfo> {
    let reader = xml::reader::EventReader::from_str(&xml);
    let mut current_element: Option<&'static str> = None;
    let mut in_val: bool = false;
    let mut basic_info = BasicInfo {
        power: false,
        volume: 0,
        mute: false,
        input: "".to_owned()
    };

    for element in reader {
        match element {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_str() {
                    "Power" => current_element = Some("Power"),
                    "Volume" => current_element = Some("Volume"),
                    "Mute" => current_element = Some("Mute"),
                    "Input_Sel" => current_element = Some("Input"),
                    "Val" => in_val = true,
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                match name.local_name.as_str() {
                    "Power" => current_element = None,
                    "Volume" => current_element = None,
                    "Mute" => current_element = None,
                    "Input_Sel" => current_element = None,
                    "Val" => in_val = false,
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(s)) => {
                match current_element {
                    Some("Power") => {
                        basic_info.power = if s == "On" { true } else { false };
                    }
                    Some("Mute") => {
                        basic_info.mute = if s == "On" { true } else { false };
                    }
                    Some("Input") => {
                        basic_info.input = s;
                    }
                    Some("Volume") => {
                        if in_val {
                            basic_info.volume = s.parse().unwrap();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    return Ok(basic_info);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_info_should_parse_xml() {
        let input = String::from("<YAMAHA_AV rsp=\"GET\" RC=\"0\"><Main_Zone><Basic_Status><Power_Control><Power>On</Power><Sleep>Off</Sleep></Power_Control><Volume><Lvl><Val>-300</Val><Exp>1</Exp><Unit>dB</Unit></Lvl><Mute>Off</Mute></Volume><Input><Input_Sel>HDMI1</Input_Sel><Input_Sel_Item_Info><Param>HDMI1</Param><RW>RW</RW><Title>  Chrome </Title><Icon><On>/YamahaRemoteControl/Icons/icon004.png</On><Off></Off></Icon><Src_Name></Src_Name><Src_Number>1</Src_Number></Input_Sel_Item_Info></Input><Surround><Program_Sel><Current><Straight>Off</Straight><Enhancer>Off</Enhancer><Sound_Program>5ch Stereo</Sound_Program></Current></Program_Sel><_3D_Cinema_DSP>Off</_3D_Cinema_DSP></Surround><Sound_Video><Tone><Bass><Val>0</Val><Exp>1</Exp><Unit>dB</Unit></Bass><Treble><Val>0</Val><Exp>1</Exp><Unit>dB</Unit></Treble></Tone><Direct><Mode>Off</Mode></Direct><HDMI><Standby_Through_Info>On</Standby_Through_Info><Output><OUT_1>On</OUT_1></Output></HDMI><Adaptive_DRC>Off</Adaptive_DRC></Sound_Video></Basic_Status></Main_Zone></YAMAHA_AV>");
        assert_eq!(parse_basic_info(input).unwrap(), BasicInfo {
            power: true,
            volume: -300,
            mute: false,
            input: "HDMI1".to_owned()
        });
    }
}