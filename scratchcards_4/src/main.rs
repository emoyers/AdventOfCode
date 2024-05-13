// --- Day 4: Scratchcards ---
// The gondola takes you up. Strangely, though, the ground doesn't seem to be coming with you; you're not climbing a mountain. As the circle of Snow Island recedes below you, an entire new landmass suddenly appears above you! The gondola carries you to the surface of the new island and lurches into the station.

// As you exit the gondola, the first thing you notice is that the air here is much warmer than it was on Snow Island. It's also quite humid. Is this where the water source is?

// The next thing you notice is an Elf sitting on the floor across the station in what seems to be a pile of colorful square cards.

// "Oh! Hello!" The Elf excitedly runs over to you. "How may I be of service?" You ask about water sources.

// "I'm not sure; I just operate the gondola lift. That does sound like something we'd have, though - this is Island Island, after all! I bet the gardener would know. He's on a different island, though - er, the small kind surrounded by water, not the floating kind. We really need to come up with a better naming scheme. Tell you what: if you can help me with something quick, I'll let you borrow my boat and you can go visit the gardener. I got all these scratchcards as a gift, but I can't figure out what I've won."

// The Elf leads you over to the pile of colorful cards. There, you discover dozens of scratchcards, all with their opaque covering already scratched off. Picking one up, it looks like each card has two lists of numbers separated by a vertical bar (|): a list of winning numbers and then a list of numbers you have. You organize the information into a table (your puzzle input).

// As far as the Elf has been able to figure out, you have to figure out which of the numbers you have appear in the list of winning numbers. The first match makes the card worth one point and each match after the first doubles the point value of that card.

// For example:

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
// In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have, four of them (48, 83, 17, and 86) are winning numbers! That means card 1 is worth 8 points (1 for the first match, then doubled three times for each of the three matches after the first).

// Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
// Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
// Card 4 has one winning number (84), so it is worth 1 point.
// Card 5 has no winning numbers, so it is worth no points.
// Card 6 has no winning numbers, so it is worth no points.
// So, in this example, the Elf's pile of scratchcards is worth 13 points.

// Take a seat in the large pile of colorful cards. How many points are they worth in total?

// --- Part Two ---
// Just as you're about to report your findings to the Elf, one of you realizes that the rules have actually been printed on the back of every card this whole time.

// There's no such thing as "points". Instead, scratchcards only cause you to win more scratchcards equal to the number of winning numbers you have.

// Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

// Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)

// This time, the above example goes differently:

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
// Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
// Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
// Your copy of card 2 also wins one copy each of cards 3 and 4.
// Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
// Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
// Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
// Your one instance of card 6 (one original) has no matching numbers and wins no more cards.
// Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!

// Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, how many total scratchcards do you end up with?

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Debug)]
enum TypeRun {
    FirstPart,
    SecondPart
}

struct Card {
    winning_numbers: Vec<u32>,
    selected_numbers: Vec<u32>,
}

impl std::str::FromStr for Card {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers_string: Vec<&str> = s.split('|').collect();

        let wining_numbers: Vec<u32> = numbers_string[0].split_whitespace()
            .map(|num| num.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        
        let selected_numbers: Vec<u32> = numbers_string[1].split_whitespace()
            .map(|num| num.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Card {
            winning_numbers: wining_numbers,
            selected_numbers: selected_numbers,
        })
    }
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


    let mut total_sum: u32 = 0;

    let mut scratchcards_q: VecDeque<u32> = VecDeque::new();

    for line in reader.lines() {

        let line_string =  line?;
        let card_info_s: Vec<&str> = line_string.split(':').collect();

        let mut card_info : Card;
        
        match Card::from_str(card_info_s[1]) {
            Ok(c) => card_info = c,
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, 
                    "Could not convert string into card :(")); 
            },
        }

        card_info.winning_numbers.sort();
        card_info.selected_numbers.sort();

        let match_numbers = get_number_match_numbers(&card_info);
        
        if matches!(type_run, TypeRun::FirstPart){
            total_sum += if match_numbers > 0 {2_u32.pow(match_numbers-1)} else {0};
        }
        else {

            if scratchcards_q.is_empty(){

                // This part is in case the are not previous copies won from the current card
                for _ in 0 .. match_numbers {
                    scratchcards_q.push_back(1);
                }

                total_sum += 1;
            }
            else {
                // This part is in case the are previous copies won from the current card

                // Read the previous won card and add (+1) the default one you have
                let num_scratchcards_current_card:u32 = scratchcards_q.get(0).unwrap() + 1;

                // Remove the count of card won from the current card
                scratchcards_q.pop_front();

                total_sum += num_scratchcards_current_card;

                // Add the the accumulative won cards +1 to the next cards equal to the match numbers
                for i in 0 .. match_numbers{

                    if i < scratchcards_q.len().try_into().unwrap() {
                        if let Some(elem) = scratchcards_q.get_mut(i.try_into().unwrap()) {
                            *elem += num_scratchcards_current_card;
                        }
                    }
                    else {
                        scratchcards_q.push_back(num_scratchcards_current_card);
                    }

                }
            }
        }

    }


    println!("The total sum for the quest for the {:?} is: {total_sum}", type_run);

    Ok(())
}

fn get_number_match_numbers(card_info: &Card) -> u32
{
    let mut index_winning: usize = 0;
    let mut index_selected: usize = 0;

    let mut match_numbers: u32 = 0;

    while index_winning < card_info.winning_numbers.len() && index_selected < card_info.selected_numbers.len() {

        if card_info.winning_numbers[index_winning] == card_info.selected_numbers[index_selected]{
            match_numbers += 1;
            index_selected += 1;
            index_winning += 1;
        }
        else if card_info.winning_numbers[index_winning] < card_info.selected_numbers[index_selected] {
            index_winning += 1;
        }
        else{
            index_selected +=1;
        }
    }
    
    match_numbers
}
