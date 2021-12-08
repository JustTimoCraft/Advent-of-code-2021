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

    // Add all straight lines to a list
    // Homophobia :(
    let mut the_straights: Vec<Line> = Vec::new();
    for data in linedata {
        if data[0] == data[2] || data[1] == data[3] {
            the_straights.push(Line::new(data));
        }
    }

    let mut grid: [[u16; 1024]; 1024] = [[0; 1024]; 1024];

    for line in the_straights {
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

    // Stack overflow occurs here somehow?????
    let mut level_count: u128 = 0;
    for line in grid {
        for field in line {
            if field >= 2 {
                level_count += 1;
            }
        }
    }
    println!("Values above 1: {}", level_count);
}
