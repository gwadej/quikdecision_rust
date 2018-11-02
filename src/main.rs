extern crate quikdecision;

use quikdecision::*;
use std::env;

fn main()
{
    match parse_args(env::args())
    {
        Ok(cmd) => println!("{}", cmd.decide()),
        Err(msg) => eprintln!("Error: {}", msg),
    };
}
