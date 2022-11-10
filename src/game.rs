use crate::card::{Card, Number};
use enum_iterator::{first, last};
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq)]
pub struct PossibleSet {
    card_set: HashSet<Card>,
}

impl PossibleSet {
    pub fn try_to_set(&self) -> Option<Set> {
        let properties = [
            self.count_distinct(Card::colour)?,
            self.count_distinct(Card::shape)?,
            self.count_distinct(Card::shading)?,
            self.count_distinct(Card::number)?,
        ];
        properties
            .iter()
            // Unwrapping is fine here because the enum is non-empty
            .all(|n| n == &first().unwrap() || n == &last().unwrap())
            .then(|| Set {
                card_set: self.card_set.clone(),
            })
    }

    fn count_distinct<T: Eq + Hash, F: FnMut(&Card) -> &T>(&self, selector: F) -> Option<Number> {
        self.card_set
            .iter()
            .map(selector)
            .collect::<HashSet<&T>>()
            .len()
            .try_into()
            .ok()
    }
}

impl TryFrom<[Card; 3]> for PossibleSet {
    type Error = PossibleSetCreationError;

    fn try_from(cards: [Card; 3]) -> Result<Self, Self::Error> {
        let card_set: HashSet<Card> = cards.into_iter().collect();
        if card_set.len() != 3 {
            Err(PossibleSetCreationError::NotDistinct)
        } else {
            Ok(PossibleSet { card_set })
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum PossibleSetCreationError {
    NotDistinct,
}

impl Display for PossibleSetCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for PossibleSetCreationError {}

#[derive(Debug, Eq, PartialEq)]
pub struct Set {
    card_set: HashSet<Card>,
}

impl IntoIterator for Set {
    type Item = Card;
    type IntoIter = std::collections::hash_set::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.card_set.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Colour, Number, Shading, Shape};

    #[test]
    fn possible_set_try_from_fails_for_repeated_cards() {
        let card1 = Card {
            colour: Colour::Red,
            shading: Shading::Filled,
            shape: Shape::Shape1,
            number: Number::One,
        };
        let card2 = Card {
            colour: Colour::Blue,
            ..card1
        };
        assert!(PossibleSet::try_from([card1, card1, card2]).is_err())
    }

    #[test]
    fn possible_sets_equal_independent_of_creation_order() {
        let card1 = Card {
            colour: Colour::Red,
            shading: Shading::Filled,
            shape: Shape::Shape1,
            number: Number::One,
        };
        let card2 = Card {
            colour: Colour::Blue,
            ..card1
        };
        let card3 = Card {
            colour: Colour::Green,
            ..card1
        };
        assert_eq!(
            PossibleSet::try_from([card1, card2, card3]),
            PossibleSet::try_from([card3, card1, card2])
        )
    }
}
