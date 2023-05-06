use std::process::Command;

pub struct Player{
    players: Vec<String>
};

impl Player{
    pub fn init() -> Self
    {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(r#"if application "Spotify" is running
            log "Spotify"
            end if 
            if application "Music" is running
            log "Music"
            end if
            
            "#)
    }

    pub fn play(&self) -> Result<String,Box<dyn std::error::Error>>
    {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"tell application "{player_name}"
            if it is running then
            play
            else
		    return "not running"
        	end if
            end tell"#))
            .output()?;
        print!("{}",std::str::from_utf8(&a.stderr)?);

         Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
    pub fn pause(&self) -> Result<String,Box<dyn std::error::Error>>
    {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"tell application "{player_name}"
            if it is running then
            pause
            else
		    return "not running"
        	end if
            end tell"#))
            .output()?;
        print!("{}",std::str::from_utf8(&a.stderr)?);

         Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
    pub fn next(&self) -> Result<String,Box<dyn std::error::Error>>
    {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"tell application "{player_name}"
            if it is running then
            next track
            else
		    return "not running"
        	end if
            end tell"#))
            .output()?;
        print!("{}",std::str::from_utf8(&a.stderr)?);

         Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
    pub fn previous(&self) -> Result<String,Box<dyn std::error::Error>>
    {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"tell application "{player_name}"
            if it is running then
            back track
            else
		    return "not running"
        	end if
            end tell"#))
            .output()?;
        print!("{}",std::str::from_utf8(&a.stderr)?);

         Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
    pub fn get_track(&self) -> Result<String,Box<dyn std::error::Error>>
    {
        let a = Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"tell application "{player_name}"
            if it is running then
            get name of current track
            else
		    return "not running"
        	end if
            end tell"#))
            .output()?;
        print!("{}",std::str::from_utf8(&a.stderr)?);

         Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
}
