use ::Command;
use ::Decision;
use ::ApiDoc;

use rand::Rng;

/// Create a PickNumber command based on the two supplied values
/// Return either the command or an error if the parameters are not appropriate.
pub fn command(low: i32, high: i32) -> Result<Command, String>
{
    match (low, high)
    {
        (l, h) if l == h => return Err(String::from("High parameter cannot equal low parameter")),
        (l, h) if l > h => Ok(Command::PickNumber(h, l)),
        (l, h) => Ok(Command::PickNumber(l, h)),
    }
}

/// Return a numeric Decision with a value between low and high (inclusive).
pub fn choose(low: i32, high: i32) -> Decision
{
    Decision::Num(rand::thread_rng().gen_range(low, high + 1))
}

/// Return an ApiDoc object containing a description of the PickNumber
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "pick",
        params: vec!["low", "high"],
        hint: "pick a number between {low} and {high}",
        help: vec![
            "Selects a number between two supplied values (inclusive) with equal probability.",
            "The two numbers cannot be the same.",
        ],
    }
}

#[cfg(test)]
mod tests
{
    const NUM_TRIES: u32 = 3;
    use ::Decision;

    #[test]
    fn choose_a_small_number()
    {
        let expected = [1, 2];

        for _ in 1..=NUM_TRIES
        {
            match super::choose(1, 2)
            {
                Decision::Num(choice) => assert_ne!(expected.iter().find(|&&x| x == choice), None),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn choose_a_larger_number()
    {
        let low: i32 = 2;
        let high: i32 = 10;
        let expected = [2, 3, 4, 5, 6, 7, 8, 9, 10];

        for _ in 1..=NUM_TRIES
        {
            match super::choose(low, high)
            {
                Decision::Num(choice) => assert_ne!(expected.iter().find(|&&x| x == choice), None),
                _ => panic!(),
            }
        }
    }
}
