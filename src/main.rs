use clap::Command;
//use ansi_term::Colour::*;
mod scriptwrap;
use scriptwrap::Player;

#[cfg(target_os = "macos")]
fn main() {
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
        Some(("pause", _)) => print!("{}", player.send_command("pause").unwrap()),
        Some(("play", _)) => print!("{}", player.send_command("play").unwrap()),
        Some(("next", _)) => print!("{}", player.send_command("next track").unwrap()),
        Some(("previous", _)) => print!("{}", player.send_command("back track").unwrap()),
        _ => {
            print!("{}", player.send_command("get name of current track").unwrap());
        }
    }
}
