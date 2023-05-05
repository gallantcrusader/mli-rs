use std::process::Command;

pub struct Player;

impl Player{
    pub fn play(player_name: &str) -> Result<String,Box<dyn std::error::Error>>
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
    pub fn pause(player_name: &str) -> Result<String,Box<dyn std::error::Error>>
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
    pub fn next(player_name: &str) -> Result<String,Box<dyn std::error::Error>>
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
    pub fn previous(player_name: &str) -> Result<String,Box<dyn std::error::Error>>
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
    pub fn get_track(player_name: &str) -> Result<String,Box<dyn std::error::Error>>
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
