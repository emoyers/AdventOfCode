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

// --- Part Two ---
// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

// What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

// After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

// For example:

// LR

// 11A = (11B, XXX)
// 11B = (XXX, 11Z)
// 11Z = (11B, XXX)
// 22A = (22B, XXX)
// 22B = (22C, 22C)
// 22C = (22Z, 22Z)
// 22Z = (22B, 22B)
// XXX = (XXX, XXX)
// Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

// Step 0: You are at 11A and 22A.
// Step 1: You choose all of the left paths, leading you to 11B and 22B.
// Step 2: You choose all of the right paths, leading you to 11Z and 22C.
// Step 3: You choose all of the left paths, leading you to 11B and 22Z.
// Step 4: You choose all of the right paths, leading you to 11Z and 22B.
// Step 5: You choose all of the left paths, leading you to 11B and 22C.
// Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
// So, in this example, you end up entirely on nodes that end in Z after 6 steps.

// Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?

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

    // init the locations and destinations
    network_data.init_single_location_destination();

    let num_steps: u64 = network_data.get_number_steps();
    println!("The number of steps to reach the destination for {:?} is: {}", type_run, num_steps);

    Ok(())
}

#[derive(Debug)]
struct NetworkData {
    init_locations: Vec<String>,
    goal_locations: Vec<String>,
    current_locations: Vec<String>,
    directions: Vec<char>,
    network_map: HashMap<String, (String, String)>,
}

impl NetworkData {
    
    fn new() -> Self {
        NetworkData{
            init_locations: Vec::new(),
            goal_locations: Vec::new(),
            current_locations: Vec::new(),
            directions: Vec::new(),
            network_map: HashMap::new(),
        }
    }

    fn init_single_location_destination(& mut self) {
        self.init_locations.push("AAA".to_string());
        self.goal_locations.push("ZZZ".to_string());
        
        for loc in &self.init_locations {
            self.current_locations.push(loc.clone());
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

    fn get_number_steps(& mut self) -> u64{
        let mut number_steps: u64 = 0;
        let mut directions_index: usize = 0;
        let mut continue_flag: bool = true;


        while continue_flag {
            

            for cur_loc in self.current_locations.iter_mut(){

                match self.network_map.get(cur_loc) {
                    Some(x) => *cur_loc = 
                        if self.directions[directions_index] == 'L' {x.0.clone()} else {x.1.clone()},
                    None => println!("Not data found for: {}", cur_loc),
                }

            }
            
            directions_index = (directions_index + 1) % self.directions.len();
            number_steps += 1;

            continue_flag = !self.all_locations_reach_destination(); 
        }
        number_steps
    }

    fn all_locations_reach_destination(&self) -> bool{
        let mut reach_location : bool = true;
        for (i, goal_loc) in self.goal_locations.iter().enumerate() {
            if goal_loc != &self.current_locations[i] {
                reach_location = false;
                break;
            }
        }
        reach_location
    }
}