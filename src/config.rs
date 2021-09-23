use {
    dirs,
    serde::{Deserialize, Serialize},
    std::{
        fs,
        io::{self, ErrorKind},
        net::IpAddr,
        path::PathBuf,
    },
    toml,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub ip: IpAddr,
    pub step: u8,
}

impl Default for Config {
    fn default() -> Self {
        use std::str::FromStr;
        Self {
            ip: IpAddr::from_str("192.168.1.105").unwrap(),
            step: 1,
        }
    }
}

impl Config {
    pub fn read() -> Result<Self, io::Error> {
        Ok(toml::from_str::<Self>(&fs::read_to_string(Self::path()?)?)?)
    }

    pub fn write() -> Result<(), crate::Error> {
        Ok(fs::write(
            Self::path()?,
            toml::to_string_pretty(&Self::default())?,
        )?)
    }

    pub fn path() -> Result<PathBuf, io::Error> {
        let dir = dirs::config_dir()
            .ok_or(io::Error::new(ErrorKind::NotFound, ""))?
            .join(env!("CARGO_PKG_NAME"));
        if !dir.exists() {
            fs::create_dir(&dir)?;
        }
        Ok(dir.join("Config.toml"))
    }
}
