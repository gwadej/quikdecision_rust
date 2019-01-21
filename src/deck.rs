extern crate numerals;

use crate::Command;
use crate::Decision;
use crate::ApiDoc;

use numerals::roman::Roman;

#[derive(Debug)]
pub enum Card
{
    Pip{glyph: Option<char>, suit: &'static str, number: usize},
    Face{glyph: Option<char>, suit: &'static str, number: usize, face: &'static str},
    Joker{glyph: Option<char>, name: &'static str},
    Trump{glyph: Option<char>, name: &'static str, number: usize},
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
    /// Return an Option containing the Unicode glyph associated with the card
    pub fn glyph(&self) -> Option<char>
    {
        match self
        {
            Card::Pip{glyph, ..}   => *glyph,
            Card::Face{glyph, ..}  => *glyph,
            Card::Joker{glyph, ..} => *glyph,
            Card::Trump{glyph, ..} => *glyph,
        }
    }

    /// Return the Card's suit. Jokers have no suit and Trump cards have a
    /// synthetic suit of "Trumps".
    pub fn suit(&self) -> &'static str
    {
        match self
        {
            Card::Pip{suit, ..}   => *suit,
            Card::Face{suit, ..}  => *suit,
            Card::Joker{..}       => "",
            Card::Trump{..}       => "Trumps",
        }
    }

    /// Return a value for the card. For numeric cards, return the number. For
    /// Face cards return a number greater than 10 that matches the order of the faces.
    /// Trump cards return their number values.
    /// Jokers return 0
    pub fn value(&self) -> u32
    {
        match self
        {
            Card::Pip{number, ..}   => *number as u32,
            Card::Face{number, ..}  => *number as u32,
            Card::Joker{..}         => 0,
            Card::Trump{number, ..} => *number as u32,
        }
    }
}

impl PartialEq for Card
{
    fn eq(&self, other: &Card) -> bool
    {
        match(self, other)
        {
            (&Card::Pip{glyph: lg, suit: ls, number: ln},
             &Card::Pip{glyph: rg, suit: rs, number: rn}) => lg == rg && ls == rs && ln == rn,
            (&Card::Face{glyph: lg, suit: ls, number: ln, face: lf},
             &Card::Face{glyph: rg, suit: rs, number: rn, face: rf}) => lg == rg && ls == rs && ln == rn && lf == rf,
            (&Card::Joker{glyph: lg, name: ln},
             &Card::Joker{glyph: rg, name: rn}) => lg == rg && ln == rn,
            (&Card::Trump{glyph: lg, name: ln, number: lv},
             &Card::Trump{glyph: rg, name: rn, number: rv}) => lg == rg && ln == rn && lv == rv,
             (_, _) => false,
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
                    1      => format!("Ace of {}", suit),
                    2...10 => format!("{} of {}", number, suit),
                    _      => panic!(format!("{} is not a valid card rank", number)),
                }
            },
            Card::Face{suit, face, ..}    => format!("{} of {}", face, suit),
            Card::Joker{name, ..}         => name.to_string(),
            Card::Trump{name, number, ..} => format!("{:X} - {}", Roman::from(*number as i16), name),
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
    use rand::Rng;

    const SUITS:  [&str; 4] = [ "Spades", "Hearts", "Diamonds", "Clubs" ];
    const FACES:  [&str; 3] = [ "Jack", "Queen", "King" ];
    const JOKERS: [&str; 3] = [ "Black Joker", "Red Joker", "White Joker" ];
    const GLYPHS: &str =
        "\u{1F0A1}\u{1F0A2}\u{1F0A3}\u{1F0A4}\u{1F0A5}\u{1F0A6}\u{1F0A7}\u{1F0A8}\u{1F0A9}\u{1F0AA}\u{1F0AB}\u{1F0AD}\u{1F0AE}\
        \u{1F0B1}\u{1F0B2}\u{1F0B3}\u{1F0B4}\u{1F0B5}\u{1F0B6}\u{1F0B7}\u{1F0B8}\u{1F0B9}\u{1F0BA}\u{1F0BB}\u{1F0BD}\u{1F0BE}\
        \u{1F0C1}\u{1F0C2}\u{1F0C3}\u{1F0C4}\u{1F0C5}\u{1F0C6}\u{1F0C7}\u{1F0C8}\u{1F0C9}\u{1F0CA}\u{1F0CB}\u{1F0CD}\u{1F0CE}\
        \u{1F0D1}\u{1F0D2}\u{1F0D3}\u{1F0D4}\u{1F0D5}\u{1F0D6}\u{1F0D7}\u{1F0D8}\u{1F0D9}\u{1F0DA}\u{1F0DB}\u{1F0DD}\u{1F0DE}\
        \u{1F0BF}\u{1F0CF}\u{1F0DF}";

    fn get_glyph(num: usize) -> Option<char>
    {
        GLYPHS.chars().nth(num)
    }

    /// Convert a number from 0 to 51 to a Card as a result
    pub fn card(num: usize) -> Result<Card,String>
    {
        if num >= 52 { return Err(format!("{} is out of range for a valid card", num)); }
        let (suit, rank) = (num / 13, (num % 13) + 1);
        match rank
        {
            1...10  => Ok(Card::Pip{ glyph: get_glyph(num), suit: SUITS[suit], number: rank }),
            11...13 => Ok(Card::Face{ glyph: get_glyph(num), suit: SUITS[suit], number: rank, face: FACES[rank-11] }),
            _       => Err(format!("{} is not a valid card rank", rank)),
        }
    }

    fn joker(num: usize) -> Result<Card,String>
    {
        match num
        {
            52...53 => Ok(Card::Joker{ glyph: get_glyph(num), name: JOKERS[num-52] }),
            _ => Err("Invalid Joker num".to_string()),
        }
    }

    /// Convert a number from 0 to 53 to a Card as a result
    pub fn card_or_joker(num: usize) -> Result<Card,String>
    {
        if num >= 54 { return Err(format!("{} is out of range for a valid card", num)); }
        if num < 52 { return card(num) }
        joker(num)
    }

    /// Randomly choose a card from a standard 52 card deck without jokers
    pub fn draw_card() -> Card
    {
        let num = rand::thread_rng().gen_range(0, 52);
        card(num).unwrap()
    }

    /// Randomly choose a card from a standard 52 card deck with jokers
    pub fn draw_card_or_joker() -> Card
    {
        let num = rand::thread_rng().gen_range(0, 54);
        card_or_joker(num).unwrap()
    }
}

mod tarot
{
    use super::Card;
    use rand::Rng;

    const SUITS:  [&str; 4] = [ "Swords", "Cups", "Coins", "Wands" ];
    const FACES:  [&str; 4] = [ "Jack", "Knight", "Queen", "King" ];
    const TRUMPS: [&str; 22] = [
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

    fn get_glyph(num: usize) -> Option<char>
    {
        GLYPHS.chars().nth(num)
    }

    fn minor_card(num: usize) -> Result<Card,String>
    {
        let (suit, rank) = (num / 14, (num % 14) + 1);
        match rank
        {
            1...10  => Ok(Card::Pip{ glyph: get_glyph(num), suit: SUITS[suit], number: rank }),
            11...14 => Ok(Card::Face{ glyph: get_glyph(num), suit: SUITS[suit], number: rank, face: FACES[rank-11] }),
            _       => Err("Can't get here!".to_string()),
        }
    }

    fn trump_card(num: usize) -> Result<Card,String>
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

    /// Convert a number from 0 to 77 into a Tarot Card as a Result
    pub fn card(num: usize) -> Result<Card,String>
    {
        if num < 56 { return minor_card(num) }
        trump_card(num)
    }

    /// Randomly select a Tarot Card
    pub fn draw_card() -> Card
    {
        let num = rand::thread_rng().gen_range(0, 78);
        card(num).unwrap()
    }
}

/// Draw a card from the deck
pub fn draw(deck: Deck) -> Decision
{
    let card = match deck
    {
        Deck::Standard52 => standard::draw_card(),
        Deck::Jokers     => standard::draw_card_or_joker(),
        Deck::Tarot      => tarot::draw_card(),
    };
    Decision::Card(card)
}

/// Return an ApiDoc object containing a description of the DrawCard
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "deck",
        params: vec!["type"],
        hint: "Draw a random card from the specified deck",
        help: vec![
            "Draw a random card from the deck. Legal deck types are :",
            "  '52-card' for the standard 52 card French deck",
            "  'jokers' for the standard deck plus 2 jokers",
            "  'tarot' for the historical Tarot deck.",
        ],
    }
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use crate::deck::standard;
    use crate::deck::tarot;
    use crate::deck::Card;

    #[test]
    fn new_standard_cards()
    {
        assert_that!(standard::card(0).unwrap() == Card::Pip{glyph: Some('\u{1F0A1}'), suit: "Spades", number: 1});
        assert_that!(standard::card(13+11).unwrap() == Card::Face{glyph: Some('\u{1F0BB}'), suit: "Hearts", number: 11, face: "Jack"});
        assert_that!(standard::card(26+2).unwrap() == Card::Pip{glyph: Some('\u{1F0C3}'), suit: "Diamonds", number: 3});
        assert_that!(standard::card(39+12).unwrap() == Card::Face{glyph: Some('\u{1F0DD}'), suit: "Clubs", number: 13, face: "Queen"});
    }

    #[test]
    fn cards_to_string()
    {
        assert_that!(standard::card(0).unwrap().to_string() == "Ace of Spades".to_string());
        assert_that!(standard::card(13+11).unwrap().to_string() == "Jack of Hearts".to_string());
        assert_that!(standard::card(26+2).unwrap().to_string() == "3 of Diamonds".to_string());
        assert_that!(standard::card(39+12).unwrap().to_string() == "Queen of Clubs".to_string());
    }

    #[test]
    fn invalid_standard_cards()
    {
        assert_that!(standard::card(52).is_err());
        assert_that!(standard::card_or_joker(54).is_err());
    }

    #[test]
    fn new_jokers()
    {
        assert_that!(standard::card_or_joker(52).unwrap() == Card::Joker{glyph: Some('\u{1F0BF}'), name: "Black Joker"});
        assert_that!(standard::card_or_joker(53).unwrap() == Card::Joker{glyph: Some('\u{1F0CF}'), name: "Red Joker"});
    }

    #[test]
    fn joker_to_string()
    {
        assert_that!(standard::card_or_joker(53).unwrap().to_string() == "Red Joker".to_string());
    }

    #[test]
    fn new_tarot_cards()
    {
        assert_that!(tarot::card(0).unwrap() == Card::Pip{glyph: Some('\u{1F0A1}'), suit: "Swords", number: 1});
        assert_that!(tarot::card(14+12).unwrap() == Card::Face{glyph: Some('\u{1F0BB}'), suit: "Cups", number: 12, face: "Knight"});
        assert_that!(tarot::card(28+2).unwrap() == Card::Pip{glyph: Some('\u{1F0C3}'), suit: "Coins", number: 3});
        assert_that!(tarot::card(42+13).unwrap() == Card::Face{glyph: Some('\u{1F0DD}'), suit: "Wands", number: 13, face: "Queen"});

        assert_that!(tarot::card(54).unwrap() == Card::Joker{glyph: Some('\u{1F0E0}'), name: "The Fool"});
        assert_that!(tarot::card(63).unwrap() == Card::Trump{glyph: Some('\u{1F0E9}'), name: "The Hermit", number: 9});
    }

    #[test]
    fn tarot_cards_to_string()
    {
        assert_that!(tarot::card(0).unwrap().to_string() == "Ace of Swords".to_string());
        assert_that!(tarot::card(14+12).unwrap().to_string() == "Knight of Cups".to_string());
        assert_that!(tarot::card(28+2).unwrap().to_string() == "3 of Coins".to_string());
        assert_that!(tarot::card(42+13).unwrap().to_string() == "Queen of Wands".to_string());

        assert_that!(tarot::card(54).unwrap().to_string() == "The Fool".to_string());
        assert_that!(tarot::card(63).unwrap().to_string() == "IX: The Hermit".to_string());
    }

//    #[test]
//    fn invalid_standard_cards()
//    {
//        assert_that!(standard::card(52).is_err());
//        assert_that!(standard::card_or_joker(54).is_err());
//    }
}
