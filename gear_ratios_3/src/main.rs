// --- Day 3: Gear Ratios ---
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

// "Aaah!"

// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

// Here is an example engine schematic:

// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

// --- Part Two ---
// The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

// Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

// This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

// Consider the same engine schematic again:

// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

// What is the sum of all of the gear ratios in your engine schematic?

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, usize};

#[derive(Debug)]
enum TypeRun {
    FirstPart,
    SecondPart
}

enum TypeLine {
    FirstLine,
    NormalLine,
    LastLine
}

#[derive(Debug)]
enum TypeCoordinateRage {
    Previuos,
    Current,
    Next
}

fn main() -> std::io::Result<()>
{
    algorithm(TypeRun::FirstPart)?;
    algorithm(TypeRun::SecondPart)?;

    Ok(()) 
}

fn algorithm (type_run: TypeRun) -> std::io::Result<()> 
{
    // Open the file for reading
    let file: File = File::open("data/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    let mut curr_line: String = "".to_string();
    let mut prev_line: String = "".to_string();
    let mut pre_prev_line: String = "".to_string();

    let mut total_sum: u32 = 0;
    let mut map_asterix: HashMap<(usize, usize), Vec<u32>> = HashMap::new(); // only for part 2

    let mut line_index: usize = 0; // Only for part 2
    for (i, line) in reader.lines().enumerate() {

        curr_line = line?;

        if 0 < i {

            let mut type_line: TypeLine = TypeLine::NormalLine;
            
            if 1 == i {
                type_line = TypeLine::FirstLine;
            }

            if matches!(type_run, TypeRun::FirstPart){
                total_sum += check_lines(&pre_prev_line, &prev_line, &curr_line, &type_line);
            }
            else {
                check_lines_2(line_index, &mut map_asterix,&pre_prev_line, &prev_line, 
                    &curr_line, &type_line);
            }

            line_index = i;
        }

        pre_prev_line = prev_line.clone();
        prev_line = curr_line.clone();
    }

    // This part is done to consider also the last line
    if matches!(type_run, TypeRun::FirstPart){
        total_sum += check_lines(&pre_prev_line, &prev_line, &curr_line, 
            &TypeLine::LastLine);
    }
    else {
        check_lines_2(line_index,&mut map_asterix,&pre_prev_line, &prev_line, &curr_line, 
            &TypeLine::LastLine);

        // checking with asterix has exactly two part numbers 
        for (_, values) in &map_asterix {
            
            if values.len() == 2 {
                total_sum += values[0] * values[1];
            }
        }
    }

    println!("The total sum for the quest for the {:?} is: {total_sum}", type_run);

    Ok(())
}

fn check_lines(line_prev: &String, line_curr: &String, line_next: &String, type_line: &TypeLine) -> u32
{

    let line_prev_chars: Vec<char> = line_prev.chars().collect();
    let line_curr_chars: Vec<char> = line_curr.chars().collect();
    let line_next_chars: Vec<char> = line_next.chars().collect();
    let mut pos = 0;

    let mut sum: u32 = 0;

    while pos < line_curr_chars.len(){
        
        if line_curr_chars[pos].is_digit(10) {
            let (pos_start, pos_end) = get_boundary(&line_curr_chars, pos);
            let ranges = get_coordinates_ranges((pos_start, pos_end), &type_line, line_curr_chars.len());
            pos = pos_end;

            if is_there_a_symbol(&ranges, &line_prev_chars, &line_curr_chars, &line_next_chars){
                sum += line_curr[pos_start..=pos_end].parse::<u32>().unwrap();
            }
        }
        pos += 1;
    }

    sum
}

fn check_lines_2(line_index: usize, map_asterix: & mut HashMap<(usize, usize), Vec<u32>>, 
    line_prev: &String, line_curr: &String, line_next: &String, type_line: &TypeLine)
{

    let line_prev_chars: Vec<char> = line_prev.chars().collect();
    let line_curr_chars: Vec<char> = line_curr.chars().collect();
    let line_next_chars: Vec<char> = line_next.chars().collect();
    let mut pos = 0;

    while pos < line_curr_chars.len(){
        
        if line_curr_chars[pos].is_digit(10) {
            let (pos_start, pos_end) = get_boundary(&line_curr_chars, pos);
            let ranges = get_coordinates_ranges((pos_start, pos_end), &type_line, line_curr_chars.len());
            pos = pos_end;

            let arterix_coords = get_asterix(line_index, &ranges, &line_prev_chars, 
                    &line_curr_chars, &line_next_chars);
            
            // Adding to map
            for (x, y) in arterix_coords {
                let number = line_curr[pos_start..=pos_end].parse::<u32>().unwrap();

                // In case the entry exists at the number to end of vector
                if let Some(values) = map_asterix.get_mut(&(x, y)) {
                    values.push(number);
                }
                // If not just create the entry
                else {
                    map_asterix.insert((x,y), vec![number]);
                }
            }
        }
        pos += 1;
    }
}

fn get_boundary(line_chars: &Vec<char>, intial_pose: usize) -> (usize, usize)
{
    
    let mut index = intial_pose+1;
    let mut end_pose = intial_pose;

    while index < line_chars.len() {

        if !line_chars[index].is_digit(10){
            break;
        }

        end_pose = index;
        index += 1;
    }

    (intial_pose, end_pose)
}

fn get_coordinates_ranges(number_pose: (usize, usize), type_line: &TypeLine, curr_len: usize) 
        -> Vec<(TypeCoordinateRage, (usize, usize))>
{
    
    let (start_pose, end_pose) = number_pose;
    
    let mut vector_result : Vec<(TypeCoordinateRage, (usize, usize))> = Vec::new();

    let mut index_begin = start_pose;
    let mut index_end = end_pose;

    //current line
    if start_pose > 0 {
        index_begin -= 1;
        vector_result.push((TypeCoordinateRage::Current, (start_pose-1, start_pose-1)));
    }

    if end_pose < curr_len - 1{
        index_end += 1;
        vector_result.push((TypeCoordinateRage::Current, (end_pose+1, end_pose+1)));
    }

    // prev line
    if !matches!(type_line, TypeLine::FirstLine) {
        vector_result.push((TypeCoordinateRage::Previuos, (index_begin, index_end)));
    }

    // next line
    if !matches!(type_line, TypeLine::LastLine) {
        vector_result.push((TypeCoordinateRage::Next, (index_begin, index_end)));
    }

    vector_result
}

fn is_there_a_symbol(ranges: &Vec<(TypeCoordinateRage, (usize, usize))>, 
        prev_chars: &Vec<char>, curr_chars: &Vec<char>, next_chars: &Vec<char>) -> bool
{
    
    for (type_coordinate, (start_index, end_index)) in ranges {
        
        let char_vector : &Vec<char>;

        if matches!(type_coordinate, TypeCoordinateRage::Previuos){
            char_vector = prev_chars;
        }
        else if matches!(type_coordinate, TypeCoordinateRage::Current){
            char_vector = curr_chars;
        }
        else {
            char_vector = next_chars;
        }

        for i in *start_index ..=*end_index {
            if char_is_symbol(char_vector[i]){
                return true;
            }
        }
    }

    false
}

fn char_is_symbol (character: char) -> bool
{
    return !character.is_alphanumeric() && character != '.';
}

fn get_asterix(line_index: usize, ranges: &Vec<(TypeCoordinateRage, (usize, usize))>, 
        prev_chars: &Vec<char>, curr_chars: &Vec<char>, next_chars: &Vec<char>) -> Vec<(usize, usize)>
{
    let mut asterix_coords: Vec<(usize, usize)> = Vec::new();
    for (type_coordinate, (start_index, end_index)) in ranges {
        
        let char_vector : &Vec<char>;
        let mut line_index_asterix = line_index;

        if matches!(type_coordinate, TypeCoordinateRage::Previuos){
            char_vector = prev_chars;
            line_index_asterix -= 1;
        }
        else if matches!(type_coordinate, TypeCoordinateRage::Current){
            char_vector = curr_chars;
        }
        else {
            char_vector = next_chars;
            line_index_asterix += 1;
        }

        for i in *start_index ..=*end_index {
            if char_vector[i] == '*'{
                asterix_coords.push((line_index_asterix, i));
            }
        }
    }

    asterix_coords
}
