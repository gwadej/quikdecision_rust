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
                    Decision::LabeledText{ value, label } => format!("{}: \"{}\"", label, value),
                    Decision::Num(ans) => ans.to_string(),
                    Decision::AnnotatedNum{ value, extra } => format!("{}: {}", value, extra),
                    Decision::Bool(ans) => ans.to_string(),
                }
            )
        },
        Err(msg) => eprintln!("Error: {}", msg),
    };
}
