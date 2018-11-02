use std::env;
use rand::Rng;
use regex::Regex;
use super::Command;

pub enum Roll
{
    Dice(u32, u32),
    ExplodingDice(u32, u32),
    Incr(u32),
}

type RollStep = (String, u32);

fn uint_from_match(m: regex::Match) -> Result<u32, String>
{
    match m.as_str()
    {
        ""   => Ok(1),
        nstr => match nstr.parse::<u32>()
        {
            Ok(n)  => Ok(n),
            Err(_) => Err(String::from("Non-number somehow passed parsing")),
        }
    }
}

fn make_dice(dice: regex::Match, sides: regex::Match) -> Result<Roll,String>
{
    Ok(Roll::Dice(uint_from_match(dice)?, uint_from_match(sides)?))
}

fn make_exploding_dice(dice: regex::Match, sides: regex::Match) -> Result<Roll,String>
{
    Ok(Roll::ExplodingDice(uint_from_match(dice)?, uint_from_match(sides)?))
}

pub fn command(args: &mut env::Args) -> Result<Command, String>
{
    let expr = match args.next()
    {
        Some(e) => e,
        None    => return Err(String::from("Missing dice expression")),
    };

    let re = Regex::new(r"^\s*(?:(?P<num>(?:[1-9][0-9]*)?)(?P<type>[dDxX])(?P<sides>4|6|8|10|12|20|100)|(?P<val>[1-9][0-9]*))\s*$").unwrap();
    let mut descr: Vec<Roll> = vec![];
    for term in expr.split("+")
    {
        let cap = match re.captures(&term)
        {
            Some(c) => c,
            None    => return Err(String::from("Failed parsing dice expression")),
        };
        descr.push(match (cap.name("num"), cap.name("sides"))
        {
            (Some(n), Some(s)) => match cap.name("type").unwrap().as_str()
            {
                "x" | "X" => make_exploding_dice(n, s)?,
                "d" | "D" => make_dice(n, s)?,
                _         => return Err(String::from("Unrecognized die type")),
            },
            (Some(_), None) => return Err(String::from("No sides specified")),
            (None, _)  => match cap.name("val")
            {
                Some(n) => Roll::Incr(uint_from_match(n)?),
                None    => return Err(String::from("Unparseable term")),
            }
        });
    }

    Ok(Command::RollDice(descr))
}

fn roll_die(rng: &mut rand::ThreadRng, sides: u32) -> RollStep
{
    let n = rng.gen_range(1, sides+1);
    (n.to_string(), n)
}

fn accum_roll(acc: RollStep, roll: RollStep, sep: &str) -> RollStep
{
    if acc.0.len() == 0
    {
        return (roll.0, acc.1+roll.1)
    }
    (acc.0 + sep + &roll.0, acc.1+roll.1)
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
        .fold((String::new(), 0), |acc, r| accum_roll(acc, r, "+"));
    (format!("{}x{}[{}]", num, sides, trim(out.0)), out.1)
}

fn trim(instr: String) -> String
{
    String::from(instr.trim_start().trim_end())
}

fn roll_exploded_step(sides: u32) -> RollStep
{
    let mut rng = rand::thread_rng();
    let r = roll_die(&mut rng, sides);
    explode(r, sides)
}

fn incr_step(num: u32) -> RollStep
{
    (num.to_string(), num)
}

fn explode(val: RollStep, sides: u32) -> RollStep
{
    if val.1 != sides
    {
        return val;
    }

    let roll = roll_exploded_step(sides);
    (format!(" ({}!+{}) ",val.0, roll.0), val.1+roll.1)
}

pub fn roll(descr: Vec<Roll>) -> String
{
    let val = descr.iter()
        .map(|ref x| match x
             {
                 Roll::Dice(num,sides)          => roll_step(*num, *sides),
                 Roll::ExplodingDice(num,sides) => roll_explode_step(*num, *sides),
                 Roll::Incr(num)                => incr_step(*num),
             })
        .fold((String::new(), 0), |acc, r| accum_roll(acc, r, " + "));
        format!("{}: {}", val.1, val.0)
}
