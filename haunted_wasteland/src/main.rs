use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::process::exit;
use regex::Regex;

#[derive(Debug, Eq, Hash)]
struct Node {
    name : String, 
    left : String, 
    right : String, 
    left_index : usize, 
    right_index : usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool { 
        if self.name == other.name {
            true
        } else {
            false
        }
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
    let mut nodes : Vec<Node> = Vec::new();
    let mut names : Vec<String> = Vec::new();
    let mut moves : String = String::new();
    let move_regex = Regex::new(r"^(L|R)+$").expect("Invalid regex");
    let node_regex = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").expect("Invalid regex");

    for line in reader.lines() {
        let line_text = &line.expect("Couldn't read line from file");

        if move_regex.is_match(line_text) {
            moves.push_str(line_text);
        } else if node_regex.is_match(line_text) {
            let captures = node_regex.captures(line_text).unwrap();

            let name = captures.get(1).unwrap().as_str();
            let left = captures.get(2).unwrap().as_str();
            let right = captures.get(3).unwrap().as_str();

            nodes.push(Node {name: String::from(name), left: String::from(left), right: String::from(right), left_index : 0, right_index : 0, });
            names.push(String::from(name));
        }
    }

    // println!("{}", nodes.len());

    for node in nodes.iter_mut() {

        // println!("Updating node {}\nLeft: {}\nRight: {}", node.name, node.left, node.right);

        let mut found_left = false;
        let mut found_right = false;

        for i in 0..names.len() {
            if *names.get(i).unwrap() == node.left {
                found_left = true;
                node.left_index = i;
            }

            if *names.get(i).unwrap() == node.right {
                found_right = true;
                node.right_index = i;
            }

            if found_left && found_right {
                break;
            }
        }
    }

    let mut step_nodes : Vec<usize> = Vec::new();
    let a_regex = Regex::new(r"[A-Z]{2}A").expect("invalid regex");
    let z_regex = Regex::new(r"[A-Z]{2}Z").expect("invalid regex");

    for i in 0..(nodes.len()) {
        let name = &nodes.get(i).unwrap().name;

        if a_regex.is_match(name) {
            // println!("Added {:?} to step from index {}", nodes.get(i).unwrap(), i);
            step_nodes.push(i);
        }
    }

    let mut cycles : Vec<usize> = Vec::with_capacity(step_nodes.len());
    let mut i = moves.chars();
    let mut counter : usize = 0;

    // println!("Step: {:?}", step_nodes);

    for start_index in step_nodes {
        let mut curr_node = nodes.get(start_index).unwrap();

        while !z_regex.is_match(&curr_node.name) {
            let check = i.next();
            let step;

            if check.is_some() {
                step = check.unwrap();
            } else {
                i = moves.chars();
                step = i.next().unwrap();
            }

            if step == 'R' {
                curr_node = nodes.get(curr_node.right_index).unwrap();
            } else {
                curr_node = nodes.get(curr_node.left_index).unwrap();
            }

            counter += 1;
        }

        cycles.push(counter);

        i = moves.chars();
        counter = 0;
        
        // println!("Move: {}", step);

        // println!("Step: {:?}", step_nodes);
    }

    let mut mult = *cycles.get(0).unwrap();

    for index in 1..(cycles.len()) {
        mult = lcm(*cycles.get(index).unwrap(), mult);
    }

    println!("{}", mult);
}

fn gcd (a: usize, b : usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd (b % a, a)
    }
}

fn lcm (a : usize, b : usize) -> usize {
    (a / gcd(a, b)) * b
}