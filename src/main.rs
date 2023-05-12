use clap::Command;
mod scriptwrap;
use scriptwrap::{Player, PlayerError};

#[cfg(target_os = "macos")]
fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("[ERROR] {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), PlayerError> {
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
        Some(("pause", _)) => print!("{}", player.send_command("pause")?),
        Some(("play", _)) => print!("{}", player.send_command("play")?),
        Some(("next", _)) => print!("{}", player.send_command("next track")?),
        Some(("previous", _)) => print!("{}", player.send_command("back track")?),
        _ => {
            print!("{}", player.send_command("get name of current track")?);
        }
    }
    Ok(())
}
