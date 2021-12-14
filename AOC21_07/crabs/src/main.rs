use std::fs;

fn calc_fuel(positions: &Vec<u16>, aimed_height: u16) -> u64 {
    let mut fuel: u64 = 0;
    for &crab in positions {
        fuel += (crab as i16 - aimed_height as i16).abs() as u64;
    }
    return fuel;
}

fn calc_fuel_exp(positions: &Vec<u16>, aimed_height: u16) -> u64 {
    let mut fuel: u64 = 0;
    let mut movement: u64;
    for &crab in positions {
        movement = (crab as i16 - aimed_height as i16).abs() as u64;
        fuel += movement * (movement + 1) / 2;
    }
    fuel
}

fn tests() {
    let testinput: [u16; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let testvec: Vec<u16> = Vec::from_iter(testinput.into_iter());
    assert_eq!(calc_fuel(&testvec, 2), 37);
    assert_eq!(calc_fuel_exp(&testvec, 5), 168);
}

fn main() {
    // Run debug tests
    tests();
    let mut positions: Vec<u16> = Vec::new();
    {
        let filecontents = fs::read_to_string("./../input.txt").expect("Error while reading file");
        let str_pos = filecontents
            .split("\n")
            .next()
            .unwrap()
            .split(",")
            .collect::<Vec<&str>>();
        for string in str_pos {
            positions.push(string.parse::<u16>().expect("Error parsing int"));
        }
    }
    let mut avg: i64 = 0;
    for pos in &positions {
        avg += *pos as i64;
    }
    avg = avg / positions.len() as i64;
    println!("Average position {}", avg);

    println!("\n---------- Part 1 ----------\n");
    let mut lowest_cost: u64 = u64::MAX;
    let mut tests: i64 = 0;
    loop {
        let cost: u64 = calc_fuel(&positions, (avg - tests) as u16);
        if cost < lowest_cost {
            lowest_cost = cost;
        }
        if avg - tests == 0 {
            break;
        }
        if cost > (10 * lowest_cost) {
            break;
        }
        let cost = calc_fuel(&positions, (avg + tests) as u16);
        if cost < lowest_cost {
            lowest_cost = cost;
        }
        tests += 1;
        print!("\rTests completed: {}    ", tests);
    }
    println!("\nLowest cost: {}", lowest_cost);

    println!("\n---------- Part 2 ----------\n");
    lowest_cost = u64::MAX;
    tests = 0;
    loop {
        let cost: u64 = calc_fuel_exp(&positions, (avg - tests) as u16);
        if cost < lowest_cost {
            lowest_cost = cost;
        }
        if avg - tests == 0 {
            break;
        }
        if cost > (10 * lowest_cost) {
            break;
        }
        let cost = calc_fuel_exp(&positions, (avg + tests) as u16);
        if cost < lowest_cost {
            lowest_cost = cost;
        }
        tests += 1;
        print!("\rTests completed: {}    ", tests);
    }
    println!("\nLowest cost: {}", lowest_cost);
}
