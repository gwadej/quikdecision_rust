use quikdecision::{shuffle,Decider,Decision::List};

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
        if let List(value) = shuffle.decide()
        {
            println!("{}", value.join(", "));
        }
    }
    Ok(())
}

