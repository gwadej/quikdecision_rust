use quikdecision::{oracle,Decider,Decision::LabelledText};

fn main() -> Result<(),String>
{
    let oracle = oracle::command()?;

    println!("The Oracle answers:");
    for _ in 0..10
    {
        if let LabelledText{value, label: _} = oracle.decide()
        {
            println!("{}", value);
        }
    }
    Ok(())
}
