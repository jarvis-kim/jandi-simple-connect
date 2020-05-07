extern crate clap;
extern crate ansi_term;

use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Style;
use clap::{Arg, App};
use jsc::jandi::connect;

fn main() {
    let version = "0.1.0";

    let matches = App::new("Jandi Simple Connect")
        .version(version)
        .about("Send message by connect")
        .arg(Arg::with_name("team_id")
            .short("i")
            .alias("ti")
            .long("team_id")
            .takes_value(true)
            .required(true)
            .validator(|id| {
                let i = &id[..];
                return match i.parse::<i32>() {
                    Ok(n) => Ok(()),
                    Err(_) => Err(String::from(format!("{} is not number", id)))
                }
            })
            .help("your team id")
        )
        .arg(Arg::with_name("token")
            .short("t")
            .long("token")
            .takes_value(true)
            .required(true)
            .help("jandi incomming token"))
        .arg(Arg::with_name("message")
            .short("m")
            .long("msg")
            .takes_value(true)
            .required(true)
            .help("connect message body")
        ).get_matches();

    let team_id = matches.value_of("team_id")
        .map(|s: &str| { s.parse::<i32>().unwrap() })
        .unwrap();

    let token = matches.value_of("token").unwrap();
    let message = matches.value_of("message").unwrap();

    let result = connect::send(team_id, token, message);
    match result {
        Ok(status) => println!("send success: {}", Green.paint(status.to_string()).to_string()),
        Err(res) => {
            println!("fail. status: {}. reason: {}",
                     Red.paint(res.status.to_string()), Red.paint(res.reason));
            println!("token: {}, message: {}",
                     Style::new().bold().paint(Red.paint(token).to_string()),
                     Style::new().bold().paint(Red.paint(message).to_string())
            );
        }
    }
}
