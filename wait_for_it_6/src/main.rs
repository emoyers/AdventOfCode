// --- Day 6: Wait For It ---
// The ferry quickly brings you across Island Island. After asking around, you discover that there is indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of water and the small island where the ferry has docked.

// As you try to figure out what to do next, you notice a poster on a wall near the ferry dock. "Boat races! Open to the public! Grand prize is an all-expenses-paid trip to Desert Island!" That must be where the sand comes from! Best of all, the boat races are starting in just a few minutes.

// You manage to sign up as a competitor in the boat races just in time. The organizer explains that it's not really a traditional race - instead, you will get a fixed amount of time during which your boat has to travel as far as it can, and you win if your boat goes the farthest.

// As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed for each race and also the best distance ever recorded in that race. To guarantee you win the grand prize, you need to make sure you go farther in each race than the current record holder.

// The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually toy boats, each with a big button on top. Holding down the button charges the boat, and releasing the button allows the boat to move. Boats move faster if their button was held longer, but time spent holding the button counts against the total race time. You can only hold the button at the start of the race, and boats don't move until the button is released.

// For example:

// Time:      7  15   30
// Distance:  9  40  200
// This document describes three races:

// The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
// The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
// The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.
// Your toy boat has a starting speed of zero millimeters per millisecond. For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by one millimeter per millisecond.

// So, because the first race lasts 7 milliseconds, you only have a few options:

// Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
// Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
// Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
// Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
// Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
// Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
// Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
// Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.
// Since the current record for this race is 9 millimeters, there are actually 4 different ways you could win: you could hold the button for 2, 3, 4, or 5 milliseconds at the start of the race.

// In the second race, you could hold the button for at least 4 milliseconds and at most 11 milliseconds and beat the record, a total of 8 different ways to win.

// In the third race, you could hold the button for at least 11 milliseconds and no more than 19 milliseconds and still beat the record, a total of 9 ways you could win.

// To see how much margin of error you have, determine the number of ways you can beat the record in each race; in this example, if you multiply these values together, you get 288 (4 * 8 * 9).

// Determine the number of ways you could beat the record in each race. What do you get if you multiply these numbers together?

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum TypeRun {
    FirstPart,
    SecondPart
}

struct BoatRaceDb {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl BoatRaceDb {

    fn new () -> Self {

        BoatRaceDb {
            times: Vec::new(),
            distances: Vec::new(),
        }
    }

    fn populate_times_like_vector(&mut self, times_info_str: &str) {
        let data: Vec<&str> = times_info_str.split(':').collect();
        match data[1].split_whitespace().map(|num| 
            num.parse::<u32>()).collect::<Result<Vec<_>, _>>() { 
                Ok(numbers) => self.times.extend(numbers) ,
                Err(_) => println!("not able to transform string to times :("),
            
        }
    }

    fn populates_distances_like_vector(&mut self, distances_info_str: &str) {
        let data: Vec<&str> = distances_info_str.split(':').collect();
        match data[1].split_whitespace().map(|num| 
            num.parse::<u32>()).collect::<Result<Vec<_>, _>>() { 
                Ok(numbers) => self.distances.extend(numbers) ,
                Err(_) => println!("not able to transform string to distances :("),
            
        }
    }

    fn get_number_of_ways(&self) -> u32 {

        let mut number_of_ways: u32 = 1;
        
        for (i, time) in self.times.iter().enumerate() {

            number_of_ways *= BoatRaceDb::get_margin(*time, self.distances[i]);
        }

        number_of_ways
    }

    fn get_margin(time: u32, distance_to_beat: u32) -> u32
    {
        let lower_limit_inclusive = BoatRaceDb::get_lower_limit(time, distance_to_beat);
        let upper_limit_inclusive = BoatRaceDb::get_upper_limit(time,lower_limit_inclusive);

        upper_limit_inclusive - lower_limit_inclusive 
    }

    fn get_lower_limit(time: u32, distance_to_beat: u32) -> u32 
    {
        BoatRaceDb::binary_search(0, time, distance_to_beat)
    }

    fn get_upper_limit(time: u32, lower_bound_time:u32) -> u32 
    {
        time - lower_bound_time + 1
    }

    fn binary_search(mut low_time: u32, mut high_time: u32, target_distance: u32) -> u32 {

        let mut mid_time: u32;
        let mut distance_calculated: u32;
        let max_time = high_time;

        while low_time <= high_time{
            mid_time = low_time + (high_time - low_time) / 2;
            distance_calculated = BoatRaceDb::calculate_distance(mid_time, max_time);

            // If x greater, ignore left half
            if distance_calculated < target_distance {
                low_time = mid_time + 1;
            }

            // If x is smaller, ignore right half
            else{

                if mid_time == 0 {
                    return mid_time;
                }
                else if BoatRaceDb::calculate_distance(mid_time-1, max_time) <= target_distance{
                    return mid_time;
                }
                else {
                    high_time = mid_time - 1;
                }
            }
        }

        // If we reach here, then element was not present
        0
    }

    fn calculate_distance(time_pressed_button:u32, max_time:u32) -> u32{
        (max_time - time_pressed_button) * time_pressed_button
    }
}

fn main () -> std::io::Result<()> {
    algorithm(TypeRun::FirstPart)?;

    Ok(())
}

fn algorithm(type_run: TypeRun) -> std::io::Result<()>{

    // Open the file for reading
    let file: File = File::open("data/input.txt")?;

    // Create a buffered reader to read the file
    let reader: BufReader<File> = BufReader::new(file);

    let mut boat_race_db = BoatRaceDb::new();

    for line in reader.lines() {

        let line_str = line?;

        if line_str.contains("Time:") {
            boat_race_db.populate_times_like_vector(&line_str);
        }
        else if line_str.contains("Distance:") {
            boat_race_db.populates_distances_like_vector(&line_str);
        }
    }

    println!("The result of {:?} of Wait for It is: {}", type_run, boat_race_db.get_number_of_ways());

    Ok(())
}
