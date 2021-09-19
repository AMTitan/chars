use clap::*;
use rand::seq::SliceRandom;
use std::str::FromStr;
use std::process;

fn main() {
    let matches = App::new("Char")
        .version("1.0")
        .about("Password Maker")
        .arg(Arg::with_name("Numbers")
            .short("n")
            .long("numbers")
            .help("sets the password to have no numbers")
            .takes_value(false))
        .arg(Arg::with_name("Special")
            .short("s")
            .long("special")
            .help("sets the password to have no special chars")
            .takes_value(false))
        .arg(Arg::with_name("Capital")
            .short("c")
            .long("capital")
            .help("sets the password to have no capitals")
            .takes_value(false))
        .arg(Arg::with_name("Lowercase")
            .short("l")
            .long("lowercase")
            .help("sets the password to have no lowercase letters")
            .takes_value(false))
        .arg(Arg::with_name("INPUT")
            .help("Number of chars")
            .required(true)
            .index(1))
        .get_matches();
    let mut chars:Vec<char> = Vec::new();
    if !matches.is_present("Numbers") {
        for x in ['0','1','2','3','4','5','6','7','8','9'] {
            chars.push(x);
        }
    }
    if !matches.is_present("Special") {
        for x in ['!','@','#','$','%','^','&','*','(',')'] {
            chars.push(x);
        }
    }
    if !matches.is_present("Capital") {
        for x in ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'] {
            chars.push(x);
        }
    }
    if !matches.is_present("Lowercase") {
        for x in ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'] {
            chars.push(x);
        }
    }
    if chars.len() == 0 {
        println!("You can block all the types try again");
        process::exit(1);
    }
    let mut password:String = String::new();
    for _x in 0..FromStr::from_str(matches.value_of("INPUT").unwrap()).unwrap() {
        password.push_str(chars.choose(&mut rand::thread_rng()).unwrap().to_string().as_str())
    }
    println!("{}", password);
}
