use std::process::Command;
use sysinfo::{System, SystemExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("[ERROR] Invalid UTF-8: {0}")]
    TextError(#[from] std::string::FromUtf8Error),
    #[error("[ERROR] Unable to get terminal output!")]
    OutputError,
}

pub struct Player {
    player: String,
}

impl Player {
    pub fn init() -> Self {
        let mut s = System::new();
        s.refresh_processes();

        let spot = s.processes_by_exact_name("Spotify").next();

        if spot.is_some() {
            Player {
                player: "Spotify".to_owned(),
            }
        } else {
            Player {
                player: "Music".to_owned(),
            }
        }
    }

    pub fn send_command(&self, command: &str) -> Result<String, PlayerError> {
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
                self.player, command
            ))
            .output();
        if let Ok(b) = a {
            print!("{}", String::from_utf8(b.stderr)?);
            
            Ok(String::from_utf8(b.stdout)?)
        } else {
            Err(PlayerError::OutputError)
        }
    }
}
