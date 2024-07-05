// --- Day 10: Pipe Maze ---
// You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.

// You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.

// The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.

// Scanning the area, you discover that the entire field you're standing on is densely packed with pipes; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).

// The pipes are arranged in a two-dimensional grid of tiles:

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
// Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.

// For example, here is a square loop of pipe:

// .....
// .F-7.
// .|.|.
// .L-J.
// .....
// If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

// .....
// .S-7.
// .|.|.
// .L-J.
// .....
// In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.

// Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:

// -L|F7
// 7S-7|
// L|7||
// -L-J|
// L|-JF
// In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).

// Here is a sketch that contains a slightly more complex main loop:

// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
// Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

// 7-F7-
// .FJ|7
// SJLL7
// |F--J
// LJ.LJ
// If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.

// In the first example with the square loop:

// .....
// .S-7.
// .|.|.
// .L-J.
// .....
// You can count the distance each tile in the loop is from the starting point like this:

// .....
// .012.
// .1.3.
// .234.
// .....
// In this example, the farthest point from the start is 4 steps away.

// Here's the more complex loop again:

// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
// Here are the distances for each tile on that loop:

// ..45.
// .236.
// 01.78
// 14567
// 23...
// Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

#[derive(Debug)]
enum TypeRun {
    FirstPart,
    SecondPart,
}

#[derive(Debug, Clone)]
struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    
    fn new() -> Self {
        Coordinate {
            x: 0,
            y: 0,
        }
    }

    fn new_with_vals(_x: u16, _y: u16) -> Self {
        Coordinate {
            x: _x,
            y: _y,
        }
    }
}

struct MapInfo {
    data: Vec<Vec<char>>,
    distance_to_start: Vec<Vec<u16>>,
    start_pose : Coordinate,
}

impl MapInfo {
    
    fn new() -> Self {

        MapInfo {
            data: Vec::new(),
            distance_to_start: Vec::new(),
            start_pose: Coordinate::new(),
        }
    }

    fn add_row_to_map(& mut self, row : &String) {

        let chars: Vec<char> = row.chars().collect();

        for (col, c) in chars.iter().enumerate() {
            if *c == 'S' {
                self.start_pose.x = col as u16;
                self.start_pose.y = self.data.len() as u16;
            }
        }

        self.distance_to_start.push(vec![u16::MAX; chars.len()]);
        self.data.push(chars);
    }

    fn get_furthest_distance_to_start(& mut self) -> u16{
        let mut max_distance: u16 = 0;
        let mut q: VecDeque<Coordinate> = VecDeque::new();
        
        self.distance_to_start[self.start_pose.y as usize][self.start_pose.x as usize] = 0;

        q.push_back(self.start_pose.clone());

        while !q.is_empty() {

            if let Some(cur_coord) = q.front() {
                let neighboors: Vec<Coordinate> = self.get_next_coordinates(cur_coord);

                let current_distance = self.distance_to_start[cur_coord.y as usize][cur_coord.x as usize];

                if max_distance < current_distance{
                    max_distance = current_distance;
                }

                for neig_coord in neighboors {
                    if current_distance + 1 < self.distance_to_start[neig_coord.y as usize][neig_coord.x as usize] {
                        self.distance_to_start[neig_coord.y as usize][neig_coord.x as usize] = current_distance + 1;
                        
                        q.push_back(neig_coord);

                    }
                }
                
                q.pop_front();
            }
        }

        max_distance

    }

    fn get_coordinates(&self,current_coord: &Coordinate, moves: &Vec<(i8, i8)>)-> Vec<Coordinate> {
        let mut next_coords: Vec<Coordinate> = Vec::new();

        for (mov_x, mov_y) in moves {

            if *mov_x != 0 {

                if *mov_x == -1 && current_coord.x > 0 {

                    next_coords.push(Coordinate::new_with_vals(current_coord.x-1, current_coord.y));
                }
                else if *mov_x == 1 && current_coord.x < (self.data[0].len() - 1) as u16 {
                    next_coords.push(Coordinate::new_with_vals(current_coord.x+1, current_coord.y));
                }

            }
            else {
                if *mov_y == -1 && current_coord.y > 0 {
                    next_coords.push(Coordinate::new_with_vals(current_coord.x, current_coord.y-1));
                }
                else if *mov_y == 1 && current_coord.y < (self.data.len() - 1) as u16 {
                    next_coords.push(Coordinate::new_with_vals(current_coord.x, current_coord.y+1));
                }
            }

        }

        next_coords
    }

    fn get_next_coordinates(&self, current_coord: &Coordinate) -> Vec<Coordinate> {
        let mut next_coords: Vec<Coordinate> = Vec::new();
        let mut movements: Vec<(i8, i8)> = Vec::new();

        match self.data[current_coord.y as usize][current_coord.x as usize] {
            'S' => {
                movements.extend(vec![(0,1),(0,-1),(1,0),(-1,0)]);


                // Check if the neighboor of S also connect to back to S
                let s_neigh_coords = self.get_coordinates(current_coord,&movements);
                for s_neigh in &s_neigh_coords{
                    let s_neigh_neigh_coords = self.get_next_coordinates(&s_neigh);
                    
                    // Check connection back to S
                    for s_neigh_neigh in s_neigh_neigh_coords {

                        // If there is connection added to next coordinates
                        if (s_neigh_neigh.x == current_coord.x) && (s_neigh_neigh.y == current_coord.y) {
                            next_coords.push(s_neigh.clone());
                        }
                    }
                }
            },
            '|' => {
                movements.extend(vec![(0,1),(0,-1)]);
                next_coords.extend(self.get_coordinates(current_coord,&movements));
            },
            '-' => {
                movements.extend(vec![(1,0),(-1,0)]);
                next_coords.extend(self.get_coordinates(current_coord,&movements));
            },
            'L' => {
                movements.extend(vec![(1,0),(0,-1)]);
                next_coords.extend(self.get_coordinates(current_coord,&movements));
            },
            'J' => {
                movements.extend(vec![(-1,0),(0,-1)]);
                next_coords.extend(self.get_coordinates(current_coord,&movements));
            },
            '7' => {
                movements.extend(vec![(-1,0),(0,1)]);
                next_coords.extend(self.get_coordinates(current_coord,&movements));
            },
            'F' => {
                movements.extend(vec![(1,0),(0,1)]);
                next_coords.extend(self.get_coordinates(current_coord,&movements));
            },
            _ => { /* Do nothing */},
        }

        next_coords
    }

}

fn main() -> std::io::Result<()> {
    algorithm(TypeRun::FirstPart)?;
    algorithm(TypeRun::SecondPart)?;

    Ok(())
}

fn algorithm(type_run: TypeRun) -> std::io::Result<()> {

    let file: File = File::open("data/input.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut map : MapInfo = MapInfo::new();

    for line in reader.lines(){

        let line_str = line?;
        map.add_row_to_map(&line_str);
    }

    println!("Farthes point in the loop to the start {} for run {:?}", map.get_furthest_distance_to_start(), type_run);

    Ok(())
}
