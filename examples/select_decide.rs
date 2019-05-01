use quikdecision::{select,Decider,Decision::Text};

fn main() -> Result<(),String>
{
    let choose = select::command(vec![
        "Burgers", "BBQ",
        "Italian", "French",
        "Pizza", "Tex-Mex",
        "Cajun", "Mediterranean",
        "Seafood", "Greek",
        "Indian", "Chinese",
        "Thai", "Vietnamese",
        "Sushi", "Steak",
    ].iter().map(|s| s.to_string()).collect::<Vec<_>>())?;

    println!("Cuisine:");
    for _ in 0..10
    {
        if let Text(value) = choose.decide()
        {
            println!("{}", value);
        }
    }
    Ok(())
}
