use crate::{Command, Decision, Decider};
use crate::ApiDoc;

use rand::Rng;
use regex::Regex;

type RollStep = (String, u32);

#[derive(Debug)]
pub enum Roll
{
    Dice(u32, u32),
    ExplodingDice(u32, u32),
    Incr(u32),
}

#[derive(Debug)]
pub struct Expr(Vec<Roll>);

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
            "The number of sides supported are 3, 4, 6, 8, 10, 12, 20, or 100. Exploding dice",
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
            .map_err(|_| "Non-number somehow passed parsing".to_string()),
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
        return Err("Missing dice expression".to_string());
    }

    let re = Regex::new(r"^\s*(?:(?P<num>(?:[1-9][0-9]*)?)(?P<type>[dDxX])(?P<sides>[3468]|1[02]|20|100)|(?P<val>[1-9][0-9]*))\s*$").unwrap();
    let mut descr: Vec<Roll> = Vec::new();
    for term in expr.split("+")
    {
        let cap = re.captures(&term)
                    .ok_or_else(|| "Failed parsing dice expression".to_string())?;
        descr.push(match (cap.name("num"), cap.name("sides"))
        {
            (Some(n), Some(s)) => match cap.name("type").unwrap().as_str()
            {
                "x" | "X" => make_exploding_dice(n, s)?,
                "d" | "D" => make_dice(n, s)?,
                _ => return Err("Unrecognized die type".to_string()),
            },
            (Some(_), None) => return Err("No sides specified".to_string()),
            (None, _) => match cap.name("val")
            {
                Some(n) => Roll::Incr(uint_from_match(n)?),
                None => return Err("Unparseable term".to_string()),
            },
        });
    }

    Ok(Command::RollDice(Expr(descr)))
}

fn roll_die<T>(rng: &mut T, sides: u32) -> RollStep
    where T: Rng
{
    incr_step(rng.gen_range(1, sides + 1))
}

fn accum_roll((desc, val): RollStep, (rdesc, roll): RollStep, sep: &str) -> RollStep
{
    if desc.is_empty()
    {
        return (rdesc, val + roll);
    }
    (desc + sep + &rdesc, val + roll)
}

fn roll_step<T>(rng: &mut T, num: u32, sides: u32) -> RollStep
    where T: Rng
{
    let (desc, roll) = (1..=num)
        .map(|_| roll_die(rng, sides))
        .fold((String::new(), 0), |acc, r| accum_roll(acc, r, "+"));
    (format!("{}d{}({})", num, sides, desc), roll)
}

fn roll_explode_step<T>(rng: &mut T, num: u32, sides: u32) -> RollStep
    where T: Rng
{
    let (desc, roll) = (1..=num)
        .map(|_| roll_exploded_step(rng, sides))
        .map(|(d, r)| (format!(" ({}) ", d), r))
        .fold((String::new(), 0), |acc, r| accum_roll(acc, r, "+"));
    (format!("{}x{}<{}>", num, sides, desc.trim()), roll)
}

fn roll_exploded_step<T>(rng: &mut T, sides: u32) -> RollStep
    where T: Rng
{
    let roll = roll_die(rng, sides);
    explode(rng, roll, sides)
}

fn incr_step(num: u32) -> RollStep
{
    (num.to_string(), num)
}

fn explode<T>(rng: &mut T, (desc, val): RollStep, sides: u32) -> RollStep
    where T: Rng
{
    if val != sides { return (desc, val); }

    let (rdesc, roll) = roll_exploded_step(rng, sides);
    (format!("{}!+{}", desc, rdesc), val + roll)
}

