use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::path::Path;
use std::str::FromStr;

#[allow(clippy::struct_field_names)]
#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    counts: HashMap<char, u8>,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (s, o) in zip(self.cards.chars(), other.cards.chars()) {
                    let cmp = cmp_card(s, o);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                Ordering::Equal
            }
            a => a,
        }
    }
}

#[derive(Debug)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut counts: HashMap<char, u8> = HashMap::new();
        s.chars().for_each(|x| {
            counts.entry(x).and_modify(|y| *y += 1).or_insert(1u8);
        });
        Ok(Self {
            cards: String::from(s),
            hand_type: HandType::get_type(&counts),
            counts,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    High = 1,
}

impl HandType {
    fn get_type(counts: &HashMap<char, u8>) -> Self {
        let max = *counts.values().max().expect("Hand empty");
        let min = *counts.values().min().expect("Hand empty");
        if let Some(&a) = counts.get(&'J') {
            match a {
                5 | 4 => Self::Five,
                3 => {
                    if min == 2 {
                        Self::Five
                    } else {
                        Self::Four
                    }
                }
                2 | 1 => {
                    let no_joker: HashMap<char, u8> = counts
                        .clone()
                        .into_iter()
                        .filter(|(x, _)| *x != 'J')
                        .collect();
                    let rem_max = *no_joker
                        .values()
                        .max()
                        .expect("Jokers were all of them, somehow");
                    let rem_min = *no_joker
                        .values()
                        .min()
                        .expect("Jokers were all of them, somehow");
                    if a == 2 {
                        if max == 3 {
                            Self::Five
                        } else if rem_max == 2 {
                            Self::Four
                        } else {
                            Self::Three
                        }
                    } else if rem_max == 4 {
                        Self::Five
                    } else if rem_max == 3 {
                        Self::Four
                    } else if rem_min == 2 {
                        Self::Full
                    } else if rem_max == 2 {
                        Self::Three
                    } else {
                        Self::OnePair
                    }
                }
                _ => panic!("How many cards you got in your hand lil bro"),
            }
        } else {
            match max {
                5 => Self::Five,
                4 => Self::Four,
                3 => {
                    if min == 2 {
                        Self::Full
                    } else {
                        Self::Three
                    }
                }
                2 => {
                    if counts.values().fold(0u8, |acc, x| acc + (*x / 2)) == 2 {
                        Self::TwoPair
                    } else {
                        Self::OnePair
                    }
                }
                1 => Self::High,
                _ => panic!("How many cards you got in your hand lil bro"),
            }
        }
    }
}

fn main() {
    let map = parse_ln(Path::new("day7.txt"));
    let total = process_cards(&map);

    println!("Total: {total}");
}

fn process_cards(map: &HashMap<String, usize>) -> usize {
    let mut total: usize = 0;
    let mut hands: Vec<Hand> = map
        .keys()
        .map(|x| Hand::from_str(&x[..]).expect("Hand malformed"))
        .collect();

    hands.sort_unstable();
    for (idx, hand) in hands.iter().enumerate() {
        total += (idx + 1)
            * map
                .get(&hand.cards)
                .expect("List of hands was broken during processing");
    }

    total
}

fn parse_ln(p: &Path) -> HashMap<String, usize> {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut map: HashMap<String, usize> = HashMap::new();
    for ln in file.lines() {
        let mut split = ln.split(' ');
        let (hand, bet) = (
            split.next().expect("Line is empty").trim(),
            split.next().expect("Line is too short").trim(),
        );
        map.insert(
            String::from(hand),
            bet.parse().expect("Integer bet malformed"),
        );
    }
    map
}

fn cmp_card(c1: char, c2: char) -> Ordering {
    let num1 = card_to_int(c1);
    let num2 = card_to_int(c2);

    num1.cmp(&num2)
}

fn card_to_int(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        n @ '2'..='9' => n.to_digit(10).expect("Char wasn't numeric apparently") as u8,
        _ => panic!("How did we get here"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn joker_hand1() {
        assert_eq!(Hand::from_str("JJJJJ").unwrap().hand_type, HandType::Five);
    }

    #[test]
    fn joker_hand2() {
        assert_eq!(Hand::from_str("JJJJA").unwrap().hand_type, HandType::Five);
    }

    #[test]
    fn joker_hand3() {
        assert_eq!(Hand::from_str("JJJAA").unwrap().hand_type, HandType::Five);
    }

    #[test]
    fn joker_hand4() {
        assert_eq!(Hand::from_str("JJJA2").unwrap().hand_type, HandType::Four);
    }

    #[test]
    fn joker_hand5() {
        assert_eq!(Hand::from_str("JJAAA").unwrap().hand_type, HandType::Five);
    }

    #[test]
    fn joker_hand6() {
        assert_eq!(Hand::from_str("JJAA2").unwrap().hand_type, HandType::Four);
    }

    #[test]
    fn joker_hand7() {
        assert_eq!(Hand::from_str("JJA23").unwrap().hand_type, HandType::Three);
    }

    #[test]
    fn joker_hand8() {
        assert_eq!(Hand::from_str("JAAAA").unwrap().hand_type, HandType::Five);
    }

    #[test]
    fn joker_hand9() {
        assert_eq!(Hand::from_str("JAAA2").unwrap().hand_type, HandType::Four);
    }

    #[test]
    fn joker_hand10() {
        assert_eq!(Hand::from_str("JAA22").unwrap().hand_type, HandType::Full);
    }

    #[test]
    fn joker_hand11() {
        assert_eq!(Hand::from_str("JAA23").unwrap().hand_type, HandType::Three);
    }

    #[test]
    fn joker_hand12() {
        assert_eq!(
            Hand::from_str("JA234").unwrap().hand_type,
            HandType::OnePair
        );
    }

    #[test]
    fn hand_type1() {
        assert_eq!(Hand::from_str("AAAAA").unwrap().hand_type, HandType::Five);
    }

    #[test]
    fn hand_type2() {
        assert_eq!(Hand::from_str("AAAA2").unwrap().hand_type, HandType::Four);
    }

    #[test]
    fn hand_type3() {
        let hand = Hand::from_str("AAA22").unwrap();
        println!("{:?}", hand.counts);
        assert_eq!(hand.hand_type, HandType::Full);
    }

    #[test]
    fn hand_type4() {
        assert_eq!(Hand::from_str("AAA23").unwrap().hand_type, HandType::Three);
    }

    #[test]
    fn hand_type5() {
        assert_eq!(
            Hand::from_str("AA223").unwrap().hand_type,
            HandType::TwoPair
        );
    }

    #[test]
    fn hand_type6() {
        assert_eq!(
            Hand::from_str("AA234").unwrap().hand_type,
            HandType::OnePair
        );
    }

    #[test]
    fn hand_type7() {
        assert_eq!(Hand::from_str("A2345").unwrap().hand_type, HandType::High);
    }

    #[test]
    fn hand_type_comp1() {
        assert!(HandType::Five > HandType::Four);
    }

    #[test]
    fn hand_type_comp2() {
        assert!(HandType::Four == HandType::Four);
    }

    #[test]
    fn hand_type_comp3() {
        assert!(HandType::Full < HandType::Four);
    }

    #[test]
    fn card_comp1() {
        assert_eq!(cmp_card('A', 'A'), Ordering::Equal);
    }

    #[test]
    fn card_comp2() {
        assert_eq!(cmp_card('K', 'A'), Ordering::Less);
    }

    #[test]
    fn card_comp3() {
        assert_eq!(cmp_card('K', 'Q'), Ordering::Greater);
    }

    #[test]
    fn card_comp4() {
        assert_eq!(cmp_card('K', '5'), Ordering::Greater);
    }

    #[test]
    fn hand_comp1() {
        assert_eq!(
            Hand::from_str("33332")
                .unwrap()
                .cmp(&Hand::from_str("2AAAA").unwrap()),
            Ordering::Greater
        );
    }

    #[test]
    fn hand_comp2() {
        assert_eq!(
            Hand::from_str("77888")
                .unwrap()
                .cmp(&Hand::from_str("77788").unwrap()),
            Ordering::Greater
        );
    }

    #[test]
    fn hand_comp3() {
        assert_eq!(
            Hand::from_str("AAAAA")
                .unwrap()
                .cmp(&Hand::from_str("A2345").unwrap()),
            Ordering::Greater
        );
    }

    #[test]
    fn full_sort() {
        let mut cards = vec![
            Hand::from_str("32T3K").unwrap(),
            Hand::from_str("T55J5").unwrap(),
            Hand::from_str("KK677").unwrap(),
            Hand::from_str("KTJJT").unwrap(),
            Hand::from_str("QQQJA").unwrap(),
        ];
        let ans = vec![
            Hand::from_str("32T3K").unwrap(),
            Hand::from_str("KK677").unwrap(),
            Hand::from_str("T55J5").unwrap(),
            Hand::from_str("QQQJA").unwrap(),
            Hand::from_str("KTJJT").unwrap(),
        ];
        cards.sort_unstable();
        assert_eq!(cards, ans);
    }

    #[test]
    fn full() {
        let map = parse_ln(Path::new("test.txt"));
        let total = process_cards(&map);

        assert_eq!(total, 5905);
    }
}
