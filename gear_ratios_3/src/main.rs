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
use std::{fs::File, io::{BufRead, BufReader}};

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

fn main() -> std::io::Result<()>{

    // Open the file for reading
    let file: File = File::open("data/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    let mut curr_line: String = "".to_string();
    let mut prev_line: String = "".to_string();
    let mut pre_prev_line: String = "".to_string();

    let mut total_sum: u32 = 0;

    for (i, line) in reader.lines().enumerate() {

        curr_line = line?;

        if 0 < i {

            let mut type_line: TypeLine = TypeLine::NormalLine;
            
            if 1 == i {
                type_line = TypeLine::FirstLine;
            }

            total_sum += check_lines(&pre_prev_line, &prev_line, &curr_line, &type_line);
        }

        pre_prev_line = prev_line.clone();
        prev_line = curr_line.clone();
    }
    total_sum += check_lines(&pre_prev_line, &prev_line, &curr_line, &TypeLine::LastLine);

    println!("The total sum for the quest is {total_sum}");

    Ok(())
}

fn check_lines(line_prev: &String, line_curr: &String, line_next: &String, type_line: &TypeLine) -> u32{

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

fn get_boundary(line_chars: &Vec<char>, intial_pose: usize) -> (usize, usize){
    
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
        -> Vec<(TypeCoordinateRage, (usize, usize))>{
    
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
        prev_chars: &Vec<char>, curr_chars: &Vec<char>, next_chars: &Vec<char>) -> bool{
    
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

fn char_is_symbol (character: char) -> bool{
    return !character.is_alphanumeric() && character != '.';
}
