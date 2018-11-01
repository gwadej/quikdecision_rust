use std::env;
use rand::Rng;
use super::Command;

pub fn command(args: &mut env::Args) -> Result<Command, String>
{
    let likely = match super::int_arg::<u32>(args)
    {
        Ok(val) => val,
        Err(e)  => return Err(format!("likely arg: {}", e)),
    };
    if likely > 100
    {
        return Err(String::from("likely arg cannot be greater than 100 percent"));
    }
    Ok(Command::PercentTrue(likely))
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

#[cfg(test)]
mod tests
{
    #[test]
    fn percent_test()
    {
        let choices: usize = (1..=1000)
            .map(|_| super::choose(35))
            .filter(|x| x == "True")
            .count();
        assert!(300 <= choices && choices <= 400);
    }
}
