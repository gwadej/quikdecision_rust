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

fn uint_from_match(m: regex::Match) -> Result<u32, String>
{
    if m.as_str() == ""
    {
        return Ok(1);
    }
    match m.as_str().parse::<u32>()
    {
        Ok(n)  => Ok(n),
        Err(_) => Err(String::from("Non-number somehow passed parsing")),
    }
}

fn make_dice(dice: Option<regex::Match>, sides: Option<regex::Match>) -> Result<Roll,String>
{
    let dice  = match dice
    {
        None    => return Err(String::from("No dice specified")),
        Some(n) => uint_from_match(n)?
    };

    let sides = match sides
    {
        None    => return Err(String::from("No sides specified")),
        Some(n) => uint_from_match(n)?
    };
    Ok(Roll::Dice(dice, sides))
}

fn make_exploding_dice(dice: Option<regex::Match>, sides: Option<regex::Match>) -> Result<Roll,String>
{
    let dice  = match dice
    {
        None    => return Err(String::from("No dice specified")),
        Some(n) => uint_from_match(n)?
    };

    let sides = match sides
    {
        None    => return Err(String::from("No sides specified")),
        Some(n) => uint_from_match(n)?
    };
    Ok(Roll::ExplodingDice(dice, sides))
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
        if cap.name("num").is_some()
        {
            let dice = match cap.name("type").unwrap().as_str()
            {
                "x" | "X" => make_exploding_dice(cap.name("num"), cap.name("sides"))?,
                "d" | "D" => make_dice(cap.name("num"), cap.name("sides"))?,
                _         => return Err(String::from("Unrecognized die type")),
            };
            descr.push(dice);
        }
        else if let Some(incr) = cap.name("val")
        {
            descr.push( Roll::Incr(uint_from_match(incr)?) );
        }
        else
        {
            return Err(String::from("Unparseable term"));
        }
    }

    Ok(Command::RollDice(descr))
}

fn roll_step(num: u32, sides: u32) -> u32
{
    let mut rng = rand::thread_rng();
    (1..=num)
        .map(|_| rng.gen_range(1, sides+1))
        .sum::<u32>()
}

fn roll_explode_step(num: u32, sides: u32) -> u32
{
    let mut rng = rand::thread_rng();
    (1..=num)
        .map(|_| rng.gen_range(1, sides+1))
        .map(|n| explode(n, sides))
        .sum::<u32>()
}

fn explode(val: u32, sides: u32) -> u32
{
    if val == sides
    {
        sides+roll_explode_step(1, sides)
    }
    else
    {
        val
    }
}

pub fn roll(descr: Vec<Roll>) -> String
{
    let value = descr.iter()
        .map(|ref x| match x
             {
                 Roll::Dice(num,sides)          => roll_step(*num, *sides),
                 Roll::ExplodingDice(num,sides) => roll_explode_step(*num, *sides),
                 Roll::Incr(num)                => *num,
             })
        .sum::<u32>();
    value.to_string()
}

