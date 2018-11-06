use super::Command;
use help;
use rand::Rng;
use std::env;

pub fn command(args: &mut env::Args) -> Result<Command, String>
{
    let low = match super::int_arg::<i32>(args)
    {
        Ok(val) => val,
        Err(e) => return Err(format!("low arg: {}", e)),
    };
    let high = match super::int_arg::<i32>(args)
    {
        Ok(val) => val,
        Err(e) => return Err(format!("high arg: {}", e)),
    };
    if low == high
    {
        return Err(String::from("High parameter cannot equal low parameter"));
    }
    if low > high
    {
        return Ok(Command::PickNumber(high, low));
    }

    Ok(Command::PickNumber(low, high))
}

pub fn choose(low: i32, high: i32) -> String
{
    let guess = rand::thread_rng().gen_range(low, high + 1);
    guess.to_string()
}

pub fn hint() -> Vec<help::Hint>
{
    vec![
        help::Hint {
            cmd: "pick",
            clue: "pick {low} {high}",
            blurb: "pick a number between {low} and {high}",
            help: vec![
                "Selects a number between two supplied values (inclusive) with equal probability.",
                "The two numbers cannot be the same.",
            ],
        },
    ]
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
        let expected = ["2", "3", "4", "5", "6", "7", "8", "9", "10"];

        for _ in 1..=NUM_TRIES
        {
            let choice = super::choose(low, high);
            assert_ne!(expected.iter().find(|&&x| x == choice), None);
        }
    }
}
