use quikdecision::{shuffle,Decider};

fn main() -> Result<(),String>
{
    let shuffle = shuffle::command(vec![
        "David", "Kirsten",
        "Mark",  "Connie",
        "Bryan", "Aramis",
        "Fred",  "Bianca",
    ].iter().map(|s| s.to_string()).collect::<Vec<_>>())?;

    println!("Names:");
    for _ in 0..10
    {
        println!("{}", shuffle.decide());
    }
    Ok(())
}

