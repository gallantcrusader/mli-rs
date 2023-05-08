use clap::Command;
//use ansi_term::Colour::*;
mod scriptwrap;
use scriptwrap::Player;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = Command::new("mli")
        .author("Gallant")
        .version("0.1.0")
        .about("controls music lmao")
        .subcommand(Command::new("pause"))
        .subcommand(Command::new("play"))
        .subcommand(Command::new("next"))
        .subcommand(Command::new("previous"))
        .get_matches();

    let player = Player::init();

    //programs need time to run?? crazy.. .

    match m.subcommand() {
        Some(("pause", _)) => print!("{}", player.pause()?),
        Some(("play", _)) => print!("{}", player.play()?),
        Some(("next", _)) => print!("{}", player.next()?),
        Some(("previous", _)) => print!("{}", player.previous()?),
        _ => {
            print!("{}", player.get_track()?);
        }
    }

    Ok(())
}
