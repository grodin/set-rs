use enum_iterator::{all, Sequence};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum Shape {
    Shape1,
    Shape2,
    Shape3,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum Shading {
    Filled,
    Hatched,
    Unshaded,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum Number {
    One,
    Two,
    Three,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Card {
    pub colour: Colour,
    pub shape: Shape,
    pub shading: Shading,
    pub number: Number,
}

impl Card {
    fn all() -> Vec<Card> {
        let mut all_cards = Vec::with_capacity(81);
        for colour in all::<Colour>() {
            for shading in all::<Shading>() {
                for shape in all::<Shape>() {
                    for number in all::<Number>() {
                        all_cards.push(Card {
                            colour,
                            shape,
                            shading,
                            number,
                        })
                    }
                }
            }
        }
        all_cards
    }

    pub fn colour(&self) -> &Colour {
        &self.colour
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn shading(&self) -> &Shading {
        &self.shading
    }

    pub fn number(&self) -> &Number {
        &self.number
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_gives_distinct_cards() {
        let all_cards = Card::all();
        let set_of_cards: HashSet<Card> = HashSet::from_iter(all_cards.into_iter());
        assert_eq!(set_of_cards.len(), 81);
    }
}
