pub struct Hint
{
    pub cmd: &'static str,
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
            print_help_seg(h.clue, &vec![h.blurb]);
        }
        else
        {
            print_help_seg(h.clue, &h.help);
        }
    }
}

fn print_help_seg(clue: &str, help: &Vec<&str>)
{
    println!("  {}", clue);
    for h in help
    {
        println!(" {:8}{}", "", h);
    }
}

pub fn usage(progname: String, cmd: Option<String>, hints: Vec<Vec<Hint>>) -> !
{
    match cmd
    {
        None => {
            println!("{} {}\n", progname, "{command} [cmd_args ...]");
            println!("{}\n", "where {command} is one of:");
            for h in hints
            {
                print_hint(h);
            }
        },
        Some(c) => {
            for h in find_hints(&hints, c)
            {
                print_hint_seg(h.clue, h.blurb);
            }
        },
    }

    std::process::exit(1);
}

fn find_hints<'a>(hints: &'a Vec<Vec<Hint>>, cmd: String) -> Vec<&'a Hint>
{
    hints.iter()
        .flat_map(|hvec| hvec.iter())
        .filter(|h| h.cmd == cmd)
        .collect()
}

pub fn help(progname: String, cmd: Option<String>, hints: Vec<Vec<Hint>>) -> !
{
    match cmd
    {
        None => {
            println!("{} {}\n", progname, "{command} [cmd_args ...]");
            println!("{}\n", "where {command} is one of:");
            for h in hints
            {
                print_help(h);
            }
        },
        Some(c) => {
            for h in find_hints(&hints, c)
            {
                print_help_seg(h.clue, &h.help);
            }
        },
    }

    std::process::exit(1);
}

pub fn hint() -> Vec<Hint>
{
    vec![
        Hint {
            cmd: "help",
            clue: "help [cmd]",
            blurb: "The help screen, or help on a particular command if one is supplied.",
            help: vec![],
        },
        Hint {
            cmd: "man",
            clue: "man [cmd]",
            blurb: "The full help description, or full help on a particular command.",
            help: vec![
                "A long form description of the various commands.",
                "If a command name is supplied, provice the full help for that",
                "command only."
            ],
        },
    ]
}
