use game_of_life::game;
use crossterm::Result;
use clap::{Arg, Command};
use std::process::exit;

fn main() -> Result<()> {
    let app = Command::new("game_of_life")
        .arg(
            Arg::new("time")
                .long("time")
                .short('t')
                .takes_value(true)
                .help("Refresh time in milliseconds")
                .required(true)
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .takes_value(true)
                .help("input file")
                .required(true)
        );


    let args = app.get_matches();
    let t: u64 = match args.value_of("time").unwrap().parse::<u64>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid value of -t (--time) argument");
            exit(1);
        }
    };

    let f: &str = args.value_of("file").unwrap();

    game::play(&t, f)?;
    Ok(())
}
