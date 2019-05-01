use quikdecision::{dice,Decider,Decision::AnnotatedNum};

fn main() -> Result<(),String>
{
    let dice = dice::command("3d6".to_string())?;

    println!("Roll 3 6-sided dice:");
    for _ in 0..10
    {
        if let AnnotatedNum{value, extra} = dice.decide()
        {
            println!("{:>2}: [{}]", value, extra);
        }
    }
    Ok(())
}
