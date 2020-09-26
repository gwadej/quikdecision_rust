use super::Card;
use rand::Rng;

const DECK_SIZE:  usize = 52;
const DECK_MAX:   usize = DECK_SIZE-1;
const JDECK_SIZE: usize = 54;
const JDECK_MAX:  usize = JDECK_SIZE-1;
const SUIT_SIZE:  usize = 13;

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
pub(crate) fn card(num: usize) -> Result<Card,String>
{
    if num >= DECK_SIZE { return Err(format!("{} is out of range for a valid card", num)); }
    let (suit, rank) = (num / SUIT_SIZE, (num % SUIT_SIZE) + 1);
    let card = match rank
    {
        1..=10  => Card::Pip{ glyph: get_glyph(num), suit: SUITS[suit], number: rank },
        11..=13 => Card::Face{ glyph: get_glyph(num), suit: SUITS[suit], number: rank, face: FACES[rank-11] },
        _       => unreachable!(), // thanks to modulo above.
    };
    Ok(card)
}

pub(crate) fn cards() -> Vec<Card>
{
    (0..DECK_SIZE).map(|n| card(n).unwrap())
            .collect()
}

fn joker(num: usize) -> Result<Card,String>
{
    match num
    {
        DECK_SIZE..=JDECK_MAX => Ok(Card::Joker{ glyph: get_glyph(num), name: JOKERS[num-DECK_SIZE] }),
        _ => Err("Invalid Joker num".to_string()),
    }
}

/// Convert a number from 0 to 53 to a Card as a result
pub(crate) fn card_or_joker(num: usize) -> Result<Card,String>
{
    match num
    {
        0..=DECK_MAX => card(num),
        DECK_SIZE | JDECK_MAX => joker(num),
        _ => Err(format!("{} is out of range for a valid card", num)),
    }
}

pub(crate) fn cards_and_jokers() -> Vec<Card>
{
    (0..JDECK_SIZE).map(|n| card_or_joker(n).unwrap())
            .collect()
}

/// Randomly choose a card from a standard 52 card deck without jokers
pub(crate) fn draw_card<T>(rng: &mut T) -> Card
    where T: Rng
{
    let num = rng.gen_range(0, DECK_SIZE);
    card(num).unwrap()
}

/// Randomly choose a card from a standard 52 card deck with jokers
pub(crate) fn draw_card_or_joker<T>(rng: &mut T) -> Card
    where T: Rng
{
    let num = rng.gen_range(0, JDECK_SIZE);
    card_or_joker(num).unwrap()
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use crate::deck::standard;
    use crate::deck::Card;
    use super::*;

    #[test]
    fn new_cards()
    {
        assert_that!(standard::card(0))
            .is_ok_containing(Card::Pip{glyph: Some('\u{1F0A1}'), suit: "Spades", number: 1});
        assert_that!(standard::card(SUIT_SIZE+10))
            .is_ok_containing(Card::Face{glyph: Some('\u{1F0BB}'), suit: "Hearts", number: 11, face: "Jack"});
        assert_that!(standard::card(2*SUIT_SIZE+2))
            .is_ok_containing(Card::Pip{glyph: Some('\u{1F0C3}'), suit: "Diamonds", number: 3});
        assert_that!(standard::card(3*SUIT_SIZE+11))
            .is_ok_containing(Card::Face{glyph: Some('\u{1F0DD}'), suit: "Clubs", number: 12, face: "Queen"});
    }

    #[test]
    fn invalid_cards()
    {
        assert_that!(standard::card(DECK_SIZE))
            .is_err_containing("52 is out of range for a valid card".to_string());
        assert_that!(standard::card_or_joker(JDECK_SIZE))
            .is_err_containing("54 is out of range for a valid card".to_string());
    }

    #[test]
    fn new_jokers()
    {
        assert_that!(standard::card_or_joker(DECK_SIZE))
            .is_ok_containing(Card::Joker{glyph: Some('\u{1F0BF}'), name: "Black Joker"});
        assert_that!(standard::card_or_joker(JDECK_MAX))
            .is_ok_containing(Card::Joker{glyph: Some('\u{1F0CF}'), name: "Red Joker"});
    }

    #[test]
    fn get_deck_52()
    {
        let deck = standard::cards();
        assert_eq!(deck.len(), DECK_SIZE);
        assert_that!(deck[0])
            .is_equal_to(Card::Pip{glyph: Some('\u{1F0A1}'), suit: "Spades", number: 1});
        assert_that!(deck[DECK_SIZE-1])
            .is_equal_to(Card::Face{glyph: Some('\u{1F0DE}'), suit: "Clubs", number: 13, face: "King"});
    }

    #[test]
    fn get_deck_jokers()
    {
        let deck = standard::cards_and_jokers();
        assert_eq!(deck.len(), JDECK_SIZE);
        assert_that!(deck[0])
            .is_equal_to(Card::Pip{glyph: Some('\u{1F0A1}'), suit: "Spades", number: 1});
        assert_that!(deck[53])
            .is_equal_to(Card::Joker{glyph: Some('\u{1F0CF}'), name: "Red Joker"});
    }
}
