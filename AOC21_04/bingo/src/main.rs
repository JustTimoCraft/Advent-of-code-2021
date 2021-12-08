use std::fs; // For reading the file
mod bingo_card;
use crate::bingo_card::BingoCard;

fn tests() {
    let mut bingo_card_one = BingoCard::new([
        11, 12, 13, 14, 15, 21, 22, 23, 24, 25, 31, 32, 33, 34, 35, 41, 42, 43, 44, 45, 51, 52, 53,
        54, 55,
    ]);
    assert_eq!(bingo_card_one.check_for_bingo(), false);
    bingo_card_one.activate_field(11);
    bingo_card_one.activate_field(22);
    bingo_card_one.activate_field(33);
    bingo_card_one.activate_field(44);
    bingo_card_one.activate_field(55);
    assert_eq!(bingo_card_one.check_for_bingo(), true);

    //println!("{}", bingo_card_one);
    //println!("{}", bingo_card_one.calculate_score(33));
    assert_eq!(bingo_card_one.calculate_score(33), 21780);
}

fn main() {
    // Read the file
    let filecontents = fs::read_to_string("./../input.txt").expect("Error while reading the file");
    // Add the lines to a String Vector
    let lines = filecontents.split("\n").collect::<Vec<&str>>();

    // For debug purposes, run a bunch of tests
    tests();

    // Get the moves from the first line as String references
    let mut moves: Vec<u8> = Vec::new();
    {
        let moves_str = lines[0].split(",").collect::<Vec<&str>>();
        for str_ref in moves_str {
            moves.push(str_ref.parse::<u8>().unwrap());
        }
    }

    // Create field arrays from the input file
    let mut field_arrays: Vec<[u8; 25]> = Vec::new();
    {
        let mut array_buffer: [u8; 25] = [0; 25];
        let mut linecount = 0;
        for line in lines.into_iter() {
            if line == "" {
                // Reset upon new line
                array_buffer = [0; 25];
                linecount = 0;
                continue;
            }
            // Only if a line gives us exactly 5 values, continue
            let line_values = line.split(" ").collect::<Vec<&str>>();
            if line_values.len() >= 5 && line_values.len() <= 10 {
                // An extra space means an extra value (because of single digit numbers)
                let mut actual_values: Vec<&str> = Vec::new();
                for value in &line_values {
                    if value != &"" {
                        actual_values.push(value);
                    }
                }
                // After all that, we should be left with exactly 5 values
                if actual_values.len() == 5 {
                    for i in 0..5 {
                        array_buffer[linecount * 5 + i] = actual_values[i].parse::<u8>().unwrap();
                    }
                    linecount += 1;
                    if linecount == 5 {
                        field_arrays.push(array_buffer);
                    }
                }
            }
        }
    }

    // Put all bingo cards in a Vector
    let mut bingo_cards: Vec<BingoCard> = Vec::new();
    for field_array in field_arrays {
        bingo_cards.push(BingoCard::new(field_array));
    }

    println!("---------- Part 1 ----------\n");

    // Main execution loop
    'bingo: loop {
        for (index, number) in moves.iter().enumerate() {
            for card in bingo_cards.iter_mut() {
                card.activate_field(*number);
            }
            // Can't get bingo in the first 5 numbers
            if index >= 5 {
                for card in bingo_cards.iter_mut() {
                    if card.calculate_score(*number) > 0 {
                        println!("Bingo found! After {} numbers in this card:", index + 1);
                        println!("{}", card);
                        println!(
                            "The final calculated score is: {}",
                            card.calculate_score(*number)
                        );
                        break 'bingo;
                    }
                }
            }
        }
    }

    println!("\n---------- Part 2 ----------\n");

    // Reset the cards to their 'inactive' states
    for cards in bingo_cards.iter_mut() {
        cards.reset();
    }

    // Second program loop
    let mut cycles = 0;
    let mut remove_cards: Vec<usize> = Vec::new();
    while bingo_cards.len() > 1 {
        // Remove cards that were flagged as candidates for
        // removal in the previous cycle
        for (number, index) in remove_cards.iter().enumerate() {
            bingo_cards.remove(index - number);
        }
        // Clear the vector after removing the proper bingo cards
        remove_cards.clear();

        // Cross the number off each of the cards
        for cards in bingo_cards.iter_mut() {
            cards.activate_field(moves[cycles]);
        }
        cycles += 1;

        if cycles >= 5 {
            for (index, card) in bingo_cards.iter_mut().enumerate() {
                if card.check_for_bingo() {
                    remove_cards.push(index);
                }
            }
        }
    }

    println!("Found the last card to get bingo:\n\n{}", bingo_cards[0]);
    println!(
        "It has a score of {}",
        bingo_cards[0].calculate_score(moves[cycles - 1])
    );
}
