use crate::Command;
use crate::Decision;
use crate::ApiDoc;

use rand::Rng;

#[derive(Debug)]
pub struct Card
{
    pub glyph: char,
    suit: &'static str,
    number: usize,
}

//#[derive(Debug)]
//pub enum Card
//{
//    Wild(&'static str),
//    Ace(&'static str),
//    Pip{suit: &'static str, value: u32},
//    Court{suit: &'static str, face: &'static str},
//    Trump{name: &'static str, value: u32},
//}
//
//impl Card
//{
//    pub fn to_string(&self) -> String
//    {
//        match self
//        {
//            Card::Wild(name) => name.to_string(),
//            Card::Ace(suit)  => format!("Ace of {}", suit),
//            Card::Pip{ suit, value } => format!("{} of {}", value, suit),
//            Card::Court{ suit, face } => format!("{} of {}", face, suit),
//            Card::Trump{ name, value } => format!("{} ({})", name, value),
//        }
//    }
//
//    pub fn short_name(&self) -> String
//    {
//        match self
//        {
//            Card::Wild(name) => name.to_string(),
//            Card::Ace(suit)  => format!("A{}", suit.get(..1).unwrap()),
//            Card::Pip{ suit, value } => format!("{}{}", value, suit.get(..1).unwrap()),
//            Card::Court{ suit, face } => format!("{}{}", face, suit.get(..1).unwrap()),
//            Card::Trump{ name:_, value } => format!("Tr{}", value),
//        }
//    }
//}

const SUITS: [&str; 4] = [
    "Spades",
    "Hearts",
    "Diamonds",
    "Clubs",
];
const FACES: [&'static str; 3] = [ "Jack", "Queen", "King" ];
//const JOKERS: [&'static str; 3] = [ "Black Joker", "Red Joker", "White Joker" ];
const GLYPHS: &str =
    "\u{1F0A1}\u{1F0A2}\u{1F0A3}\u{1F0A4}\u{1F0A5}\u{1F0A6}\u{1F0A7}\u{1F0A8}\u{1F0A9}\u{1F0AA}\u{1F0AB}\u{1F0AD}\u{1F0AE}\
     \u{1F0B1}\u{1F0B2}\u{1F0B3}\u{1F0B4}\u{1F0B5}\u{1F0B6}\u{1F0B7}\u{1F0B8}\u{1F0B9}\u{1F0BA}\u{1F0BB}\u{1F0BD}\u{1F0BE}\
     \u{1F0C1}\u{1F0C2}\u{1F0C3}\u{1F0C4}\u{1F0C5}\u{1F0C6}\u{1F0C7}\u{1F0C8}\u{1F0C9}\u{1F0CA}\u{1F0CB}\u{1F0CD}\u{1F0CE}\
     \u{1F0D1}\u{1F0D2}\u{1F0D3}\u{1F0D4}\u{1F0D5}\u{1F0D6}\u{1F0D7}\u{1F0D8}\u{1F0D9}\u{1F0DA}\u{1F0DB}\u{1F0DD}\u{1F0DE}\
     \u{1F0BF}\u{1F0CF}\u{1F0DF}"
//     \u{1F0E0}\u{1F0E1}\u{1F0E2}\u{1F0E3}\u{1F0E4}\u{1F0E5}\u{1F0E6}\u{1F0E7}\u{1F0E8}\u{1F0E9}\u{1F0EA}\u{1F0EB}\u{1F0EC}\u{1F0ED}\u{1F0EE}\u{1F0EF}\u{1F0F0}\u{1F0F1}\u{1F0F2}\u{1F0F3}\u{1F0F4}\u{1F0F5}"
;

impl std::string::ToString for Card
{
    fn to_string(&self) -> String
    {
        match self.number
        {
            1 => format!("Ace of {}", self.suit),
            2...10 => format!("{} of {}", self.number, self.suit),
            11...13 => format!("{} of {}", FACES[self.number-11], self.suit),
            _ => String::new(),
        }
    }
}

/// Create a DrawCard Command
pub fn command() -> Result<Command, String>
{
    Ok(Command::DrawCard)
}

/// Draw a card from the deck
pub fn draw() -> Decision
{
    let num = rand::thread_rng().gen_range(0, 52);
    let (suit, rank) = (num / 13, (num % 13) + 1);
    Decision::Card(
        Card{ glyph: GLYPHS.chars().nth(num).unwrap(), suit: SUITS[suit], number: rank }
    )
}

/// Return an ApiDoc object containing a description of the DrawCard
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "deck",
        params: Vec::new(),
        hint: "Draw a random card from the deck",
        help: vec![
            "Draw a random card from the deck",
        ],
    }
}
