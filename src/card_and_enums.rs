#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub suit: SuitEnum,
    pub number: NumberEnum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SuitEnum {
    Spade,
    Club,
    Heart,
    Diamond,
}

impl SuitEnum {
    pub fn iterator() -> impl Iterator<Item = SuitEnum> {
        const ALL_SUITS: [SuitEnum; 4] = [
            SuitEnum::Spade,
            SuitEnum::Club,
            SuitEnum::Heart,
            SuitEnum::Diamond,
        ];
        ALL_SUITS.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NumberEnum {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl NumberEnum {
    pub fn iterator() -> impl Iterator<Item = NumberEnum> {
        const ALL_NUMS: [NumberEnum; 13] = [
            NumberEnum::Ace,
            NumberEnum::Two,
            NumberEnum::Three,
            NumberEnum::Four,
            NumberEnum::Five,
            NumberEnum::Six,
            NumberEnum::Seven,
            NumberEnum::Eight,
            NumberEnum::Nine,
            NumberEnum::Ten,
            NumberEnum::Jack,
            NumberEnum::Queen,
            NumberEnum::King,
        ];
        ALL_NUMS.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn suitenum_iterator_contains_all_suits() {
        let output: Vec<SuitEnum> = SuitEnum::iterator().collect();

        assert_eq!(output.len(), 4);
        assert!(output.contains(&SuitEnum::Club));
        assert!(output.contains(&SuitEnum::Spade));
        assert!(output.contains(&SuitEnum::Diamond));
        assert!(output.contains(&SuitEnum::Heart));
    }

    #[test]
    fn numberenum_iterator_contains_all_numbers() {
        let output: Vec<NumberEnum> = NumberEnum::iterator().collect();

        assert_eq!(output.len(), 13);
        assert!(output.contains(&NumberEnum::Ace));
        assert!(output.contains(&NumberEnum::Two));
        assert!(output.contains(&NumberEnum::Three));
        assert!(output.contains(&NumberEnum::Four));
        assert!(output.contains(&NumberEnum::Five));
        assert!(output.contains(&NumberEnum::Six));
        assert!(output.contains(&NumberEnum::Seven));
        assert!(output.contains(&NumberEnum::Eight));
        assert!(output.contains(&NumberEnum::Nine));
        assert!(output.contains(&NumberEnum::Ten));
        assert!(output.contains(&NumberEnum::Jack));
        assert!(output.contains(&NumberEnum::Queen));
        assert!(output.contains(&NumberEnum::King));
    }
}