impl Decider for Expr {
    /// Perform the random function and return a Decision object representing
    /// the result.
    fn decide(&self) -> Decision
    {
        let mut rng = rand::thread_rng();
        // { value: roll, description: roll_string }
        let (desc, roll) = self.0
            .iter()
            .map(|ref x| match x
            {
                Roll::Dice(num, sides) => roll_step(&mut rng, *num, *sides),
                Roll::ExplodingDice(num, sides) => roll_explode_step(&mut rng, *num, *sides),
                Roll::Incr(num) => incr_step(*num),
            })
            .fold((String::new(), 0), |acc, r| accum_roll(acc, r, " + "));
        Decision::AnnotatedNum{ value: roll, extra: desc.to_string() }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use crate::Decision;
    use crate::DecisionAssertions;
    use crate::Decider;
    use crate::Command;
    use super::*;

    #[test]
    fn command_empty_string()
    {
        assert_that!(command(String::new()))
            .is_err_containing("Missing dice expression".to_string());
    }

    #[test]
    fn command_simple_roll()
    {
        assert_that!(command("3d8".into()))
            .is_ok_containing(Command::RollDice(Expr(vec![Roll::Dice(3, 8)])))
    }

    #[test]
    fn command_all_sides()
    {
        for i in vec![3,4,6,8,10,12,20,100]
        {
            assert_that!(command(format!("1d{}", i)))
                .is_ok_containing(Command::RollDice(Expr(vec![Roll::Dice(1, i)])))
        }
    }

    #[test]
    fn command_exploding_roll()
    {
        assert_that!(command("3x6".into()))
            .is_ok_containing(Command::RollDice(Expr(vec![Roll::ExplodingDice(3, 6)])))
    }

    #[test]
    fn command_multiterm_expresion()
    {
        assert_that!(command("2d12 + 3x6 + 2".into()))
            .is_ok_containing(Command::RollDice(Expr(vec![Roll::Dice(2, 12), Roll::ExplodingDice(3, 6), Roll::Incr(2)])))
    }

    #[test]
    fn dice_roll_decision()
    {
        assert_that!(command("2d12 + 3x6 + 2".into()).unwrap().decide())
            .matches_enum_variant(Decision::AnnotatedNum{value: 1, extra: "foo".into()});
    }

    #[test]
    fn simple_roll_value()
    {
        match Expr(vec![Roll::Dice(1, 6)]).decide()
        {
            Decision::AnnotatedNum{value, extra} => {
                assert_that!(&value).is_greater_than_or_equal_to(1);
                assert_that!(&value).is_less_than_or_equal_to(6);
                assert_that!(extra).starts_with("1d6(");
                assert_that!(extra).ends_with(")");
            },
            _ => assert!(false, "Wrong decision type"),
        }
    }

    #[test]
    fn explode_roll_value()
    {
        match Expr(vec![Roll::ExplodingDice(1, 6)]).decide()
        {
            Decision::AnnotatedNum{value, extra} => {
                assert_that!(&value).is_greater_than_or_equal_to(1);
                assert_that!(value % 6).is_not_equal_to(0);
                assert_that!(extra).starts_with("1x6<");
                assert_that!(extra).ends_with(">");
            },
            _ => assert!(false, "Wrong decision type"),
        }
    }

    #[test]
    fn incr_value()
    {
        match Expr(vec![Roll::Incr(1)]).decide()
        {
            Decision::AnnotatedNum{value, extra} => {
                assert_that!(value).is_equal_to(1);
                assert_that!(extra).is_equal_to("1".to_string());
            },
            _ => assert!(false, "Wrong decision type"),
        }
    }

    #[test]
    fn multi_roll_value()
    {
        match Expr(vec![Roll::Dice(3, 6)]).decide()
        {
            Decision::AnnotatedNum{value, extra} => {
                assert_that!(&value).is_greater_than_or_equal_to(3);
                assert_that!(&value).is_less_than_or_equal_to(18);
                assert_that!(extra).starts_with("3d6(");
                assert_that!(extra).ends_with(")");
            },
            _ => assert!(false, "Wrong decision type"),
        }
    }

    #[test]
    fn multi_exploding_roll_value()
    {
        match Expr(vec![Roll::ExplodingDice(3, 6)]).decide()
        {
            Decision::AnnotatedNum{value, extra} => {
                assert_that!(&value).is_greater_than_or_equal_to(3);
                assert_that!(extra).starts_with("3x6<");
                assert_that!(extra).ends_with(">");
            },
            _ => assert!(false, "Wrong decision type"),
        }
    }

    #[test]
    fn complex_roll_value()
    {
        match Expr(vec![Roll::Dice(3, 6), Roll::Dice(2, 8), Roll::ExplodingDice(1, 20), Roll::Incr(2)]).decide()
        {
            Decision::AnnotatedNum{value, extra} => {
                assert_that!(&value).is_greater_than_or_equal_to(8);
                assert_that!(extra).starts_with("3d6(");
                assert_that!(extra).contains(") + 2d8(");
                assert_that!(extra).contains(") + 1x20<");
                assert_that!(extra).contains("> + 2");
            },
            _ => assert!(false, "Wrong decision type"),
        }
    }
}
