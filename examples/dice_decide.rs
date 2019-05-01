use quikdecision::{dice,Decider};

fn main() -> Result<(),String>
{
    let dice = dice::command("3d6".to_string())?;

    println!("Roll 3 6-sided dice:");
    for _ in 0..10
    {
        println!("{}", dice.decide());
    }
    Ok(())
}
