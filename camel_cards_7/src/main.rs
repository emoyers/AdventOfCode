// --- Day 7: Camel Cards ---
// Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

// "Did you bring the parts?"

// You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

// "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

// "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

// After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

// You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

// Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

// In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

// Every hand is exactly one type. From strongest to weakest, they are:

// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456
// Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

// If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.

// So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).

// To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
// This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.

// So, the first step is to put the hands in order of strength:

// 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
// KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
// T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
// Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

// Find the rank of every hand in your set. What are the total winnings?

// --- Part Two ---
// To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

// To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

// J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

// Now, the above example goes very differently:

// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
// 32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
// KK677 is now the only two pair, making it the second-weakest hand.
// T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
// With the new joker rule, the total winnings in this example are 5905.

// Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::Ordering;
use crate::data_structures::min_heap::MinHeap;

pub mod data_structures;

#[derive(PartialEq, Debug)]
enum TypeRun {
    FirstPart,
    SecondPart
}

fn main() -> std::io::Result<()>{
    algorithm(TypeRun::FirstPart)?;
    algorithm(TypeRun::SecondPart)?;

    Ok(())
}

fn algorithm(type_run: TypeRun) -> std::io::Result<()> {
    
    let file:File = File::open("data/input.txt")?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut hands_heap: MinHeap<HandInfo> = MinHeap::new();

    let use_joker: bool = if type_run == TypeRun::FirstPart {false} else {true};

    for line in reader.lines(){
        let line_str:String = line?;
        hands_heap.push(HandInfo::new(&line_str, use_joker));
    }

    println!("The result of {:?} is: {}", type_run, get_camel_card_result(&mut hands_heap));

    Ok(())
}

fn get_camel_card_result(hands_data: &mut MinHeap<HandInfo>) -> u64{
    let mut result: u64 = 0;
    let mut multiplier: u64 = 1;

    while !hands_data.empty() {

        match hands_data.top() {
            Some(x) => {
                result += multiplier * x.value as u64;
                multiplier += 1;
            },
            None => println!("not data found on peek! :("),
        }

        hands_data.pop();
    }


    result
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct HandInfo {
    data: String,
    value: u16,
    h_type: HandType,
    using_joker: bool,
}

impl HandInfo {

    fn new(info_card: &str, with_joker: bool) -> Self {

        let data:Vec<&str> = info_card.split_whitespace().collect();

        HandInfo {
            data: data[0].to_string(),
            value: data[1].parse::<u16>().unwrap_or(0),
            h_type: Self::get_type_hand(data[0], with_joker),
            using_joker: with_joker
        }
    }

    fn get_type_hand(data_card: &str, with_joker: bool) -> HandType{

        let mut frequency_chars_map: HashMap<char, u8> = HashMap::new();
        let mut number_jokers: u8 = 0;
        for c in data_card.chars() {
            if let Some(x) = frequency_chars_map.get_mut(&c) {
                *x += 1;
            }
            else {
                frequency_chars_map.insert(c, 1);
            }

            if c == 'J' {number_jokers +=1;}
        }

        // Convert to vector and sort
        let mut frequency_chars_vec: Vec<(&char, &u8)> = frequency_chars_map.iter().collect();
        frequency_chars_vec.sort_by(|a, b| b.1.cmp(&a.1));

        let mut hand_type: HandType;
        if *frequency_chars_vec[0].1 == 5 {
            hand_type = HandType::FiveOfAKind;
        }
        else if *frequency_chars_vec[0].1 == 4 {
            hand_type = HandType::FourOfAKind   
        }
        else if *frequency_chars_vec[0].1 == 3 {
            
            if (frequency_chars_vec.len() > 1) && (*frequency_chars_vec[1].1 == 2) {
                hand_type = HandType::FullHouse;
            }
            else {
                hand_type = HandType::ThreeOfAKind;
            }   
        }
        else if *frequency_chars_vec[0].1 == 2 {
            
            if (frequency_chars_vec.len() > 1) && (*frequency_chars_vec[1].1 == 2) {
                hand_type = HandType::TwoPair;
            }
            else {
                hand_type = HandType::OnePair;
            }   
        }
        else {
            hand_type = HandType::HighCard;
        }

        // Modifying hand type in case there is a joker
        if with_joker && (number_jokers > 0) {

            if hand_type >= HandType::FullHouse {
                hand_type = HandType::FiveOfAKind;
            }
            else if hand_type == HandType::ThreeOfAKind {
                if number_jokers < 3 {
                    hand_type = HandType::FourOfAKind;
                }
            }
            else if hand_type == HandType::TwoPair {
                if number_jokers == 1 {
                    hand_type = HandType::FullHouse;
                }
                else {
                    hand_type = HandType::FourOfAKind;
                }
            }
            else if  hand_type == HandType::OnePair {
                hand_type = HandType::ThreeOfAKind;
            }
            else {
                hand_type = HandType::OnePair;
            }
        }

        hand_type
    }

    fn get_relative_streght(card: char, with_joker: bool) -> u8{

        let streght: u8;
        match card {
            'A' => streght = 14,
            'K' => streght = 13,
            'Q' => streght = 12,
            'J' => {
                if with_joker {streght = 1;} else {streght = 11;} 
            },
            'T' => streght = 10,
            '9' => streght = 9,
            '8' => streght = 8,
            '7' => streght = 7,
            '6' => streght = 6,
            '5' => streght = 5,
            '4' => streght = 4,
            '3' => streght = 3,
            '2' => streght = 2,
            _   => streght = 0,  
        };

        streght
    }
    
}

impl Ord for HandInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut order: Ordering = Ordering::Equal;
        if self.h_type > other.h_type {
            order = Ordering::Greater;
        }
        else if self.h_type < other.h_type {
            order = Ordering::Less;
        }
        else {
            let self_data_chars: Vec<char> = self.data.chars().collect();
            let other_data_chars: Vec<char> = other.data.chars().collect();
            if self_data_chars.len() == other_data_chars.len(){

                let mut index_to_check: usize = 0;
                let mut index_found: bool = false;

                for i in 0 .. self_data_chars.len() {

                    if self_data_chars[i] != other_data_chars[i] {
                        index_found = true;
                        index_to_check = i;
                        break;
                    }
                }

                if index_found {

                    if HandInfo::get_relative_streght(self_data_chars[index_to_check], self.using_joker) > 
                        HandInfo::get_relative_streght(other_data_chars[index_to_check], other.using_joker) {
                        order = Ordering::Greater;
                    }
                    else {
                        order = Ordering::Less;
                    }
                }
                else {
                    order = Ordering::Equal;
                }
            }

        }
        order
    }
}

impl PartialOrd for HandInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
