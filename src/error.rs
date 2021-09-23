#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Sonos(#[from] sonos::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[error("Unknown command \"{0}\", run \"{} help\" to see available commands", env!("CARGO_PKG_NAME"))]
    UnknownCommand(String),
    #[error("Missing command, run \"{0} help\" to see available commands", env!("CARGO_PKG_NAME"))]
    MissingCommand,
    #[error("Unknown transport state")]
    UnknownTransportState,
}

/*impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self::Other(s.into())
    }
}*/
