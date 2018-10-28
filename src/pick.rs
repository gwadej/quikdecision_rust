use std::env;
use rand::Rng;
use super::Command;

pub fn command(args: &mut env::Args) -> Result<Command, String>
{
    let low = match super::int_arg(args)
    {
        Ok(val) => val,
        Err(e)  => return Err(format!("low arg: {}", e)),
    };
    let high = match super::int_arg(args)
    {
        Ok(val) => val,
        Err(e)  => return Err(format!("high arg: {}", e)),
    };

    Ok(Command::PickNumber(low, high))
}

pub fn choose(low: u32, high: u32) -> String
{
    let guess = rand::thread_rng().gen_range(low, high+1);
    guess.to_string()
}
