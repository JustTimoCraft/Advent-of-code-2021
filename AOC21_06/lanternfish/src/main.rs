use std::fs;

fn main() {
    let filecontents = fs::read_to_string("./../input.txt").expect("Error reading file");
    let filedata = filecontents.split("\n").next().unwrap(); // We only have one line

    let mut lanternfish: Vec<u8> = Vec::new();
    let data: Vec<&str> = filedata.split(",").collect::<Vec<&str>>();
    for number in data {
        let entry = number.parse::<u8>().expect("Failed to parse str as int");
        lanternfish.push(entry);
    }

    // Save the initial state as an immutable variable
    let initial_lanternfish = lanternfish.clone();

    println!("\n---------- Part 1 ----------\n");
    let simulate_days = 80;
    for day in 0..simulate_days {
        for index in 0..lanternfish.len() {
            if lanternfish[index] > 0 {
                lanternfish[index] -= 1;
            } else {
                lanternfish[index] = 6;
                lanternfish.push(8);
            }
        }
        if day == 79 {
            println!("After 80 days there are {} lanternfish", lanternfish.len());
        }
    }

    println!("\n---------- Part 2 ----------\n");
    let mut lanternfish_groups: [usize; 9] = [0; 9];
    for fish in &initial_lanternfish {
        lanternfish_groups[*fish as usize] += 1;
    }

    let simulate_days = 256;
    for day in 1..=simulate_days {
        // Save the age 0 lanternfish before they get overwritten
        let age0 = lanternfish_groups[0];
        // Move all fish one spot down
        for i in 1..9 {
            lanternfish_groups[i - 1] = lanternfish_groups[i];
        }
        lanternfish_groups[6] += age0; // Reset the age 0 fish back to 6
        lanternfish_groups[8] = age0; // Also set as many age 8 fish, overwriting the old value
        if day == 80 {
            {
                let mut total_lanternfish = 0;
                for i in 0..9 {
                    total_lanternfish += lanternfish_groups[i];
                }
                assert_eq!(lanternfish.len(), total_lanternfish);
            }
        }
        if day == 256 {
            let mut total_lanternfish = 0;
            for i in 0..9 {
                println!("Lanternfish with age {}: {}", i, lanternfish_groups[i]);
                total_lanternfish += lanternfish_groups[i];
            }
            println!(
                "For a total of {} lanternfish on day 256",
                total_lanternfish
            );
        }
    }
}
