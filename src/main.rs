use regex :: Regex;
use clap::{Command,Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn main() {
    let args = Command::new("lite-grep")
    .version("v0.1.0")
    .about("A grep clone built for learning Rust concepts by Mitesh")
    .arg(
        Arg::new("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true)
    ).
    arg(
        Arg::new("input")
        .help("File to search")
        .takes_value(true)
        .required(true)
    )
    .get_matches();   

    let pattern = args.value_of("pattern").unwrap();

    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();

    let f = File::open(input).unwrap();//panics if the file path doesnot exist
    let reader = BufReader::new(f);

    // let text = "\
    // Every face, every shop,
    // bedroom window, public-house, and
    // dark square is a picture
    // feverishly turned--in search of what?
    // It is the same with books.
    // What do we seek
    // through millions of pages?";

    for line in reader.lines(){
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}",line),
            None => (),
        }
    }
}
