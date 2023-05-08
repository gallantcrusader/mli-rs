use std::process::Command;

pub struct Player {
    players: Vec<String>,
}

impl Player {
    pub fn init() -> Self {
        let spot = Command::new("osascript")
            .arg("-e")
            .arg(
                r#"if application "Spotify" is running then
            return "Spotify"
            end if 
            "#,
            )
            .output()
            .unwrap()
            .stdout;
        let mus = Command::new("osascript")
            .arg("-e")
            .arg(
                r#"if application "Music" is running then
            return "Music"
            end if"#,
            )
            .output()
            .unwrap()
            .stdout;
        let mut vecc: Vec<String> = Vec::new();

        if spot.len() > 0 {
            vecc.push(std::str::from_utf8(&spot).unwrap().trim().to_string());
        }
        if mus.len() > 0 {
            vecc.push(std::str::from_utf8(&mus).unwrap().trim().to_string());
        }
        if vecc.len() == 0 {
            //fuck it
            vecc.push("Music".to_owned());
        }
        Player { players: vecc }
    }

    pub fn play(&self) -> Result<String, Box<dyn std::error::Error>> {
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
            .output()?;
        print!("{}", std::str::from_utf8(&a.stderr)?);

        Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
    pub fn pause(&self) -> Result<String, Box<dyn std::error::Error>> {
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
            .output()?;
        print!("{}", std::str::from_utf8(&a.stderr)?);

        Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
    pub fn next(&self) -> Result<String, Box<dyn std::error::Error>> {
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
            .output()?;
        print!("{}", std::str::from_utf8(&a.stderr)?);

        Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
    pub fn previous(&self) -> Result<String, Box<dyn std::error::Error>> {
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
            .output()?;
        print!("{}", std::str::from_utf8(&a.stderr)?);

        Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
    pub fn get_track(&self) -> Result<String, Box<dyn std::error::Error>> {
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
            .output()?;
        print!("{}", std::str::from_utf8(&a.stderr)?);

        Ok(std::str::from_utf8(&a.stdout)?.to_string())
    }
}
