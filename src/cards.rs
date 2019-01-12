use crate::Command;
use crate::Decision;
use crate::ApiDoc;

use rand::Rng;

#[derive(Debug)]
pub enum Card
{
    Wild(&'static str),
    Ace(&'static str),
    Pip{suit: &'static str, value: u32},
    Court{suit: &'static str, face: &'static str},
    Trump{name: &'static str, value: u32},
}

impl Card
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Card::Wild(name) => name.to_string(),
            Card::Ace(suit)  => format!("Ace of {}", suit),
            Card::Pip{ suit, value } => format!("{} of {}", value, suit),
            Card::Court{ suit, face } => format!("{} of {}", face, suit),
            Card::Trump{ name, value } => format!("{} ({})", name, value),
        }
    }

    pub fn short_name(&self) -> String
    {
        match self
        {
            Card::Wild(name) => name.to_string(),
            Card::Ace(suit)  => format!("A{}", suit.get(..1).unwrap()),
            Card::Pip{ suit, value } => format!("{}{}", value, suit.get(..1).unwrap()),
            Card::Court{ suit, face } => format!("{}{}", face, suit.get(..1).unwrap()),
            Card::Trump{ name:_, value } => format!("Tr{}", value),
        }
    }
}

const SUITS: [&str; 4] = [
    "Spades",
    "Hearts",
    "Diamonds",
    "Clubs",
];
const FACES: [&'static str; 3] = [ "Jack", "Queen", "King" ];

/// Create a DrawCard Command
pub fn command() -> Result<Command, String>
{
    Ok(Command::DrawCard)
}

/// Draw a card from the deck
pub fn draw() -> Decision
{
    let num = rand::thread_rng().gen_range(0, 52);
    let (suit, value) = (num / 13, (num % 13) + 1);
    let card = match value
    {
        1       => Card::Ace(SUITS[suit]),
        2...10  => Card::Pip{ suit: SUITS[suit], value: value as u32 },
        11...13 => Card::Court{ suit: SUITS[suit], face: FACES[value-11] },
        _ => panic!("Should not get here"),
    };
    Decision::Card(card)
}

/// Return an ApiDoc object containing a description of the DrawCard
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "card",
        params: Vec::new(),
        hint: "Draw a random card from the deck",
        help: vec![
            "Draw a random card from the deck",
        ],
    }
}
