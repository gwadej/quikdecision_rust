extern crate quikdecision;

use std::env;
use quikdecision::*;

fn main()
{
    match parse_args(env::args())
    {
        Ok(cmd)  => println!("{}", cmd.decide()),
        Err(msg) => eprintln!("Error: {}", msg),
    };
}
