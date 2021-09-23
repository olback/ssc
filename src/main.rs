use {
    config::Config,
    error::Error,
    sonos::{Speaker, TransportState},
    std::io::ErrorKind,
};

mod config;
mod error;

type PreCommand = (&'static str, fn() -> Result<(), Error>);
type Command = (&'static str, fn(&Speaker, &Config) -> Result<(), Error>);

const PRE_COMMANDS: &[PreCommand] = &[
    ("help", || {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        println!("Usage: {} [command]", env!("CARGO_PKG_NAME"));
        println!("Available commands: ");
        for c in COMMANDS
            .iter()
            .map(|c| c.0)
            .chain(PRE_COMMANDS.iter().map(|c| c.0))
        {
            println!("\t{}", c);
        }
        Ok(())
    }),
    ("create-config", || {
        Config::write()?;
        println!("Config written to {:?}", Config::path()?);
        Ok(())
    }),
];

const COMMANDS: &[Command] = &[
    ("volume-up", |s, c| {
        let volume = s.volume()?;
        println!("Current volume: {}", volume);
        let new_volume = volume + c.step;
        println!("New volume: {}", new_volume);
        s.set_volume(new_volume)?;
        Ok(())
    }),
    ("volume-down", |s, c| {
        let volume = s.volume()?;
        println!("Current volume: {}", volume);
        let new_volume = volume - c.step;
        println!("New volume: {}", new_volume);
        s.set_volume(new_volume)?;
        Ok(())
    }),
    ("play-pause", |s, _| {
        let state = s.transport_state()?;
        match state {
            TransportState::Playing => {
                println!("Current state: Playing");
                s.pause()?;
                println!("New state: Paused");
            }
            TransportState::PausedPlayback => {
                println!("Current state: Paused");
                s.play()?;
                println!("New state: Playing");
            }
            _ => return Err(Error::UnknownTransportState),
        }
        Ok(())
    }),
    ("next-track", |s, _| {
        let current_track = s.track()?;
        println!(
            "Current track: {} - {}",
            current_track.title, current_track.artist
        );
        s.next()?;
        let next_track = s.track()?;
        println!("Next track: {} - {}", next_track.title, next_track.artist);
        Ok(())
    }),
    ("previous-track", |s, _| {
        let current_track = s.track()?;
        println!(
            "Current track: {} - {}",
            current_track.title, current_track.artist
        );
        s.previous()?;
        let next_track = s.track()?;
        println!("Next track: {} - {}", next_track.title, next_track.artist);
        Ok(())
    }),
];

fn main() -> Result<(), Error> {
    let res: Result<(), Error> = (|| {
        let arg = std::env::args().nth(1).ok_or(Error::MissingCommand)?;

        for (c, f) in PRE_COMMANDS {
            if *c == arg {
                f()?;
                return Ok(());
            }
        }

        let conf = match Config::read() {
            Ok(c) => c,
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("Config.toml not found");
                        eprintln!("Create config with 'ssc create-config'");
                        return Ok(());
                    }
                    e => eprintln!("{:?}", e),
                }
                return Err(Error::from(e));
            }
        };

        let speaker = Speaker::from_ip(conf.ip)?;

        for (c, f) in COMMANDS {
            if *c == arg {
                println!("Command: {}", c);
                f(&speaker, &conf)?;
                return Ok(());
            }
        }

        Err(Error::UnknownCommand(arg))
    })();

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            Err(e)
        }
    }
}
