use std::env;
use rand::Rng;
use super::Command;

pub fn command(args: &mut env::Args) -> Result<Command, String>
{
    match super::int_arg(args)
    {
        Ok(likely) => Ok(Command::PercentTrue(likely)),
        Err(e)     => Err(format!("likely arg: {}", e)),
    }
}

pub fn choose(likely: u32) -> String
{
    let ans = if rand::thread_rng().gen_bool(likely as f64 / 100.0)
    {
        "True"
    }
    else
    {
        "False"
    };
    String::from(ans)
}