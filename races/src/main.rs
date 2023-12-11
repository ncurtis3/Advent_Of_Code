use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::process::exit;
use regex::Regex;

fn main() {
    let args : Vec<String> = args().collect();

    if args.len() != 2 {
        println!("usage: <filename>");
        exit(1);
    }

    let fname = &args[1];
    let reader = BufReader::new(File::open(fname).expect("Couldn't open file"));

    let mut raw_text : Vec<String> = Vec::new();
    let mut times : Vec<usize> = Vec::with_capacity(4);
    let mut records : Vec<usize> = Vec::with_capacity(4);

    for line in reader.lines() {
        raw_text.push(line.expect("Couldn't read line from file"));
    }

    let num_regex = Regex::new(r"[0-9]+").unwrap();

    for num_string in num_regex.find_iter(raw_text.get(0).unwrap()) {
        times.push(num_string.as_str().parse().expect("Couldn't read num for times"));
    }

    for num_string in num_regex.find_iter(raw_text.get(1).unwrap()) {
        records.push(num_string.as_str().parse().expect("Couldn't read num for record"));
    }

    println!("{:?}", times);
    println!("{:?}", records);

    let mut num_ways : Vec<usize> = Vec::with_capacity(4);

    for i in 0..times.len() {
        let time = times.get(i).unwrap();
        let record = records.get(i).unwrap();

        let min = min_time(*time, 0, *time, *record);
        let max = max_time(*time, 0, *time, *record);

        num_ways.push(max - min + 1);
    }

    let mut mult = *num_ways.first().unwrap();

    for i in 1..num_ways.len() {
        mult *= *num_ways.get(i).unwrap();
    }

    println!("{}", mult);
}

fn min_time(high : usize, low : usize, time : usize, record : usize) -> usize {
    // println!("Min_time");
    // println!("High: {}\nLow: {}\n", high, low);

    if high < low {
        return usize::MAX;
    }

    if high == low {
        if (time - low) * low > record {
            return low;
        } else {
            return usize::MAX;
        }
    }

    let num1 =  low + ((high - low) / 4);
    let num2 = low + ((high - low) / 2);
    let num3 = low + ((3 * (high - low)) / 4);

    let dist1 = (time - num1) * num1;
    let dist2 = (time - num2) * num2;
    let dist3 = (time - num3) * num3;

    let pass1 = dist1 > record;
    let pass2 = dist2 > record;
    let pass3 = dist3 > record;

    // println!("Pass1: {}\nPass2: {}\nPass3: {}", pass1, pass2, pass3);

    if !pass1 && !pass2 && !pass3 {
        let check1 = min_time(high, num3 + 1, time, record);
        let check2 = min_time(num1, low, time, record);

        if check1 < check2 {
            return check1;
        } else {
            return check2;
        }
    } else if pass1 {
        return min_time(num1, low, time, record);
    } else if !pass1 && pass2 {
        return min_time(num2, num1, time, record);
    } else {
        return min_time(num3, num2, time, record);
    }
}

fn max_time(high : usize, low : usize, time : usize, record : usize) -> usize {
    // println!("Max_time");
    // println!("High: {}\nLow: {}\n", high, low);

    if high < low {
        return 0;
    }

    if high == low {
        if (time - low) * low > record {
            return low;
        } else {
            return 0;
        }
    }

    let num1 =  low + ((high - low) / 4);
    let num2 = low + ((high - low) / 2);
    let num3 = low + ((3 * (high - low)) / 4);

    let dist1 = (time - num1) * num1;
    let dist2 = (time - num2) * num2;
    let dist3 = (time - num3) * num3;

    let pass1 = dist1 > record;
    let pass2 = dist2 > record;
    let pass3 = dist3 > record;

    // println!("Pass1: {}\nPass2: {}\nPass3: {}", pass1, pass2, pass3);

    if !pass1 && !pass2 && !pass3 {
        let check1 = max_time(high, num3 + 1, time, record);
        let check2 = max_time(num1, low, time, record);

        if check1 > check2 {
            return check1;
        } else {
            return check2;
        }
    } else if pass3 {
        let check = max_time(high, num3 + 1, time, record);

        if check > num3 {
            return check;
        } else {
            return num3;
        }
    } else if !pass3 && pass2 {
        return max_time(num3, num2, time, record);
    } else {
        return max_time(num2, num1, time, record);
    }
}