use quikdecision::{percent,Decider};

fn main() -> Result<(),String>
{
    let likely = percent::command(35)?;

    println!("True 35% of the time:");
    for _ in 0..10
    {
        println!("{}", likely.decide());
    }
    Ok(())
}
