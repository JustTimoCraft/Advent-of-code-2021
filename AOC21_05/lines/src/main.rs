use std::cmp;
use std::fmt;
use std::fs;

struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)
    }
}

impl Line {
    fn new(coords: [u16; 4]) -> Self {
        Self {
            x1: coords[0],
            y1: coords[1],
            x2: coords[2],
            y2: coords[3],
        }
    }
}

fn main() {
    let filecontents = fs::read_to_string("./../input.txt") // Read the file
        .expect("File read error"); // Throw this message upon error

    let filelines = filecontents
        .split("\n") // Separate the string on every newline character
        .collect::<Vec<&str>>(); // Collect the parts in a new String vector

    let mut linedata: Vec<[u16; 4]> = Vec::new();
    for line in filelines {
        // Skip if line is empty
        if line == "" {
            continue;
        }
        let mut iter = line.split(" -> "); // Split the line through the centre
        let mut coord1 = iter.next().unwrap().split(","); // Split the halves again
        let mut coord2 = iter.next().unwrap().split(",");

        let data: [u16; 4] = [
            coord1
                .next()
                .unwrap()
                .parse::<u16>()
                .expect("Error parsing as int"),
            coord1
                .next()
                .unwrap()
                .parse::<u16>()
                .expect("Error parsing as int"),
            coord2
                .next()
                .unwrap()
                .parse::<u16>()
                .expect("Error parsing as int"),
            coord2
                .next()
                .unwrap()
                .parse::<u16>()
                .expect("Error parsing as int"),
        ];
        linedata.push(data);
    }

    println!("\n---------- Part 1 ----------\n");
    // Add all straight lines to a list
    // Homophobia :(
    let mut the_straights: Vec<Line> = Vec::new();
    for data in &linedata {
        if data[0] == data[2] || data[1] == data[3] {
            the_straights.push(Line::new(*data));
        }
    }
    println!("Considering {} straight lines...", the_straights.len());

    let mut grid: [[u16; 1024]; 1024] = [[0; 1024]; 1024];

    for line in &the_straights {
        let small_x = cmp::min(line.x1, line.x2);
        let large_x = cmp::max(line.x1, line.x2);
        let small_y = cmp::min(line.y1, line.y2);
        let large_y = cmp::max(line.y1, line.y2);
        for i in small_x..=large_x {
            for j in small_y..=large_y {
                grid[i as usize][j as usize] += 1;
            }
        }
    }

    // Count the number of intersections where 2 or more
    // lines cross
    let mut level_count: u128 = 0;
    for i in 0..1024 {
        for j in 0..1024 {
            if grid[i][j] >= 2 {
                level_count += 1;
            }
        }
    }
    println!("Intersections with at least 2 lines: {}", level_count);

    println!("\n---------- Part 2 ----------\n");
    let mut diagonal_lines: Vec<Line> = Vec::new();
    for data in &linedata {
        // If were not dealing with a straight line
        if !(data[0] == data[2] || data[1] == data[3]) {
            // Check if the line is a perfect diagonal
            if (data[0] as i16 - data[2] as i16).abs() == (data[1] as i16 - data[3] as i16).abs() {
                diagonal_lines.push(Line::new(*data));
            } else {
                println!("Found an imperfect diagonal line!");
            }
        }
    }
    println!(
        "Also considering {} diagonal lines for a total of {} lines",
        diagonal_lines.len(),
        diagonal_lines.len() + &the_straights.len()
    );

    for line in diagonal_lines {
        let x_multiplier: i16;
        let y_multiplier: i16;
        if line.x1 < line.x2 {
            x_multiplier = 1;
        } else {
            x_multiplier = -1;
        }
        if line.y1 < line.y2 {
            y_multiplier = 1;
        } else {
            y_multiplier = -1;
        }

        let length = (line.x1 as i16 - line.x2 as i16).abs(); // end_y - start_y should give the same value
        for i in 0..=length {
            grid[(line.x1 as i16 + i * x_multiplier) as usize]
                [(line.y1 as i16 + i * y_multiplier) as usize] += 1;
        }
    }

    // Count the number of intersections where 2 or more
    // lines cross
    let mut level_count: u128 = 0;
    for i in 0..1024 {
        for j in 0..1024 {
            if grid[i][j] >= 2 {
                level_count += 1;
            }
        }
    }
    println!("Intersections with at least 2 lines: {}", level_count);
}
