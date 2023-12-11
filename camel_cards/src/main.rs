use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::process::exit;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind = 7, 
    FourOfAKind = 6, 
    FullHouse = 5, 
    ThreeOfAKind = 4, 
    TwoPair = 3, 
    OnePair = 2, 
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
    hand_type: HandType, 
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering { 
        // println!("Comparing: {:?} and {:?}", self, other);
        if self.hand_type as usize == other.hand_type as usize{
            let mut i : usize = 0;

            while i < 5 {
                let char1 = self.cards.chars().nth(i).unwrap();
                let char2 = other.cards.chars().nth(i).unwrap();
                if compare_cards(char1, char2) == 1 {
                    // println!("Returning greater");
                    return Ordering::Greater;
                } else if compare_cards(char1, char2) == -1 {
                    // println!("Returning less");
                    return Ordering::Less;
                }

                i += 1;
            }

            // println!("Returning equal");
            return Ordering::Equal;
        } else if self.hand_type as usize > other.hand_type as usize {
            // println!("Returning greater");
            return Ordering::Greater;
        } else {
            // println!("Returning less");
            return Ordering::Less;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // println!("Comparing: {:?} and {:?}", self, other);
        if self.hand_type as usize == other.hand_type as usize{
            let mut i : usize = 0;

            while i < 5 {
                let char1 = self.cards.chars().nth(i).unwrap();
                let char2 = other.cards.chars().nth(i).unwrap();

                // println!("Chars at {} are {}", i, compare_cards(char1, char2));

                if compare_cards(char1, char2) == 1 {
                    // println!("Returning greater");
                    return Some(Ordering::Greater);
                } else if compare_cards(char1, char2) == -1 {
                    // println!("Returning less");
                    return Some(Ordering::Less);
                }

                i += 1;
            }

            // println!("Returning equal");
            return Some(Ordering::Equal);
        } else if self.hand_type as usize > other.hand_type as usize {
            // println!("Returning greater");
            return Some(Ordering::Greater);
        } else {
            // println!("Returning less");
            return Some(Ordering::Less);
        }
    }
}

fn compare_cards(a: char, b: char) -> isize {
    // match a {
    //     'K' => match b {'A' => -1, 'K' => 0, _ => 1, },
    //     'Q' => match b {'A' | 'K' => -1, 'Q' => 0, _ => 1, }, 
    //     'J' => match b {'A' | 'K' | 'Q' => -1, 'J' => 0, _ => 1, }, 
    //     'T' => match b {'A' | 'K' | 'Q' | 'J' => -1, 'T' => 0, _ => 1, }, 
    //     _ => match b {'A' | 'K' | 'Q' | 'J' | 'T' =>  -1, _ => if a == b {0} else if a < b {-1} else {1}, },
    // }

    // println!("{} {}", a, b);

    if a == b {
        return 0;
    }

    if a == 'A' {
        return 1;
    } else if a == 'K' {
        if b == 'A' {
            return -1;
        } else {
            return 1;
        }
    } else if a == 'Q' {
        if b == 'A' || b == 'K' {
            return -1;
        } else {
            return 1;
        }
    } else if a == 'J' {
        return -1;
    } else if a == 'T' {
        if b == 'A' || b == 'K' || b == 'Q' {
            return -1;
        } else {
            return 1;
        }
    } else {
        if b == 'A' || b == 'K' || b == 'Q' || b == 'T' {
            return -1;
        } else if a < b && b != 'J' {
            return -1;
        } else {
            return 1;
        }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool { 
        if self.cards == other.cards {
            return true;
        }

        return false;
    }
}

fn main() {
    let args : Vec<String> = args().collect();

    if args.len() != 2 {
        println!("usage: <filename>");
        exit(1);
    }

    let fname = &args[1];
    let reader = BufReader::new(File::open(fname).expect("Couldn't open file"));

    let mut hands : BinaryHeap<Hand> = BinaryHeap::with_capacity(1000);
    let regex = Regex::new(r"((?:A|K|Q|J|T|[2-9]){5}) ([0-9]+)").expect("Invalid regex");

    for line in reader.lines() {
        let line_text = &line.expect("Couldn't read line");
        let captures = regex.captures(line_text).unwrap();
        let cards = captures.get(1).unwrap().as_str();

        hands.push(Hand { cards: String::from(cards), 
                        bid: captures.get(2).unwrap().as_str().parse().expect("Couldn't parse bid"), 
                        hand_type: get_type(cards), } );
    }

    //println!("{:?}\n", hands);

    let mut sum = 0;
    let length = hands.len();

    for i in (1..=length).rev() {
        let hand = hands.pop().unwrap();
        // println!("{:?}", hand);
        sum += i * hand.bid;
    }

    println!("{}", sum);
}

fn get_type (cards: &str) -> HandType{
    let mut map : HashMap<char, usize> = HashMap::new();

    for character in cards.chars() {
        if map.contains_key(&character) {
            map.insert(character, map.get(&character).unwrap() + 1);
        } else {
            map.insert(character, 1);
        }
    }

    if map.contains_key(&'J') {
        if *map.get(&'J').unwrap() == 5 {
            return HandType::FiveOfAKind;
        }

        let (mut key, mut value) = ('J', 0);
        let add = map.remove(&'J').unwrap();

        for (k, v) in &map {
            if *v > value {
                key = *k;
                value = *v;
            }
        }

        map.insert(key, value + add);
    }

    if map.len() == 1 {
        HandType::FiveOfAKind
    } else if map.len() == 2 {
        let mut ret = HandType::FullHouse;

        for (_, val) in map {
            if val == 2 || val == 3{
                ret = HandType::FullHouse;
            } else {
                ret = HandType::FourOfAKind;
            }
        }

        ret
    } else if map.len() == 3 {
        let mut ret = HandType::TwoPair;

        for (_, val) in map {
            if val == 2 {
                ret = HandType::TwoPair;
            } else if val == 3 {
                ret = HandType::ThreeOfAKind;
            }
        }

        ret
    } else if map.len() == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
 }
