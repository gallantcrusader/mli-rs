use clap::Command;
//use ansi_term::Colour::*;
mod scriptwrap;
use scriptwrap::Player;

fn main(){
    if cfg!(linux) || cfg!(windows){
        panic!("Get out of here scamp!");
    }
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
        Some(("pause", _)) => print!("{}", player.pause().unwrap()),
        Some(("play", _)) => print!("{}", player.play().unwrap()),
        Some(("next", _)) => print!("{}", player.next().unwrap()),
        Some(("previous", _)) => print!("{}", player.previous().unwrap()),
        _ => {
            print!("{}", player.get_track().unwrap());
        }
    }

    
}
