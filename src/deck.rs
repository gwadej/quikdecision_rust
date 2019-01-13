extern crate numerals;
use crate::Command;
use crate::Decision;
use crate::ApiDoc;

use rand::Rng;
use numerals::roman::Roman;

#[derive(Debug)]
pub enum Card
{
    Pip{glyph: char, suit: &'static str, number: usize},
    Face{glyph: char, suit: &'static str, face: &'static str},
    Joker{glyph: char, name: &'static str},
    Trump{glyph: char, name: &'static str, number: usize},
}

#[derive(Debug)]
pub enum Deck
{
    Standard52,
    Jokers,
    Tarot,
}

impl PartialEq for Deck
{
    fn eq(&self, other: &Deck) -> bool
    {
        match (self, other)
        {
            (Deck::Standard52, Deck::Standard52) => true,
            (Deck::Jokers,     Deck::Jokers)     => true,
            (Deck::Tarot,      Deck::Tarot)      => true,
            (_,_)                                => false,
        }
    }
}

impl Card
{
    pub fn glyph(&self) -> char
    {
        match self
        {
            Card::Pip{glyph, ..}   => *glyph,
            Card::Face{glyph, ..}  => *glyph,
            Card::Joker{glyph, ..} => *glyph,
            Card::Trump{glyph, ..} => *glyph,
        }
    }
}

impl std::string::ToString for Card
{
    fn to_string(&self) -> String
    {
        match self
        {
            Card::Pip{number, suit, ..} => {
                match number
                {
                    1 => format!("Ace of {}", suit),
                    2...10 => format!("{} of {}", number, suit),
                    _ => String::new(),
                }
            },
            Card::Face{suit, face, ..} => {
                format!("{} of {}", face, suit)
            },
            Card::Joker{name, ..} => {
                name.to_string()
            },
            Card::Trump{name, number, ..} => {
                format!("{:X} - {}", Roman::from(*number as i16), name)
            },
        }
    }
}

/// Create a DrawCard Command
pub fn command(deck: &str) -> Result<Command, String>
{
    match deck
    {
        "52-card"  => Ok(Command::DrawCard(Deck::Standard52)),
        "jokers"   => Ok(Command::DrawCard(Deck::Jokers)),
        "tarot"    => Ok(Command::DrawCard(Deck::Tarot)),
        _          => Err("Unrecognized deck type".to_string()),
    }
}

mod standard
{
    use super::Card;

    const SUITS: [&str; 4] = [
        "Spades",
        "Hearts",
        "Diamonds",
        "Clubs",
    ];
    const FACES: [&'static str; 3] = [ "Jack", "Queen", "King" ];
    const JOKERS: [&'static str; 3] = [ "Black Joker", "Red Joker", "White Joker" ];
    const GLYPHS: &str =
        "\u{1F0A1}\u{1F0A2}\u{1F0A3}\u{1F0A4}\u{1F0A5}\u{1F0A6}\u{1F0A7}\u{1F0A8}\u{1F0A9}\u{1F0AA}\u{1F0AB}\u{1F0AD}\u{1F0AE}\
        \u{1F0B1}\u{1F0B2}\u{1F0B3}\u{1F0B4}\u{1F0B5}\u{1F0B6}\u{1F0B7}\u{1F0B8}\u{1F0B9}\u{1F0BA}\u{1F0BB}\u{1F0BD}\u{1F0BE}\
        \u{1F0C1}\u{1F0C2}\u{1F0C3}\u{1F0C4}\u{1F0C5}\u{1F0C6}\u{1F0C7}\u{1F0C8}\u{1F0C9}\u{1F0CA}\u{1F0CB}\u{1F0CD}\u{1F0CE}\
        \u{1F0D1}\u{1F0D2}\u{1F0D3}\u{1F0D4}\u{1F0D5}\u{1F0D6}\u{1F0D7}\u{1F0D8}\u{1F0D9}\u{1F0DA}\u{1F0DB}\u{1F0DD}\u{1F0DE}\
        \u{1F0BF}\u{1F0CF}\u{1F0DF}"
    ;

    fn get_glyph(num: usize) -> char
    {
        GLYPHS.chars().nth(num).unwrap_or('\u{1F0A0}')
    }

    pub fn draw_card(num: usize) -> Result<Card,String>
    {
        let (suit, rank) = (num / 13, (num % 13) + 1);
        match rank
        {
            1...10  => Ok(Card::Pip{ glyph: get_glyph(num), suit: SUITS[suit], number: rank }),
            11...13 => Ok(Card::Face{ glyph: get_glyph(num), suit: SUITS[suit], face: FACES[rank-11] }),
            _       => Err("Can't get here!".to_string()),
        }
    }
    pub fn joker(num: usize) -> Result<Card,String>
    {
        match num
        {
            52...53 => Ok(Card::Joker{ glyph: get_glyph(num), name: JOKERS[num-52] }),
            _ => Err("Invalid Joker num".to_string()),
        }
    }
}

mod tarot
{
    use super::Card;

    const SUITS: [&str; 4] = [
        "Swords",
        "Cups",
        "Coins",
        "Wands",
    ];
    const FACES: [&'static str; 4] = [ "Jack", "Knight", "Queen", "King" ];
    const TRUMPS: [&'static str; 22] = [
        "The Fool", "The Magician", "The High Priestess", "The Empress", "The Emperor",
        "The Hierophant", "The Lovers", "The Chariot", "Justice", "The Hermit",
        "Wheel of Fortune", "Strength", "The Hanged Man", "Death", "Temperance", "The Devil",
        "The Tower", "The Star", "The Moon", "The Sun", "Judgement", "The World"
    ];
    const GLYPHS: &str =
        "\u{1F0A1}\u{1F0A2}\u{1F0A3}\u{1F0A4}\u{1F0A5}\u{1F0A6}\u{1F0A7}\u{1F0A8}\u{1F0A9}\u{1F0AA}\u{1F0AB}\u{1F0AC}\u{1F0AD}\u{1F0AE}\
        \u{1F0B1}\u{1F0B2}\u{1F0B3}\u{1F0B4}\u{1F0B5}\u{1F0B6}\u{1F0B7}\u{1F0B8}\u{1F0B9}\u{1F0BA}\u{1F0BB}\u{1F0BC}\u{1F0BD}\u{1F0BE}\
        \u{1F0C1}\u{1F0C2}\u{1F0C3}\u{1F0C4}\u{1F0C5}\u{1F0C6}\u{1F0C7}\u{1F0C8}\u{1F0C9}\u{1F0CA}\u{1F0CB}\u{1F0CC}\u{1F0CD}\u{1F0CE}\
        \u{1F0D1}\u{1F0D2}\u{1F0D3}\u{1F0D4}\u{1F0D5}\u{1F0D6}\u{1F0D7}\u{1F0D8}\u{1F0D9}\u{1F0DA}\u{1F0DB}\u{1F0DC}\u{1F0DD}\u{1F0DE}\
        \u{1F0E0}\u{1F0E1}\u{1F0E2}\u{1F0E3}\u{1F0E4}\u{1F0E5}\u{1F0E6}\u{1F0E7}\u{1F0E8}\u{1F0E9}\u{1F0EA}\u{1F0EB}\u{1F0EC}\u{1F0ED}\u{1F0EE}\u{1F0EF}\u{1F0F0}\u{1F0F1}\u{1F0F2}\u{1F0F3}\u{1F0F4}\u{1F0F5}"
    ;

    fn get_glyph(num: usize) -> char
    {
        GLYPHS.chars().nth(num).unwrap_or('\u{1F0A0}')
    }

    pub fn draw_card(num: usize) -> Result<Card,String>
    {
        let (suit, rank) = (num / 14, (num % 14) + 1);
        match rank
        {
            1...10  => Ok(Card::Pip{ glyph: get_glyph(num), suit: SUITS[suit], number: rank }),
            11...14 => Ok(Card::Face{ glyph: get_glyph(num), suit: SUITS[suit], face: FACES[rank-11] }),
            _       => Err("Can't get here!".to_string()),
        }
    }
    pub fn trump_card(num: usize) -> Result<Card,String>
    {
        match num
        {
            56 => {
                Ok(Card::Joker{ glyph: get_glyph(num), name: TRUMPS[0] })
            },
            57...78 => {
                let value = num - 56; // Values from 1 - 21
                Ok(Card::Trump{ glyph: get_glyph(num), name: TRUMPS[value], number: value})
            },
            _ => Err("Invalid Trump num".to_string()),
        }
    }
}

/// Draw a card from the deck
pub fn draw(deck: Deck) -> Decision
{
    let card = match deck
    {
        Deck::Standard52 => {
            let num = rand::thread_rng().gen_range(0, 52);
            standard::draw_card(num).unwrap()
        },
        Deck::Jokers => {
            let num = rand::thread_rng().gen_range(0, 54);
            if num >= 52
            {
                standard::joker(num).unwrap()
            }
            else
            {
                standard::draw_card(num).unwrap()
            }
        },
        Deck::Tarot => {
            let num = rand::thread_rng().gen_range(0, 78);
            if num >= 56
            {
                tarot::trump_card(num).unwrap()
            }
            else
            {
                tarot::draw_card(num).unwrap()
            }
        },
    };
    Decision::Card(card)
}

/// Return an ApiDoc object containing a description of the DrawCard
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "deck",
        params: vec![],
        hint: "Draw a random card from the specified deck",
        help: vec![
            "Draw a random card from the deck. Legal decks are :",
            "  '52-card' for the standard 52 card French deck",
            "  'jokers' for the standard deck plus 2 jokers",
            "  'tarot' for the historical Tarot deck.",
        ],
    }
}
