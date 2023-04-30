use clap::Command;
//use ansi_term::Colour::*;
use mpris::{PlaybackStatus, PlayerFinder};



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let m = Command::new("mli")
        .author("Gallant")
        .version("0.1.0")
        .about("controls music lmao")
        .subcommand(Command::new("pause"))
        .subcommand(Command::new("play"))
        .subcommand(Command::new("next"))
        .subcommand(Command::new("previous"))
        .get_matches();

    let pl_find = PlayerFinder::new()
        .map_err(|e| format!("[ERROR] D-Bus denied: {}", e))?;
    let pl = pl_find
        .find_active()
        .map_err(|e| format!("[ERROR] No player: {}",e))?;

    //programs need time to run?? crazy.. .

    match m.subcommand() {
        Some(("pause", _)) => pl.pause()?,
        Some(("play", _)) => pl.play()?,
        Some(("next", _)) => pl.next()?,
        Some(("previous", _)) => pl.previous()?,
        _ => {
            let status = pl
                .get_playback_status()
                .map_err(|e| format!("[ERROR] Can't get status: {}",e))?;
            if status != PlaybackStatus::Stopped
            {
                println!("currently playing: {:?}", pl
                         .get_metadata().unwrap()
                         .title().unwrap());
            }
        }
    }

    Ok(())
}
