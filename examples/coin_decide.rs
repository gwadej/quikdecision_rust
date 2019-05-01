use quikdecision::{coin,Decider,Decision::Text};

fn main() -> Result<(),String>
{
    let coin = coin::command()?;

    println!("Flipping a coin 10 times:");
    for _ in 0..10
    {
        if let Text(face) = coin.decide()
        {
            println!("{}", face);
        }
    }
    Ok(())
}
