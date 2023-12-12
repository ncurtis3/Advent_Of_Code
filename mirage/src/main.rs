use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::process::exit;
use std::collections::VecDeque;
use regex::Regex;

fn main() {
    let args : Vec<String> = args().collect();

    if args.len() != 2 {
        println!("usage: <filename>");
        exit(1);
    }

    let reader = BufReader::new(File::open(&args[1]).expect("Couldn't open file"));
    let num_regex = Regex::new(r"-?[0-9]+").expect("Invalid regex");
    let mut results : Vec<isize> = Vec::new();

    for line in reader.lines() {
        let mut history : Vec<VecDeque<isize>> = Vec::new();

        let mut first : VecDeque<isize> = VecDeque::new();

        for num_match in num_regex.find_iter(&line.expect("Couldn't read line")) {
            first.push_back(num_match.as_str().parse().expect("Couldn't parse int"));
        }

        history.push(first);

        let mut i = 0;

        while i < history.len() {
            let pred = history.get(i).unwrap();

            let mut add_vec : VecDeque<isize> = VecDeque::with_capacity(pred.len());
            let mut num1 = pred.get(0).unwrap();
            let mut all_zero = true;

            for i in 1..pred.len() {
                let num2 = pred.get(i).unwrap();
                add_vec.push_back(num2 - num1);

                if num2 - num1 != 0 {
                    all_zero = false;
                }

                num1 = num2;
            }

            history.push(add_vec);

            if all_zero {
                i += 1;
                break;
            }

            i += 1;
        }

        // println!("{:?}", history);
        // println!("{:?}", history.get_mut(i).unwrap());

        // println!("{:?}", lasts);
        // println!("{}", lasts.len());

        let mut lasts : Vec<isize> = Vec::new();

        history.get_mut(i).unwrap().push_front(0);
        lasts.push(0);

        for index in (0..i).rev() {
            let curr : &mut VecDeque<isize> = history.get_mut(index).unwrap();
            curr.push_front(*curr.front().unwrap() - lasts.last().unwrap());
            lasts.push(*curr.front().unwrap());
        }

        results.push(*history.get(0).unwrap().front().unwrap());

        // println!("{:?}", history);

        // println!("{:?}", results);
    }

    let mut sum = 0;

    for result in results {
        sum += result;
    }

    println!("{}", sum);
}
