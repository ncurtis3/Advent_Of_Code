use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::process::exit;
use std::collections::VecDeque;
use regex::Regex;

fn main() {
    let args : Vec<String> = args().collect();

    if args.len() != 2 {
        println!("usage: <fname>");
        exit(1);
    }

    let fname = &args[1];
    let reader = BufReader::new(File::open(fname).expect("Couldn't open file"));

    let mut all_text : Vec<String> = Vec::with_capacity(187);
    let mut seeds : VecDeque<(usize, usize)> = VecDeque::new();
    let mut maps : Vec<Vec<(usize, usize, usize)>> = Vec::with_capacity(7);

    for line in reader.lines() {
        let first = all_text.first_mut();
        
        if first.is_some() {
            let s = first.unwrap();
            s.push_str(&line.expect("Couldn't read line from file"));
            s.push(' ');
        } else {
            all_text.push(line.expect("Couldn't read line from file"));
        }
    }

    for _ in 0..7 {
        maps.push(Vec::new());
    }

    let mut maps_regex : Vec<Regex> = Vec::with_capacity(7);

    maps_regex.push(Regex::new(r"seed-to-soil map: \s*((?:([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*)+)").expect("Invalid regex"));
    maps_regex.push(Regex::new(r"soil-to-fertilizer map: \s*((?:([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*)+)").expect("Invalid regex"));
    maps_regex.push(Regex::new(r"fertilizer-to-water map: \s*((?:([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*)+)").expect("Invalid regex"));
    maps_regex.push(Regex::new(r"water-to-light map: \s*((?:([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*)+)").expect("Invalid regex"));
    maps_regex.push(Regex::new(r"light-to-temperature map: \s*((?:([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*)+)").expect("Invalid regex"));
    maps_regex.push(Regex::new(r"temperature-to-humidity map: \s*((?:([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*)+)").expect("Invalid regex"));
    maps_regex.push(Regex::new(r"humidity-to-location map: \s*((?:([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*)+)").expect("Invalid regex"));
    
    let seed_regex = Regex::new(r"seeds: (([0-9]+\s*[0-9]+\s*)+)").expect("Invalid regex");
    let seed_num_regex = Regex::new(r"([0-9]+)\s*([0-9]+)").expect("Invalid regex");
    let data_regex = Regex::new(r"([0-9]+)\s([0-9]+)\s([0-9]+)").expect("Invalid regex");

    let seeds_raw = seed_regex.captures(all_text.first().unwrap()).unwrap();

    for seed_nums in seed_num_regex.captures_iter(seeds_raw.get(1).unwrap().as_str()) {
        let start = seed_nums.get(1).unwrap().as_str().parse().expect("Couldn't parse seed source");
        let range = seed_nums.get(2).unwrap().as_str().parse().expect("Couldn't parse seed range");
        seeds.push_back((start, range));
    }

    for i in 0..7 {
        load_ranges(maps.get_mut(i).unwrap(), maps_regex.get(i).unwrap(), &data_regex, &all_text);
    }

    //println!("{:?}", maps);

    for i in 0..7 {
        let mut temp : VecDeque<(usize, usize)> = VecDeque::new();
        // let mut count = 0;

        // println!("-----------------------------------------------------------");
        // println!("New Iteration");

        // println!("Seeds: {:?}", seeds);

        while !seeds.is_empty() {
            // if count == 12 {
            //     println!("Seeds: {:?}", seeds);
            //     println!("Temp: {:?}", temp);
            //     exit(1);
            // }

            let (seed, seed_range) = seeds.pop_front().unwrap();
            let mut found = false;

            // println!("Seed: {}\nSeed Range: {}\n", seed, seed_range);
            // println!("Seeds: {:?}", seeds);
            // println!("Temp: {:?}", temp);

            for (source, dest, range) in maps.get(i).unwrap() {
                // println!("Source: {}\nDest: {}\nRange: {}\n", source, dest, range);

                if seed >= *source && seed + seed_range - 1 <= *source + *range { 
                    //seed is immersed in source

                    // println!("Seed: {}\nSeed Range: {}\n", seed, seed_range);
                    // println!("Seed immersed");

                    temp.push_back((*dest + (seed - *source), seed_range));
                    found = true;
                    break;
                } else if seed >= *source && seed <= *source + *range && seed + seed_range - 1 > *source + *range {
                    //seed overlaps the end of source

                    // println!("Seed: {}\nSeed Range: {}\n", seed, seed_range);
                    // println!("Overlap end");

                    temp.push_back((*dest + (seed - *source), (*range + *source) - seed));
                    seeds.push_back((*source + *range + 1, (seed_range + seed) - (*range + *source)));
                    found = true;
                    break;
                } else if seed < *source && seed + seed_range - 1 <= *source + *range && seed + seed_range - 1 >= *source {
                    //seed overlaps the beginning of source

                    // println!("Seed: {}\nSeed Range: {}\n", seed, seed_range);
                    // println!("Source: {}\nDest: {}\nRange: {}", source, dest, range);
                    // println!("Overlap beginning\n");

                    temp.push_back((*dest, (seed_range + seed) - *source));
                    seeds.push_back((seed, *source - seed));
                    found = true;
                    break;
                } else if seed < *source && seed + seed_range - 1 > *source + *range {
                    //source is immersed in seed

                    // println!("Seed: {}\nSeed Range: {}\n", seed, seed_range);
                    // println!("Source immersed");

                    seeds.push_back((seed, *source - seed));
                    temp.push_back((*dest, *range));
                    seeds.push_back((*range + *source + 1, (seed_range + seed) - *range - *source));
                    found = true;
                    break;
                }      
            }

            if !found {
                temp.push_back((seed, seed_range));
            }
        }

        while !temp.is_empty() {
            seeds.push_back(temp.pop_front().unwrap());
        }

        // let len = seeds.len();

        // for _ in 0..len {
        //     let mut found = false;
        //     let seed = seeds.pop_front().unwrap();

        //     for (source, dest, range) in maps.get(i).unwrap() {
        //         if seed >= *source && seed <= (*source) + (*range) - 1 {
        //             found = true;

        //             seeds.push_back(dest + (seed - source));
        //             break;
        //         }
        //     }

        //     if !found {
        //         seeds.push_back(seed);
        //     }
        // }
    }

    let (mut min, _) = seeds.pop_front().unwrap();

    for (seed, _) in seeds {
        if seed < min {
            min = seed;
        }
    }

    println!("{}", min);
}

fn load_ranges(vec: &mut Vec<(usize, usize, usize)>, line_regex: &Regex, data_regex: &Regex, all_text: &Vec<String>) {
    // println!("All Text First: {:?}", all_text.first());
    // println!("All Text First Unwrap: {:?}", all_text.first().unwrap());
    // println!("Captures: {:?}", line_regex.captures(all_text.first().unwrap()));
    
    let raw_data = line_regex.captures(all_text.first().unwrap()).unwrap();

    // println!("{:?}", raw_data);

    for data in data_regex.captures_iter(raw_data.get(1).unwrap().as_str()) {
        // println!("{:?}", data);

        let dest_start: usize = data.get(1).unwrap().as_str().parse().expect("Couldnt parse dest int");
        let source_start: usize = data.get(2).unwrap().as_str().parse().expect("Couldn't parse source int");
        let range: usize = data.get(3).unwrap().as_str().parse().expect("Couldn't parse range");

        // println!("{}", dest_start);
        // println!("{}", source_start);
        // println!("{}", range);

        vec.push((source_start, dest_start, range));
    }
}