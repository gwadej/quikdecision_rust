use quikdecision::{percent,Decider,Decision::Bool};

fn main() -> Result<(),String>
{
    let likely = percent::command(35)?;

    println!("True 35% of the time:");
    for _ in 0..10
    {
        if let Bool(value) = likely.decide()
        {
            println!("{}", value);
        }
    }
    Ok(())
}
