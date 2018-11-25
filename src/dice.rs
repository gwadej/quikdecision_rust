use ::Command;
use ::Decision;
use ::ApiDoc;

use rand::Rng;
use regex::Regex;

#[derive(Debug)]
pub enum Roll
{
    Dice(u32, u32),
    ExplodingDice(u32, u32),
    Incr(u32),
}

impl PartialEq for Roll
{
    fn eq(&self, other: &Roll) -> bool
    {
        match (self, other)
        {
            (Roll::Dice(sl, sh), Roll::Dice(ol, oh)) => sl == ol && sh == oh,
            (Roll::ExplodingDice(sl, sh), Roll::ExplodingDice(ol, oh)) => sl == ol && sh == oh,
            (Roll::Incr(val), Roll::Incr(oval)) => val == oval,
            (_, _) => false,
        }
    }
}

type RollStep = (String, u32);

/// Return an ApiDoc object describing the Dice decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "roll",
        params: vec!["dice expression"],
        hint: "Roll the described combination of dice",
        help: vec![
            "Roll the described combination of dice, returning a number and description of the",
            "roll. The {dice expr} is a combination of terms of one of three forms joined by +:",
            "  - {n}d{s}: roll n s-sided dice (3d6)",
            "  - {n}x{s}: roll n s-sided exploding dice (2x8)",
            "  - {n}: an increment.",
            "The number of sides supported are 4, 6, 8, 10, 12, 20, or 100. Exploding dice",
            "work much like normal, except when a die rolls the maximum value for the die,",
            "then it is re-rolled to generate a value to add to the original roll. This may",
            "happen more than once.",
            "The return is the sum of all of the rolls followed by a string representing the",
            "individual rolls.",
            "Normal dice are represented by the expression, followed by the results of the",
            "individual die rolls in parens: 3d6(3+5+1).",
            "Exploding dice are represented by the dice expression followed by an expression",
            "in angle brackets. The expression is made of the results of each individual die",
            "in parens, with any die that exploded being followed by an !, and the re-roll",
            "added as needed: 3x6<(1)+(6!+5)+(2)>."
        ],
    }
}

fn uint_from_match(m: regex::Match) -> Result<u32, String>
{
    match m.as_str()
    {
        ""   => Ok(1),
        nstr => nstr
            .parse::<u32>()
            .map_err(|_| String::from("Non-number somehow passed parsing")),
    }
}

fn make_dice(dice: regex::Match, sides: regex::Match) -> Result<Roll, String>
{
    Ok(Roll::Dice(uint_from_match(dice)?, uint_from_match(sides)?))
}

fn make_exploding_dice(dice: regex::Match, sides: regex::Match) -> Result<Roll, String>
{
    Ok(Roll::ExplodingDice(
        uint_from_match(dice)?,
        uint_from_match(sides)?,
    ))
}

/// Construct a Command object representing the dice to roll.
/// Expects a string containing a dice expression.
pub fn command(expr: String) -> Result<Command, String>
{
    if expr.is_empty()
    {
        return Err(String::from("Missing dice expression"));
    }

    let re = Regex::new(r"^\s*(?:(?P<num>(?:[1-9][0-9]*)?)(?P<type>[dDxX])(?P<sides>4|6|8|10|12|20|100)|(?P<val>[1-9][0-9]*))\s*$").unwrap();
    let mut descr: Vec<Roll> = vec![];
    for term in expr.split("+")
    {
        let cap = match re.captures(&term)
        {
            Some(c) => c,
            None => return Err(String::from("Failed parsing dice expression")),
        };
        descr.push(match (cap.name("num"), cap.name("sides"))
        {
            (Some(n), Some(s)) => match cap.name("type").unwrap().as_str()
            {
                "x" | "X" => make_exploding_dice(n, s)?,
                "d" | "D" => make_dice(n, s)?,
                _ => return Err(String::from("Unrecognized die type")),
            },
            (Some(_), None) => return Err(String::from("No sides specified")),
            (None, _) => match cap.name("val")
            {
                Some(n) => Roll::Incr(uint_from_match(n)?),
                None => return Err(String::from("Unparseable term")),
            },
        });
    }

    Ok(Command::RollDice(descr))
}

