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
        57...77 => {
            let value = num - 56; // Values from 1 - 21
            Ok(Card::Trump{ glyph: get_glyph(num), name: TRUMPS[value], number: value})
        },
        _ => Err("Invalid Trump num".to_string()),
    }
}

/// Convert a number from 0 to 77 into a Tarot Card as a Result
pub fn card(num: usize) -> Result<Card,String>
{
    match num
    {
        0...55 => minor_card(num),
        56...77 => trump_card(num),
        _ => Err(format!("{} is out of range for a valid card", num)),
    }
}

/// Randomly select a Tarot Card
pub fn draw_card<T>(rng: &mut T) -> Card
    where T: Rng
{
    let num = rng.gen_range(0, 78);
    card(num).unwrap()
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use crate::deck::tarot;
    use crate::deck::Card;

    #[test]
    fn new_cards()
    {
        assert_that!(tarot::card(0).unwrap() == Card::Pip{glyph: Some('\u{1F0A1}'), suit: "Swords", number: 1});
        assert_that!(tarot::card(14+12).unwrap() == Card::Face{glyph: Some('\u{1F0BB}'), suit: "Cups", number: 12, face: "Knight"});
        assert_that!(tarot::card(28+2).unwrap() == Card::Pip{glyph: Some('\u{1F0C3}'), suit: "Coins", number: 3});
        assert_that!(tarot::card(42+13).unwrap() == Card::Face{glyph: Some('\u{1F0DD}'), suit: "Wands", number: 13, face: "Queen"});

        assert_that!(tarot::card(54).unwrap() == Card::Joker{glyph: Some('\u{1F0E0}'), name: "The Fool"});
        assert_that!(tarot::card(63).unwrap() == Card::Trump{glyph: Some('\u{1F0E9}'), name: "The Hermit", number: 9});
        assert_that!(tarot::card(77).unwrap() == Card::Trump{glyph: Some('\u{1F0F5}'), name: "The World", number: 21});
    }

    #[test]
    fn invalid_card()
    {
        assert_that!(tarot::card(78).is_err());
    }
}