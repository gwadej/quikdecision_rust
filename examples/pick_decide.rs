use quikdecision::{pick,Decider};

fn main() -> Result<(),String>
{
    let num = pick::command(1, 20)?;

    println!("Number between 1 and 20:");
    for _ in 0..10
    {
        println!("{}", num.decide());
    }
    Ok(())
}
