// --- Day 8: Haunted Wasteland ---
// You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

// One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

// It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

// After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

// This format defines each node of the network individually. For example:

// RL

// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
// Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

// Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

// LLR

// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
// Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum TypeRun {
    FirstPart,
    SecondPart
}

fn main() -> std::io::Result<()> {
    algorithm(TypeRun::FirstPart)?;

    Ok(())
}

fn algorithm(type_run: TypeRun) -> std::io::Result<()> {

    let file:File = File::open("data/input.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut network_data: NetworkData = NetworkData::new();
    let re = Regex::new(r"^[A-Z]{3} = \([A-Z]{3}, [A-Z]{3}\)$").unwrap();

    for (i,line) in reader.lines().enumerate() {

        let mut line_str = line?;

        if i == 0 {
            network_data.init_directions(&line_str);
        }
        else {
            if re.is_match(&line_str){
                network_data.add_entry_to_network(&mut line_str);
            }
        }
    }

    let num_steps: u64 = network_data.get_number_steps();
    println!("The number of steps to reach the destination for {:?} is: {}", type_run, num_steps);

    Ok(())
}

#[derive(Debug)]
struct NetworkData {
    init_location: String,
    goal_location: String,
    directions: Vec<char>,
    network_map: HashMap<String, (String, String)>,
}

impl NetworkData {
    
    fn new() -> Self {
        NetworkData{
            init_location: "AAA".to_string(),
            goal_location: "ZZZ".to_string(),
            directions: Vec::new(),
            network_map: HashMap::new(),
        }
    }

    fn init_directions(& mut self, directions_str:&str) {
        self.directions.extend(directions_str.chars());
    }

    fn add_entry_to_network(& mut self, entry_str: & mut str) {
        let entry_str = entry_str.replace(" ", "");
        let data: Vec<&str> = entry_str.split('=').collect();

        if data.len() == 2 {
            let value:&Vec<&str> = &data[1][1..=data[1].len()-2].split(',').collect();

            if !self.network_map.contains_key(data[0]) && (value.len() == 2) {
                self.network_map.insert(data[0].to_string(), (value[0].to_string(), value[1].to_string()));
            }
            else {
                println!("Something wrong with entry: {entry_str}");
            }
        }
    }

    fn get_number_steps(&self) -> u64{
        let mut number_steps: u64 = 0;
        let mut directions_index: usize = 0;
        let mut current_location: &String = &self.init_location;

        while current_location != &self.goal_location {
            
            match self.network_map.get(current_location) {
                Some(x) => current_location = 
                    if self.directions[directions_index] == 'L' {&x.0} else {&x.1},
                None => println!("Not data found for: {current_location}"),
            }

            directions_index = (directions_index + 1) % self.directions.len();
            number_steps += 1;
        }
        number_steps
    }
}