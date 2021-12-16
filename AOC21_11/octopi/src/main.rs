fn main() {
    // TESTING
    /*const TEST_START_VALUES: [[u8; 10]; 10] = [
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];
    const TEST_1STEP_SOLUTION: [[u8; 10]; 10] = [
        [6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
        [3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
        [6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
        [7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
        [7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
        [5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
        [3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
        [7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
        [5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
        [6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
    ];
    const TEST_10STEP_SOLUTION: [[u8; 10]; 10] = [
        [0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
        [0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
        [0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
        [0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
        [0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
        [0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
        [0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
        [5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
        [0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
        [0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
    ];*/

    //let start_values = TEST_START_VALUES;
    let start_values: [[u8; 10]; 10] = [
        [2, 3, 4, 4, 6, 7, 1, 2, 1, 2],
        [6, 6, 1, 1, 7, 4, 2, 6, 8, 1],
        [5, 5, 7, 5, 5, 7, 5, 5, 7, 3],
        [3, 1, 6, 7, 8, 4, 8, 5, 3, 6],
        [1, 3, 5, 3, 8, 2, 7, 3, 1, 1],
        [4, 4, 1, 6, 4, 6, 3, 2, 6, 6],
        [2, 6, 2, 4, 7, 6, 1, 6, 1, 5],
        [1, 7, 8, 6, 5, 6, 1, 2, 6, 3],
        [3, 6, 2, 2, 6, 4, 3, 2, 1, 5],
        [4, 1, 4, 3, 2, 8, 4, 6, 5, 3],
    ];

    let mut current_values: [[u8; 10]; 10] = start_values;
    let mut flashes: u32 = 0;
    let mut flashes_step100: u32 = 0;
    let mut steps = 0;
    let moves: [[i8; 2]; 8] = [
        [-1, -1], // Top left
        [-1, 0],  // Top centre
        [-1, 1],  // Top right
        [0, -1],  // Centre left
        [0, 1],   // Centre right
        [1, -1],  // Bottom left
        [1, 0],   // Bottom centre
        [1, 1],   // Bottom right
    ];

    'outerloop: loop {
        steps += 1;
        let mut flashed: [[bool; 10]; 10] = [[false; 10]; 10];
        // Add one to every field
        current_values = current_values.map(|arr| arr.map(|elem| elem + 1));
        'steploop: loop {
            let mut flashed_this_loop = 0;

            for i in 0..10 {
                for j in 0..10 {
                    if current_values[i][j] > 9 && !flashed[i][j] {
                        // Octopus flashes
                        flashed[i][j] = true;
                        flashes += 1;
                        flashed_this_loop += 1;
                        for movement in &moves {
                            if !(i as i8 + movement[0] < 0
                                || i as i8 + movement[0] > 9
                                || j as i8 + movement[1] < 0
                                || j as i8 + movement[1] > 9)
                            {
                                // Increment the value of a adjacent cell by one
                                current_values[((i as i8) + movement[0]) as usize]
                                    [((j as i8) + movement[1]) as usize] += 1; // I'm sorry...
                            }
                        }
                    }
                }
            }

            // if nothing happened this loop, this step is done
            if flashed_this_loop == 0 {
                // Reset values above 9 back to 0
                current_values =
                    current_values.map(|arr| arr.map(|elem| if elem > 9 { 0 } else { elem }));
                break 'steploop;
            }
        }

        let mut flashcount: u32 = 0;
        for i in 0..10 {
            for j in 0..10 {
                if flashed[i][j] {
                    flashcount += 1;
                }
            }
        }
        if flashcount == 100 {
            break 'outerloop;
        }

        if steps == 100 {
            flashes_step100 = flashes;
        }
    }
    // TESTING
    //assert_eq!(TEST_1STEP_SOLUTION, current_values);
    //assert_eq!(204, flashes);
    //assert_eq!(TEST_10STEP_SOLUTION, current_values);

    println!("Number of flashes after 100 steps: {:?}", flashes_step100);

    println!("\n---------- Part 2 ----------\n");
    println!("Synchronised flash after {} steps", steps);
}
