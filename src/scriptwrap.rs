use std::process::Command;
use sysinfo::{System, SystemExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("Invalid UTF-8: {0}")]
    TextError(#[from] std::str::Utf8Error),
    #[error("Unable to get terminal output!")]
    OutputError,
}

pub struct Player {
    players: Vec<String>,
}

impl Player {
    pub fn init() -> Self {
        let mut s = System::new();
        s.refresh_processes();

        let mut the_vec_that_contains_players: Vec<String> = Vec::new();

        let spot = s.processes_by_exact_name("Spotify").next();


        if spot.is_some() {
            the_vec_that_contains_players.push("Spotify".to_owned());
        } else {
            the_vec_that_contains_players.push("Music".to_owned());
        }
        Player { players: the_vec_that_contains_players }
    }

    pub fn send_command(&self, command: &str) -> Result<String, PlayerError>
    {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "{}"
            if it is running then
            {}
            else
		    return "not running"
        	end if
            end tell"#,
                self.players.get(0).unwrap(),
                command
            ))
            .output();
        if let Ok(b) = a {
            print!("{}", std::str::from_utf8(&b.stderr)?);
            Ok(std::str::from_utf8(&b.stdout)?.to_string())
        } else {
            Err(PlayerError::OutputError)
        }
    }

}
