extern crate quikdecision;

use quikdecision::*;
use std::env;

fn main()
{
    match parse_args(env::args())
    {
        Ok(cmd) => {
            println!("{}",
                match cmd.decide()
                {
                    Decision::Text(ans) => ans,
                    Decision::Num(ans) => ans.to_string(),
                    Decision::AnnotatedNum(ans, extra) => format!("{}: {}", ans, extra),
                }
            )
        },
        Err(msg) => eprintln!("Error: {}", msg),
    };
}
