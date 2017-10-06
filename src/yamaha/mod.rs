use hyper;
use std::result;
use std::io::Result;

mod http;
mod system_config;
mod basic_info;

pub struct YamahaAvr {
    ip: String
}

impl YamahaAvr {
    pub fn new(ip: String) -> YamahaAvr {
        YamahaAvr {
            ip
        }
    }

    fn exec(&mut self, xml: String) -> result::Result<String, hyper::Error> {
        http::exec(self.ip.clone(), xml)
    }

    pub fn get_basic_info(&mut self) -> result::Result<basic_info::BasicInfo, hyper::Error> {
        let cmd = "<YAMAHA_AV cmd=\"GET\"><Main_Zone><Basic_Status>GetParam</Basic_Status></Main_Zone></YAMAHA_AV>".to_owned();
        let res = self.exec(cmd)?;
        let info = basic_info::parse_basic_info(res)?;
        Ok(info)
    }

    pub fn get_power(&mut self) -> Result<bool> {
        let info = self.get_basic_info().unwrap();
        Ok(info.power)
    }

    pub fn set_power(&mut self, value: bool) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><Main_Zone><Power_Control><Power>{}</Power></Power_Control></Main_Zone></YAMAHA_AV>", if value { "On" } else { "Standby" });
        self.exec(cmd).unwrap();
        Ok(())
    }

    pub fn get_mute(&mut self) -> Result<bool> {
        let info = self.get_basic_info().unwrap();
        Ok(info.mute)
    }

    pub fn set_mute(&mut self, value: bool) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><Main_Zone><Volume><Mute>{}</Mute></Volume></Main_Zone></YAMAHA_AV>", if value { "On" } else { "Off" });
        self.exec(cmd).unwrap();
        Ok(())
    }

    pub fn get_volume(&mut self) -> Result<i32> {
        let info = self.get_basic_info().unwrap();
        Ok(info.volume)
    }

    pub fn set_volume(&mut self, value: i32) -> Result<()> {
        let cmd = format!("<YAMAHA_AV cmd=\"PUT\"><Main_Zone><Volume><Lvl><Val>{}</Val><Exp>1</Exp><Unit>dB</Unit></Lvl></Volume></Main_Zone></YAMAHA_AV>", value);
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

    pub fn get_inputs(&mut self) -> Result<Vec<system_config::Input>> {
        let config = self.get_system_config()?;
        Ok(config.inputs)
    }

    pub fn get_zones(&mut self) -> Result<Vec<String>> {
        let config = self.get_system_config()?;
        Ok(config.available_zones)
    }

    pub fn get_system_config(&mut self) -> Result<system_config::SystemConfig> {
        let cmd = "<YAMAHA_AV cmd=\"GET\"><System><Config>GetParam</Config></System></YAMAHA_AV>".to_owned();
        let res = self.exec(cmd).unwrap();
        system_config::parse_system_config(res)
    }
}