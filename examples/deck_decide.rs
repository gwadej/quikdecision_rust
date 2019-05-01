use quikdecision::{deck,Decider,Decision::Card};

fn main() -> Result<(),String>
{
    let deck = deck::command("52-card")?;

    println!("Randomly pick 10 cards:");
    for _ in 0..10
    {
        if let Card(card) = deck.decide()
        {
            println!("{}", card.to_string());
        }
    }
    Ok(())
}

