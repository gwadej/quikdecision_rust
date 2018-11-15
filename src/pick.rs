use ::Command;
use ::Decision;
use ::Hint;
use ::HintList;

use rand::Rng;

pub fn int_arg<T>(opt: Option<String>) -> Result<T, String>
where
    T: std::str::FromStr,
{
    match opt
    {
        None => Err(String::from("Missing required parameter")),
        Some(arg) => match arg.parse::<T>()
        {
            Ok(a) => Ok(a),
            Err(_) => Err(String::from("Argument not a valid integer")),
        },
    }
}

pub fn command(low: Option<String>, high: Option<String>) -> Result<Command, String>
{
    match (int_arg::<i32>(low), int_arg::<i32>(high))
    {
        (Ok(low), Ok(high)) if low == high => return Err(String::from("High parameter cannot equal low parameter")),
        (Ok(low), Ok(high)) if low > high => Ok(Command::PickNumber(high, low)),
        (Ok(low), Ok(high)) => Ok(Command::PickNumber(low, high)),
        (Err(e),  _) => return Err(format!("low arg: {}", e)),
        (_,       Err(e)) => return Err(format!("high arg: {}", e)),
    }
}

pub fn choose(low: i32, high: i32) -> Decision
{
    Decision::Num(rand::thread_rng().gen_range(low, high + 1))
}

pub fn hint() -> HintList
{
    vec![
        Hint {
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
        let low: i32 = 2;
        let high: i32 = 10;
        let expected = ["2", "3", "4", "5", "6", "7", "8", "9", "10"];

        for _ in 1..=NUM_TRIES
        {
            let choice = super::choose(low, high);
            assert_ne!(expected.iter().find(|&&x| x == choice), None);
        }
    }
}
