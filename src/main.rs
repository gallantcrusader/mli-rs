use clap::Command;
//use ansi_term::Colour::*;
mod scriptwrap;
use scriptwrap::Player;


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
    
    
    

    //programs need time to run?? crazy.. .

    match m.subcommand() {
        Some(("pause", _)) => print!("{}",Player::pause("Music")?),
        Some(("play", _)) => print!("{}",Player::play("Music")?),
        Some(("next", _)) => print!("{}",Player::next("Music")?),
        Some(("previous", _)) => print!("{}",Player::previous("Music")?),
        _ => {
            print!("{}",Player::get_track("Music")?)
        }
    }

    Ok(())
}
