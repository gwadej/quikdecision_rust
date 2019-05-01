use quikdecision::{dice,Decision::AnnotatedNum};

fn main() -> Result<(),String>
{
    let dice = dice::command( "3d6".to_string() )?;

    for roll in dice.iter().take(10)
    {
        match roll
        {
            AnnotatedNum{value, extra} => println!("{:>2} [{}]", value, extra),
            _ => println!("Unrecognized response"),
        };
    }

    Ok(())
}
