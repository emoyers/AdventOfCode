// --- Day 9: Mirage Maintenance ---
// You ride the camel through the sandstorm and stop where the ghost's maps told you to stop. The sandstorm subsequently subsides, somehow seeing you standing at an oasis!

// The camel goes to get some water and you stretch your neck. As you look up, you discover what must be yet another giant floating island, this one made of metal! That must be where the parts to fix the sand machines come from.

// There's even a hang glider partially buried in the sand here; once the sun rises and heats up the sand, you might be able to use the glider and the hot air to get all the way up to the metal island!

// While you wait for the sun to rise, you admire the oasis hidden here in the middle of Desert Island. It must have a delicate ecosystem; you might as well take some ecological readings while you wait. Maybe you can report any environmental instabilities you find to someone so the oasis can be around for the next sandstorm-worn traveler.

// You pull out your handy Oasis And Sand Instability Sensor and analyze your surroundings. The OASIS produces a report of many values and how they are changing over time (your puzzle input). Each line in the report contains the history of a single value. For example:

// 0 3 6 9 12 15
// 1 3 6 10 15 21
// 10 13 16 21 30 45
// To best protect the oasis, your environmental report should include a prediction of the next value in each history. To do this, start by making a new sequence from the difference at each step of your history. If that sequence is not all zeroes, repeat this process, using the sequence you just generated as the input sequence. Once all of the values in your latest sequence are zeroes, you can extrapolate what the next value of the original history should be.

// In the above dataset, the first history is 0 3 6 9 12 15. Because the values increase by 3 each step, the first sequence of differences that you generate will be 3 3 3 3 3. Note that this sequence has one fewer value than the input sequence because at each step it considers two numbers from the input. Since these values aren't all zero, repeat the process: the values differ by 0 at each step, so the next sequence is 0 0 0 0. This means you have enough information to extrapolate the history! Visually, these sequences can be arranged like this:

// 0   3   6   9  12  15
//   3   3   3   3   3
//     0   0   0   0
// To extrapolate, start by adding a new zero to the end of your list of zeroes; because the zeroes represent differences between the two values above them, this also means there is now a placeholder in every sequence above it:

// 0   3   6   9  12  15   B
//   3   3   3   3   3   A
//     0   0   0   0   0
// You can then start filling in placeholders from the bottom up. A needs to be the result of increasing 3 (the value to its left) by 0 (the value below it); this means A must be 3:

// 0   3   6   9  12  15   B
//   3   3   3   3   3   3
//     0   0   0   0   0
// Finally, you can fill in B, which needs to be the result of increasing 15 (the value to its left) by 3 (the value below it), or 18:

// 0   3   6   9  12  15  18
//   3   3   3   3   3   3
//     0   0   0   0   0
// So, the next value of the first history is 18.

// Finding all-zero differences for the second history requires an additional sequence:

// 1   3   6  10  15  21
//   2   3   4   5   6
//     1   1   1   1
//       0   0   0
// Then, following the same process as before, work out the next value in each sequence from the bottom up:

// 1   3   6  10  15  21  28
//   2   3   4   5   6   7
//     1   1   1   1   1
//       0   0   0   0
// So, the next value of the second history is 28.

// The third history requires even more sequences, but its next value can be found the same way:

// 10  13  16  21  30  45  68
//    3   3   5   9  15  23
//      0   2   4   6   8
//        2   2   2   2
//          0   0   0
// So, the next value of the third history is 68.

// If you find the next value for each history in this example and add them together, you get 114.

// Analyze your OASIS report and extrapolate the next value for each history. What is the sum of these extrapolated values?

// --- Part Two ---
// Of course, it would be nice to have even more history included in your report. Surely it's safe to just extrapolate backwards as well, right?

// For each history, repeat the process of finding differences until the sequence of differences is entirely zero. Then, rather than adding a zero to the end and filling in the next values of each previous sequence, you should instead add a zero to the beginning of your sequence of zeroes, then fill in new first values for each previous sequence.

// In particular, here is what the third example history looks like when extrapolating back in time:

// 5  10  13  16  21  30  45
//   5   3   3   5   9  15
//    -2   0   2   4   6
//       2   2   2   2
//         0   0   0
// Adding the new values on the left side of each sequence from bottom to top eventually reveals the new left-most history value: 5.

// Doing this for the remaining example data above results in previous values of -3 for the first history and 0 for the second history. Adding all three new values together produces 2.

// Analyze your OASIS report again, this time extrapolating the previous value for each history. What is the sum of these extrapolated values?

use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
enum TypeRun {
    FirstPart,
    SecondPart,
}

fn main() -> std::io::Result<()> {
    algorithm(TypeRun::FirstPart)?;
    algorithm(TypeRun::SecondPart)?;

    Ok(())
}

fn algorithm(type_run: TypeRun) -> std::io::Result<()> {

    let file: File = File::open("data/input.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut total_sum: i64 = 0;

    for line in reader.lines(){

        let line_str = line?;
        let numbers: Vec<i64> = line_str.split_whitespace()
                                         .map(|s| s.parse().unwrap())
                                         .collect();
        
        total_sum += get_next_sequence_number(&numbers, &type_run);
    }

    println!("The result for part {:?} is {total_sum}", type_run);

    Ok(())
}

fn get_next_sequence_number(list_numbers: &Vec<i64>, type_run: &TypeRun) -> i64 {
    
    let mut lists: Vec<Vec<i64>> = Vec::new();
    form_the_pyramid_lists(list_numbers, &mut lists);

    let mut result: i64 = 0;

    for i in 0..lists.len() {

        match type_run {
            TypeRun::FirstPart => result += lists[i][lists[i].len()-1],
            TypeRun::SecondPart => {

                if i%2 == 0 {
                    result += lists[i][0];
                }
                else {
                    result -= lists[i][0];
                }
            },
        }
    }

    result
}

fn form_the_pyramid_lists(list_numbers: &Vec<i64>, result_lists: &mut Vec<Vec<i64>>) {

    result_lists.push(list_numbers.clone());
    let mut current_list: &Vec<i64>;
    let mut continue_flag: bool = true;

    while continue_flag {

        current_list = &result_lists[result_lists.len()-1];
        if current_list.len() > 1  {
            let mut next_list: Vec<i64> = Vec::new();
            let mut last_number: i64 = 0;
            let mut new_number: i64;
            let mut counter: usize = 0;
            for i in 1..current_list.len() {
                new_number = current_list[i] - current_list[i-1];
                next_list.push(new_number);

                if (i==1) || (last_number == new_number) {
                    counter += 1;
                }

                last_number = new_number;
            }
            result_lists.push(next_list.clone());
            
            continue_flag = if counter == next_list.len() {false} else {true};
        }
    
    }
}
