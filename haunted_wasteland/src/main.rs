use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::process::exit;
use regex::Regex;

#[derive(Eq, Hash)]
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

    let mut iterator = names.iter_mut();

    let mut curr_node = nodes.get(iterator.position(|x| x == &"AAA").unwrap()).unwrap();
    let mut i = moves.chars();
    let mut counter = 0;

    while curr_node.name != "ZZZ" {
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

    println!("{}", counter);
}
