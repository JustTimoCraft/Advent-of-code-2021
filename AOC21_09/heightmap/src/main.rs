use std::fs;

fn basin_size(grid: &[[u8; 100]; 100], coord: &[u8; 2]) -> usize {
    let mut basin_coords: Vec<[u8; 2]> = Vec::new();
    let mut new_coords: Vec<[u8; 2]> = vec![*coord];
    loop {
        // if there are no new coords to be checked
        if new_coords.len() == 0 {
            break;
        }
        //let mut current_new_coords_amount = new_coords.len();
        let mut new_new_coords: Vec<[u8; 2]> = Vec::new();
        for coord in &new_coords {
            basin_coords.push([coord[0], coord[1]]);
            if coord[0] != 0 {
                // if the cell above the coord is not a 9
                // AND if said cell is not already in basin_coords
                if grid[(coord[0] - 1) as usize][coord[1] as usize] != 9
                    && !basin_coords.contains(&[coord[0] - 1, coord[1]])
                {
                    new_new_coords.push([coord[0] - 1, coord[1]]);
                }
            }
            if coord[0] != 99 {
                // if the cell above the coord is not a 9
                // AND if said cell is not already in basin_coords
                if grid[(coord[0] + 1) as usize][coord[1] as usize] != 9
                    && !basin_coords.contains(&[coord[0] + 1, coord[1]])
                {
                    new_new_coords.push([coord[0] + 1, coord[1]]);
                }
            }
            if coord[1] != 0 {
                // if the cell above the coord is not a 9
                // AND if said cell is not already in basin_coords
                if grid[coord[0] as usize][(coord[1] - 1) as usize] != 9
                    && !basin_coords.contains(&[coord[0], coord[1] - 1])
                {
                    new_new_coords.push([coord[0], coord[1] - 1]);
                }
            }
            if coord[1] != 99 {
                // if the cell above the coord is not a 9
                // AND if said cell is not already in basin_coords
                if grid[coord[0] as usize][(coord[1] + 1) as usize] != 9
                    && !basin_coords.contains(&[coord[0], coord[1] + 1])
                {
                    new_new_coords.push([coord[0], coord[1] + 1]);
                }
            }
        }
        new_coords = new_new_coords;
        //new_coords = new_coords.split_off(current_new_coords_amount)[1];
    }
    basin_coords.sort();
    basin_coords.dedup();
    return basin_coords.len();
}

fn tests() {
    let mut test_grid: [[u8; 100]; 100] = [[0; 100]; 100];
    let heights: [[u8; 11]; 6] = [
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 9],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2, 9],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8, 9],
        [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
    ];
    for i in 0..6 {
        for j in 0..11 {
            test_grid[i][j] = heights[i][j];
        }
    }
    assert_eq!(3, basin_size(&test_grid, &[0, 1]));
    assert_eq!(9, basin_size(&test_grid, &[0, 9]));
    assert_eq!(9, basin_size(&test_grid, &[4, 6]));
    assert_eq!(14, basin_size(&test_grid, &[2, 2]));
}

fn main() {
    tests();

    let filecontents = fs::read_to_string("./../input.txt").expect("Error while reading file");
    let filelines = filecontents.lines().collect::<Vec<&str>>();
    let mut grid: [[u8; 100]; 100] = [[0; 100]; 100];
    for (index, line) in filelines.iter().enumerate() {
        for (c_index, character) in line.chars().enumerate() {
            grid[index][c_index] = character.to_digit(10).unwrap() as u8;
        }
    }
    println!("\n---------- Part 1 ----------\n");
    let mut current_value: &u8;
    let mut lowest: bool;
    let mut risk_level: usize = 0;
    let mut lowcations: Vec<[u8; 2]> = Vec::new(); // For Part 2
    for i in 0..100 {
        for j in 0..100 {
            current_value = &grid[i][j];
            lowest = true;
            if i != 0 {
                if &grid[i - 1][j] <= current_value {
                    lowest = false;
                }
            }
            if i != 99 {
                if &grid[i + 1][j] <= current_value {
                    lowest = false;
                }
            }
            if j != 0 {
                if &grid[i][j - 1] <= current_value {
                    lowest = false;
                }
            }
            if j != 99 {
                if &grid[i][j + 1] <= current_value {
                    lowest = false;
                }
            }

            if lowest {
                risk_level += (1 + grid[i][j]) as usize;
                lowcations.push([i as u8, j as u8]);
            }
        }
    }
    println!("Risk level: {}", risk_level);

    println!("\n---------- Part 2 ----------\n");
    // Instead of going through each and every point, we choose
    // The points we found in part 1
    let mut largest_basins: [usize; 4] = [1; 4];
    for height in lowcations {
        largest_basins[0] = basin_size(&grid, &height);
        largest_basins.sort();
    }
    println!(
        "Basins: 1) {} | 2) {} | 3) {}",
        largest_basins[1], largest_basins[2], largest_basins[3]
    );
    println!(
        "Answer: {}",
        largest_basins[1] * largest_basins[2] * largest_basins[3]
    );
}
