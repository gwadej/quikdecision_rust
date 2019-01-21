extern crate numerals;

use crate::Command;
use crate::Decision;
use crate::ApiDoc;

use numerals::roman::Roman;

/// Enum representing each of the types of cards.
/// - Card::Pip describes the numbered cards
/// - Card::Face describes the face or court cards
/// - Card::Joker describes the joker or fool cards
/// - Card::Trump describes the trump cards from a tarot deck
#[derive(Debug)]
pub enum Card
{
    Pip{glyph: Option<char>, suit: &'static str, number: usize},
    Face{glyph: Option<char>, suit: &'static str, number: usize, face: &'static str},
    Joker{glyph: Option<char>, name: &'static str},
    Trump{glyph: Option<char>, name: &'static str, number: usize},
}

/// Enum representing the supported kinds of decks
/// - Deck::Standrd52: the standard 52-card French or poker deck, without jokers
/// - Deck::Jokers: The same deck as above with 2 jokers
/// - Deck::Tarot: the historical tarot deck
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
            (_, _)                               => false,
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
            Card::Pip{suit, ..}  => *suit,
            Card::Face{suit, ..} => *suit,
            Card::Joker{..}      => "",
            Card::Trump{..}      => "Trumps",
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

mod standard;
mod tarot;

/// Draw a card from the deck
pub fn draw(deck: Deck) -> Decision
{
    let mut rng = rand::thread_rng();
    let card = match deck
    {
        Deck::Standard52 => standard::draw_card(&mut rng),
        Deck::Jokers     => standard::draw_card_or_joker(&mut rng),
        Deck::Tarot      => tarot::draw_card(&mut rng),
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

    use crate::deck;
    use crate::deck::standard;
    use crate::deck::tarot;
    use crate::deck::Deck;

    struct CardTestData
    {
        num: usize,
        display: &'static str,
        glyph: char,
        suit: &'static str,
        val: u32,
    }

    const STD_DATA: [CardTestData; 5] =
    [
        CardTestData{num: 0,     glyph: '\u{1f0a1}', val:  1, suit: "Spades", display: "Ace of Spades"},
        CardTestData{num: 13+11, glyph: '\u{1f0bb}', val: 11, suit: "Hearts", display: "Jack of Hearts"},
        CardTestData{num: 26+2,  glyph: '\u{1f0c3}', val:  3, suit: "Diamonds", display: "3 of Diamonds"},
        CardTestData{num: 39+12, glyph: '\u{1f0dd}', val: 12, suit: "Clubs", display: "Queen of Clubs"},
        CardTestData{num: 53,    glyph: '\u{1f0cf}', val:  0, suit: "", display: "Red Joker"},
    ];

    const TAROT_DATA: [CardTestData; 7] =
    [
        CardTestData{num: 0,     glyph: '\u{1f0a1}', val:  1, suit: "Spades", display: "Ace of Swords"},
        CardTestData{num: 14+12, glyph: '\u{1f0bb}', val: 11, suit: "Hearts", display: "Jack of Cups"},
        CardTestData{num: 28+2,  glyph: '\u{1f0c3}', val:  3, suit: "Diamonds", display: "3 of Coins"},
        CardTestData{num: 42+13, glyph: '\u{1f0dd}', val: 12, suit: "Clubs", display: "Queen of Wands"},
        CardTestData{num: 54,    glyph: '\u{1f0e0}', val:  0, suit: "", display: "The Fool"},
        CardTestData{num: 63,    glyph: '\u{1f0e9}', val:  9, suit: "Trumps", display: "The Hermit"},
        CardTestData{num: 77,    glyph: '\u{1f0f5}', val: 21, suit: "Trumps", display: "The World"},
    ];

    #[test]
    fn cards_to_string()
    {
        for CardTestData{num, display, ..} in STD_DATA.iter()
        {
            assert_that!(standard::card_or_joker(*num).unwrap().to_string() == display.to_string());
        }
    }

    #[test]
    fn cards_to_glyph()
    {
        for CardTestData{num, glyph, ..} in STD_DATA.iter()
        {
            assert_that!(standard::card_or_joker(*num).unwrap().glyph().unwrap() == *glyph);
        }
    }

    #[test]
    fn cards_to_suit()
    {
        for CardTestData{num, suit, ..} in STD_DATA.iter()
        {
            assert_that!(standard::card_or_joker(*num).unwrap().suit() == *suit);
        }
    }

    #[test]
    fn cards_to_value()
    {
        for CardTestData{num, val, ..} in STD_DATA.iter()
        {
            assert_that!(standard::card_or_joker(*num).unwrap().value() == *val);
        }
    }

    #[test]
    fn check_command()
    {
        assert_that!(deck::command("52-card").unwrap() == deck::Command::DrawCard(Deck::Standard52));
        assert_that!(deck::command("jokers").unwrap()  == deck::Command::DrawCard(Deck::Jokers));
        assert_that!(deck::command("tarot").unwrap()   == deck::Command::DrawCard(Deck::Tarot));
    }

    #[test]
    fn tarot_to_string()
    {
        for CardTestData{num, display, ..} in TAROT_DATA.iter()
        {
            assert_that!(tarot::card(*num).unwrap().to_string() == display.to_string());
        }
    }

    #[test]
    fn tarot_to_glyph()
    {
        for CardTestData{num, glyph, ..} in TAROT_DATA.iter()
        {
            assert_that!(tarot::card(*num).unwrap().glyph().unwrap() == *glyph);
        }
    }

    #[test]
    fn tarot_to_suit()
    {
        for CardTestData{num, suit, ..} in TAROT_DATA.iter()
        {
            assert_that!(tarot::card(*num).unwrap().suit() == *suit);
        }
    }

    #[test]
    fn tarot_to_value()
    {
        for CardTestData{num, val, ..} in TAROT_DATA.iter()
        {
            assert_that!(tarot::card(*num).unwrap().value() == *val);
        }
    }
}