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
        _       => unreachable!(), // thanks to modulo above.
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
    match num
    {
        0...51 => card(num),
        52 | 53 => joker(num),
        _ => Err(format!("{} is out of range for a valid card", num)),
    }
}

/// Randomly choose a card from a standard 52 card deck without jokers
pub fn draw_card<T>(rng: &mut T) -> Card
    where T: Rng
{
    let num = rng.gen_range(0, 52);
    card(num).unwrap()
}

/// Randomly choose a card from a standard 52 card deck with jokers
pub fn draw_card_or_joker<T>(rng: &mut T) -> Card
    where T: Rng
{
    let num = rng.gen_range(0, 54);
    card_or_joker(num).unwrap()
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use crate::deck::standard;
    use crate::deck::Card;

    #[test]
    fn new_cards()
    {
        assert_that!(standard::card(0))
            .is_ok_containing(Card::Pip{glyph: Some('\u{1F0A1}'), suit: "Spades", number: 1});
        assert_that!(standard::card(13+10))
            .is_ok_containing(Card::Face{glyph: Some('\u{1F0BB}'), suit: "Hearts", number: 11, face: "Jack"});
        assert_that!(standard::card(26+2))
            .is_ok_containing(Card::Pip{glyph: Some('\u{1F0C3}'), suit: "Diamonds", number: 3});
        assert_that!(standard::card(39+11))
            .is_ok_containing(Card::Face{glyph: Some('\u{1F0DD}'), suit: "Clubs", number: 12, face: "Queen"});
    }

    #[test]
    fn invalid_cards()
    {
        assert_that!(standard::card(52))
            .is_err_containing("52 is out of range for a valid card".to_string());
        assert_that!(standard::card_or_joker(54))
            .is_err_containing("54 is out of range for a valid card".to_string());
    }

    #[test]
    fn new_jokers()
    {
        assert_that!(standard::card_or_joker(52))
            .is_ok_containing(Card::Joker{glyph: Some('\u{1F0BF}'), name: "Black Joker"});
        assert_that!(standard::card_or_joker(53))
            .is_ok_containing(Card::Joker{glyph: Some('\u{1F0CF}'), name: "Red Joker"});
    }
}
