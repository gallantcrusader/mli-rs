use std::process::Command;
use sysinfo::{System, SystemExt};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum PlayerError{
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
        
        let mut vecc: Vec<String> = Vec::new();

        let mus = s.processes_by_exact_name("Music").next();
        let spot = s.processes_by_exact_name("Spotify").next();
        match mus{
            Some(_) =>  vecc.push("Music".to_owned()),
            None => (),
        }
        match spot{
            Some(_) => vecc.push("Spotify".to_owned()),
            None => vecc.push("Music".to_owned()),
        }
        Player {
            players: vecc,
        }
    }

    pub fn play(&self) -> Result<String, PlayerError> {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "{}"
            if it is running then
            play
            else
		    return "not running"
        	end if
            end tell"#,
                self.players.get(0).unwrap()
            ))
            .output();
        if let Ok(b) = a{
            print!("{}", std::str::from_utf8(&b.stderr)?);
            Ok(std::str::from_utf8(&b.stdout)?.to_string())
        }else {
            Err(PlayerError::OutputError)
        }
        

        
    }
    pub fn pause(&self) -> Result<String, PlayerError> {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "{}"
            if it is running then
            pause
            else
		    return "not running"
        	end if
            end tell"#,
                self.players.get(0).unwrap()
            ))
            .output();
        if let Ok(b) = a{
            print!("{}", std::str::from_utf8(&b.stderr)?);
            Ok(std::str::from_utf8(&b.stdout)?.to_string())
        }else {
            Err(PlayerError::OutputError)
        }
    }
    pub fn next(&self) -> Result<String, PlayerError> {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "{}"
            if it is running then
            next track
            else
		    return "not running"
        	end if
            end tell"#,
                self.players.get(0).unwrap()
            ))
            .output();
        if let Ok(b) = a{
            print!("{}", std::str::from_utf8(&b.stderr)?);
            Ok(std::str::from_utf8(&b.stdout)?.to_string())
        }else {
            Err(PlayerError::OutputError)
        }
    }
    pub fn previous(&self) -> Result<String, PlayerError> {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "{}"
            if it is running then
            back track
            else
		    return "not running"
        	end if
            end tell"#,
                self.players.get(0).unwrap()
            ))
            .output();
        if let Ok(b) = a{
            print!("{}", std::str::from_utf8(&b.stderr)?);
            Ok(std::str::from_utf8(&b.stdout)?.to_string())
        }else {
            Err(PlayerError::OutputError)
        }
    }
    pub fn get_track(&self) -> Result<String, PlayerError> {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "{}"
            if it is running then
            get name of current track
            else
		    return "not running"
        	end if
            end tell"#,
                match self.players.get(0) {
                    Some(a) => a,
                    None => "Music",
                }
            ))
            .output();
        if let Ok(b) = a{
            print!("{}", std::str::from_utf8(&b.stderr)?);
            Ok(std::str::from_utf8(&b.stdout)?.to_string())
        }else {
            Err(PlayerError::OutputError)
        }
    }
}
