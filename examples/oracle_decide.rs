use quikdecision::{oracle,Decider};

fn main() -> Result<(),String>
{
    let oracle = oracle::command()?;

    println!("The Oracle answers:");
    for _ in 0..10
    {
        println!("{}", oracle.decide());
    }
    Ok(())
}
