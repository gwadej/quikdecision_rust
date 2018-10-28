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

#[cfg(test)]
mod tests
{
    const NUM_TRIES: u32 = 3;

    #[test]
    fn choose_a_small_number()
    {
        let expected = ["1", "2"];

        for _ in 1..=NUM_TRIES
        {
            let choice = super::choose(1, 2);
            assert_ne!(expected.iter().find(|&&x| x == choice), None);
        }
    }

    #[test]
    fn choose_a_larger_number()
    {
        let low: u32 = 2;
        let high: u32 = 10;
        let expected = [
            "2", "3", "4", "5", "6",
            "7", "8", "9", "10",
        ];

        for _ in 1..=NUM_TRIES
        {
            let choice = super::choose(low, high);
            assert_ne!(expected.iter().find(|&&x| x == choice), None);
        }
    }
}
