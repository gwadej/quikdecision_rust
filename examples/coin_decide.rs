use quikdecision::{coin,Decider};

fn main() -> Result<(),String>
{
    let coin = coin::command()?;

    println!("Flipping a coin 10 times:");
    for _ in 0..10
    {
        println!("{}", coin.decide());
    }
    Ok(())
}
