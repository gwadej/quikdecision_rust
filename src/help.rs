pub struct Hint
{
    pub clue: &'static str,
    pub blurb: &'static str,
}


fn print_hint(hints: Vec<Hint>)
{
    for h in hints
    {
        print_hint_seg(h.clue, h.blurb);
    }
}

fn print_hint_seg(clue: &str, blurb: &str)
{
    if clue.len() < 8
    {
        println!("  {:8}- {}", clue, blurb);
    }
    else
    {
        println!("  {}\n  {:8}- {}", clue, "", blurb);
    }
}

pub fn usage(progname: String, hints: Vec<Vec<super::Hint>>) -> !
{
    println!("{} {}\n", progname, "{command} [cmd_args ...]");
    println!("{}\n", "where {command} is one of:");
    for h in hints
    {
        print_hint(h);
    }

    std::process::exit(1);
}

pub fn hint() -> Vec<Hint>
{
    vec![
        Hint {
            clue: "help",
            blurb: "The help screen",
        }
    ]
}
