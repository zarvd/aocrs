use std::cmp::Ordering;
use std::collections::HashMap;

const FIVE_OF_A_KIND: u8 = 7;
const FOUR_OF_A_KIND: u8 = 6;
const FULL_HOUSE: u8 = 5;
const THREE_OF_A_KIND: u8 = 4;
const TWO_PAIR: u8 = 3;
const ONE_PAIR: u8 = 2;
const HIGH_CARD: u8 = 1;

struct Game {
    with_joker: bool,
    cards: HashMap<char, usize>,
}

impl Game {
    pub fn new(with_joker: bool) -> Game {
        let cards = if with_joker {
            [
                'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
            ]
        } else {
            [
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ]
        }
        .into_iter()
        .enumerate()
        .map(|(i, card)| (card, i))
        .collect::<HashMap<_, _>>();
        Game { with_joker, cards }
    }

    pub fn kind_of_hand(&self, cards: &str) -> u8 {
        let mut jokers = 0;
        let m = cards
            .as_bytes()
            .into_iter()
            .fold(HashMap::<u8, usize>::new(), |mut acc, &elem| {
                *acc.entry(elem).or_default() += 1;
                if elem == b'J' {
                    jokers += 1;
                }
                acc
            });
        let mut has_one_pair = false;
        let mut has_two_pair = false;
        let mut has_three_kind = false;
        for cnt in m.into_values() {
            match cnt {
                5 => return FIVE_OF_A_KIND,
                4 if self.with_joker && jokers > 0 => return FIVE_OF_A_KIND,
                4 => return FOUR_OF_A_KIND,
                3 => has_three_kind = true,
                2 if has_one_pair => has_two_pair = true,
                2 => has_one_pair = true,
                _ => {}
            }
        }
        if self.with_joker {
            if has_two_pair {
                return if jokers == 1 {
                    FULL_HOUSE
                } else if jokers == 2 {
                    FOUR_OF_A_KIND
                } else {
                    TWO_PAIR
                };
            }
            if has_three_kind {
                if has_one_pair {
                    return if jokers > 0 {
                        FIVE_OF_A_KIND
                    } else {
                        FULL_HOUSE
                    };
                }
                return if jokers > 0 {
                    FOUR_OF_A_KIND
                } else {
                    THREE_OF_A_KIND
                };
            }
            if has_one_pair {
                return if jokers > 0 {
                    THREE_OF_A_KIND
                } else {
                    ONE_PAIR
                };
            }
            return if jokers > 0 { ONE_PAIR } else { HIGH_CARD };
        } else {
            if has_two_pair {
                return TWO_PAIR;
            }
            if has_three_kind {
                return if has_one_pair {
                    FULL_HOUSE
                } else {
                    THREE_OF_A_KIND
                };
            }
            if has_one_pair {
                return ONE_PAIR;
            }
        }

        HIGH_CARD
    }

    #[inline(always)]
    pub fn power_of_card(&self, card: char) -> usize {
        *self
            .cards
            .get(&card)
            .expect(&format!("{card} must be valid"))
    }
}

pub fn solve_part1(input: String) -> String {
    part1(normalize_input(input)).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(normalize_input(input)).to_string()
}

fn normalize_input(s: String) -> Vec<(String, u64)> {
    s.split('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(cards, bid)| (cards.to_owned(), bid.trim().parse::<u64>().unwrap()))
        .collect::<Vec<_>>()
}

fn play(game: Game, hands: Vec<(String, u64)>) -> u64 {
    let mut hands = hands
        .into_iter()
        .map(|(cards, bid)| {
            let kind = game.kind_of_hand(&cards);
            (cards, kind, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|l, r| {
        if l.1 < r.1 {
            return Ordering::Less;
        } else if l.1 > r.1 {
            return Ordering::Greater;
        }
        let (lb, rb) = (l.0.as_bytes(), r.0.as_bytes());
        for i in 0..5 {
            if lb[i] == rb[i] {
                continue;
            }
            let lp = game.power_of_card(lb[i] as char);
            let rp = game.power_of_card(rb[i] as char);
            return lp.cmp(&rp);
        }
        Ordering::Equal
    });
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (rank, (_, _, bid))| acc + (rank as u64 + 1) * bid)
}

fn part1(hands: Vec<(String, u64)>) -> u64 {
    let game = Game::new(false);
    play(game, hands)
}

fn part2(hands: Vec<(String, u64)>) -> u64 {
    let game = Game::new(true);
    play(game, hands)
}
