use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::process::exit;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let args : Vec<String> = args().collect();
    if args.len() != 2 {
        println!("usage: {} <filename>", args[0]);
        exit(1);
    }

    let fname = &args[1];
    let file = File::open(fname)
                    .expect("Couldn't open file: {fname}");
    let reader = BufReader::new(file);
    let mut unprocessed : Vec<String> = Vec::new();

    for line in reader.lines() {
        unprocessed.push(line.expect("Couldn't read a string"));
    }

    let mut map : HashMap<usize, usize> = HashMap::new();
    let card_regex = Regex::new(r"Card\s*([0-9]+)").expect("Invalid regex");

    for card in &unprocessed {
        let mid_process = card_regex.captures(&card).unwrap();

        let card_num : usize = mid_process.get(1).unwrap().as_str().parse().expect("Couldn't read card number");
        map.insert(card_num, 1);
    }

    let process = Regex::new(r"Card\s*([0-9]+):\s*((?:[0-9]+\s*)+)\|\s*((?:[0-9]+\s*)+)").expect("Invalid regex");
    let num_regex = Regex::new(r"[0-9]+").expect("Invalid regex");
    for card in &unprocessed {
        let check = process.captures(&card);
    
        if check.is_some() {
            let processing = check.unwrap();
    
            let card_num : usize= processing.get(1).unwrap().as_str().parse().expect("Couldn't read an int");

            println!("Card {}", card_num);

                let winners : &str = processing.get(2).unwrap().as_str();
                let personals : &str = processing.get(3).unwrap().as_str();
                let mut winning : Vec<usize> = Vec::with_capacity(10);
                let mut processed : Vec<usize> = Vec::with_capacity(25);
    
                let winner_subs = num_regex.find_iter(winners);
    
                for winner_num in winner_subs {
                    winning.push(winner_num.as_str().parse::<usize>().expect("Couldn't read an int from winners list"));
                }
    
                let personal_subs = num_regex.find_iter(personals);
    
                for personal_num in personal_subs {
                    processed.push(personal_num.as_str().parse::<usize>().expect("Couldn't read an int from personal nums"));
                }
    
                let mut num_wins = 0;
    
                for personal_num in &processed {
                    if winning.contains(&personal_num) {
                        num_wins += 1;
                    }
                }

                for _ in 0..*map.get(&card_num).unwrap() {
                    for i in 1..=num_wins {
                        map.insert(card_num + i, map.get(&(card_num + i)).unwrap() + 1);
                    }
                }
            
        }
    }
    let mut sum : usize = 0;

    for (_, val) in map.iter() {
        sum += val;
    }

    println!("{sum}")
}
