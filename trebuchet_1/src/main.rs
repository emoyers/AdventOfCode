use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
/*
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

/*
--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
*/
#[derive(Debug)]
enum TypeRun {
    FirstPart,
    SecondPart
}

fn main () -> std::io::Result<()> {
    algorithm(TypeRun::FirstPart)?;
    algorithm(TypeRun::SecondPart)?;

    Ok(())
}

fn algorithm (type_run: TypeRun) -> std::io::Result<()>
{
    // Open the file for reading
    let file = File::open("data/calibration_document.txt")?;

    //Create a buffered reader to read the file
    let reader = BufReader::new(file);

    let mut digits_dictionary = Trie::new();
    let helper = HelperWordsToDigit::new();
    helper.init_digit_dictionary(&mut digits_dictionary);
    
    let mut total_sum:u32 = 0;
    // Read the file line by line
    for line in reader.lines() {
        let line_string = line?;
        let line_string_reverse: String = line_string.chars().rev().collect();
        let chars:Vec<char> = line_string.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            if c.is_digit(10) {
                total_sum += 10 * c.to_digit(10).expect("Cannot convert char to digit");
                break;
            }
            else {
                if matches!(type_run, TypeRun::SecondPart) {
                    // Try searching for number name
                    let (result, word_len) = digits_dictionary.search(&line_string[i..]);
                    if matches!(result, TrieSearchResult::WordFoundExactMatch) || 
                    matches!(result, TrieSearchResult::WordFoundButNeedCropping) {
                        match helper.word_to_digit(&line_string[i..i+word_len]){
                            Some(number) => {
                                total_sum  += 10 * number;
                                break;
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        for (i, c) in chars.iter().rev().enumerate() {
            if c.is_digit(10) {
                total_sum += c.to_digit(10).expect("Cannot convert char to digit");
                break;
            }
            else {
                if matches!(type_run, TypeRun::SecondPart) {
                    // Try searching for number name
                    let (result, word_len) = digits_dictionary.search(&line_string_reverse[i..]);
                    if matches!(result, TrieSearchResult::WordFoundExactMatch) || 
                    matches!(result, TrieSearchResult::WordFoundButNeedCropping) {
                        match helper.word_to_digit(&line_string_reverse[i..i+word_len]){
                            Some(number) => {
                                total_sum  += number;
                                break;
                            }
                            None => {}
                        }
                    }
                }
            }
            
        }
    }

    println! ("The result of the {:?} of the puzzle is: {total_sum} !!!!", type_run);


    Ok(())
}

struct HelperWordsToDigit<'a>
{
    words: Vec<&'a str>,
    map_word_to_digit: HashMap<String, u32>
}

impl<'a> HelperWordsToDigit<'a> {

    fn new() -> HelperWordsToDigit<'a> {
        
        let temp_vec = vec!["zero", "one", "two", "three", 
            "four", "five", "six", "seven", "eight", "nine", "orez", "eno", "owt", 
            "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
        let mut temp_map: HashMap<String, u32> = HashMap::new();

        let mut counter = 0;
        for w in &temp_vec {
            temp_map.insert(w.to_string(), counter);
            counter = (counter + 1) % 10;
        }
        
        HelperWordsToDigit {
            words: temp_vec,
            map_word_to_digit: temp_map
        }
    }

    fn init_digit_dictionary(&self, dicitionary: &mut Trie) {

        for w in &self.words  {
            dicitionary.insert(w);
        }
    }

    fn word_to_digit(&self, word: &str) -> Option<&u32>{
        self.map_word_to_digit.get(word)
    }
}

#[derive(Default, Debug)]
struct TrieNode {
    is_end_word: bool,
    children: HashMap<char, TrieNode>
}

impl TrieNode {
    fn new() -> Self {

        TrieNode {
            is_end_word: false,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum TrieSearchResult {
    WordFoundButNeedCropping,
    WordFoundExactMatch,
    NotFound
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode
}

impl Trie {

    fn new() -> Self {
        Trie {
            root: TrieNode::new()
        }
    }
    
    fn insert(&mut self, word: &str) {
        let mut current_node: &mut TrieNode = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_word = true;
    }

    fn search (& self, word: &str) -> (TrieSearchResult, usize) {
        let mut current_node = &self.root;

        for (i, c) in word.chars().enumerate() {
            match current_node.children.get(&c) {
                Some(node) => {
                    if node.is_end_word{
                        
                        if i == (word.len() - 1) {
                            return (TrieSearchResult::WordFoundExactMatch, word.len());
                        }
                        else {
                            return (TrieSearchResult::WordFoundButNeedCropping, i+1);
                        }
                    }

                    current_node = node;
                },
                None => return (TrieSearchResult::NotFound, 0),
            }
        }
        
        if current_node.is_end_word {
            return (TrieSearchResult::WordFoundExactMatch, word.len());
        }
        else {
            return (TrieSearchResult::NotFound, 0);
        }
    }
    
}