fn roll_die(rng: &mut rand::ThreadRng, sides: u32) -> RollStep
{
    incr_step(rng.gen_range(1, sides + 1))
}

fn accum_roll(acc: RollStep, roll: RollStep, sep: &str) -> RollStep
{
    if acc.0.is_empty()
    {
        return (roll.0, acc.1 + roll.1);
    }
    (acc.0 + sep + &roll.0, acc.1 + roll.1)
}

fn roll_step(num: u32, sides: u32) -> RollStep
{
    let mut rng = rand::thread_rng();
    let out = (1..=num)
        .map(|_| roll_die(&mut rng, sides))
        .fold((String::new(), 0), |acc, r| accum_roll(acc, r, "+"));
    (format!("{}d{}({})", num, sides, out.0), out.1)
}

fn roll_explode_step(num: u32, sides: u32) -> RollStep
{
    let mut rng = rand::thread_rng();
    let out = (1..=num)
        .map(|_| roll_die(&mut rng, sides))
        .map(|r| explode(r, sides))
        .map(|r| (format!(" ({}) ", r.0), r.1))
        .fold((String::new(), 0), |acc, r| accum_roll(acc, r, "+"));
    (format!("{}x{}<{}>", num, sides, out.0.trim()), out.1)
}

fn roll_exploded_step(sides: u32) -> RollStep
{
    let mut rng = rand::thread_rng();
    explode(roll_die(&mut rng, sides), sides)
}

fn incr_step(num: u32) -> RollStep
{
    (num.to_string(), num)
}

fn explode(val: RollStep, sides: u32) -> RollStep
{
    if val.1 != sides { return val; }

    let roll = roll_exploded_step(sides);
    (format!("{}!+{}", val.0, roll.0), val.1 + roll.1)
}

/// Perform the random function and return a Decision object representing
/// the result.
pub fn roll(descr: Vec<Roll>) -> Decision
{
    // { value: roll, description: roll_string }
    let val = descr
        .iter()
        .map(|ref x| match x
        {
            Roll::Dice(num, sides) => roll_step(*num, *sides),
            Roll::ExplodingDice(num, sides) => roll_explode_step(*num, *sides),
            Roll::Incr(num) => incr_step(*num),
        })
        .fold((String::new(), 0), |acc, r| accum_roll(acc, r, " + "));
    Decision::AnnotatedNum{ value: val.1, extra: val.0.to_string() }
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use ::Decision;
    use ::DecisionAssertions;
    use ::Decider;
    use ::Command;
    use super::*;

    #[test]
    fn command_empty_string()
    {
        assert_that!(command(String::new())).is_err()
            .is_equal_to(String::from("Missing dice expression"));
    }

    #[test]
    fn command_simple_roll()
    {
        assert_that!(command("3d8".into())).is_ok()
            .is_equal_to(Command::RollDice(vec![Roll::Dice(3, 8)]))
    }

    #[test]
    fn command_exploding_roll()
    {
        assert_that!(command("3x6".into())).is_ok()
            .is_equal_to(Command::RollDice(vec![Roll::ExplodingDice(3, 6)]))
    }

    #[test]
    fn command_multiterm_expresion()
    {
        assert_that!(command("2d12 + 3x6 + 2".into())).is_ok()
            .is_equal_to(Command::RollDice(vec![Roll::Dice(2, 12), Roll::ExplodingDice(3, 6), Roll::Incr(2)]))
    }

    #[test]
    fn dice_roll_decision()
    {
        assert_that!(command("2d12 + 3x6 + 2".into()).unwrap().decide())
            .matches_enum_variant(Decision::AnnotatedNum{value: 1, extra: "foo".into()});
    }
}
