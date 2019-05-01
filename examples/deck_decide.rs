use quikdecision::{deck,Decider};

fn main() -> Result<(),String>
{
    let deck = deck::command("52-card")?;

    println!("Randomly pick 10 cards:");
    for _ in 0..10
    {
        println!("{}", deck.decide());
    }
    Ok(())
}

