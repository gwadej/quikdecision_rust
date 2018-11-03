pub struct Hint
{
    pub clue: &'static str,
    pub blurb: &'static str,
    pub help: Vec<&'static str>,
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

fn print_help(hints: Vec<Hint>)
{
    for h in hints
    {
        if h.help.len() == 0
        {
            print_help_seg(h.clue, vec![h.blurb]);
        }
        else
        {
            print_help_seg(h.clue, h.help);
        }
    }
}

fn print_help_seg(clue: &str, help: Vec<&str>)
{
    println!("  {}", clue);
    for h in help
    {
        println!(" {:8}{}", "", h);
    }
}

pub fn usage(progname: String, hints: Vec<Vec<Hint>>) -> !
{
    println!("{} {}\n", progname, "{command} [cmd_args ...]");
    println!("{}\n", "where {command} is one of:");
    for h in hints
    {
        print_hint(h);
    }

    std::process::exit(1);
}

pub fn help(progname: String, hints: Vec<Vec<Hint>>) -> !
{
    println!("{} {}\n", progname, "{command} [cmd_args ...]");
    println!("{}\n", "where {command} is one of:");
    for h in hints
    {
        print_help(h);
    }

    std::process::exit(1);
}

pub fn hint() -> Vec<Hint>
{
    vec![
        Hint {
            clue: "help",
            blurb: "The help screen",
            help: vec![],
        },
        Hint {
            clue: "man",
            blurb: "The full help description.",
            help: vec![
                "A long form description of the various commands.",
            ],
        },
    ]
}
