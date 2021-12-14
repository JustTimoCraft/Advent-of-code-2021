use std::fs;

fn segment_decoder(map: &Vec<&str>, output: &Vec<&str>) -> u16 {
    let mut segments: [char; 7] = ['x'; 7]; // t,tl,tr,m,bl,br,b
    let mut one: &str = "";
    let mut four: &str = "";
    let mut seven: &str = "";
    for segment in map {
        if segment.len() == 2 {
            // Digit with 2 segments is a one
            one = *segment;
        } else if segment.len() == 3 {
            // Digit with 3 segments is a seven
            seven = *segment;
        } else if segment.len() == 4 {
            // Digit with 4 segments is a four
            four = *segment;
        }
    }
    // Check which character does appear in '1', but not in '7'
    // (this is the top segment)
    for character in seven.chars() {
        if !one.contains(character) {
            segments[0] = character;
        }
    }
    // Check for all characters that don't have one of the two
    // segments that make up '1'. This is 2 , 5 and 6. Only 6 has 6 segments
    // This means the missing segment from one is TR
    for segment in map {
        if segment.len() == 6 {
            // Possibilities: 0, 6, 9
            for character in one.chars() {
                if !(segment.contains(character)) {
                    segments[2] = character; // Top right
                    for other_char in one.chars() {
                        if !(other_char == character) {
                            segments[5] = other_char; // Bottom right
                        }
                    }
                }
            }
        }
    }
    // Top, top right and bottom right are known at this point
    // We can figure out the middle segment by looking at 0, 6 and 9
    // Checking which misses a segment that is in '4' but is not Top Right
    for segment in map {
        if segment.len() == 6 {
            // Possibilities: 0, 6, 9
            for character in four.chars() {
                if !segment.contains(character) && character != segments[2] {
                    // If the segment doesn't contain this character
                    // And the missing character is not Top Right
                    segments[3] = character;
                }
            }
        }
    }
    // The only segment we're missing from 4
    for c in four.chars() {
        if c != segments[2] && c != segments[3] && c != segments[5] {
            segments[1] = c;
        }
    }
    // Find bottom left by looking at 9
    for segment in map {
        if segment.len() == 6 {
            // Wow, it's still: 0, 6, 9
            let mut contains_all = true;
            for character in four.chars() {
                if !segment.contains(character) {
                    contains_all = false;
                }
            }
            if contains_all {
                // if it contains all segments in '4', then it's a 9
                for c in "abcdefg".chars() {
                    if !segment.contains(c) {
                        segments[4] = c;
                    }
                }
            }
        }
    }
    // Only the bottom segment needs to be found now
    {
        let combined_segments = String::from_iter(segments.iter());
        for c in "abcdefg".chars() {
            if !combined_segments.contains(c) {
                segments[6] = c;
            }
        }
    }

    // Deciphering the output
    let mut result: String = String::new();
    for digit in output {
        match digit.len() {
            2 => result.push('1'),
            3 => result.push('7'),
            4 => result.push('4'),
            7 => result.push('8'),
            _ => {
                if !digit.contains(segments[3]) {
                    // if the digit misses the middle segment, it has to be a '0'
                    result.push('0');
                } else if !digit.contains(segments[1]) {
                    // 2 or 3
                    if digit.contains(segments[4]) {
                        result.push('2');
                    } else {
                        result.push('3');
                    }
                } else if digit.len() == 6 {
                    if !digit.contains(segments[2]) {
                        result.push('6');
                    } else {
                        result.push('9');
                    }
                } else {
                    result.push('5');
                }
            }
        }
    }

    return result.parse::<u16>().expect("Error parsing int");
}

fn main() {
    let filecontents = fs::read_to_string("./../input.txt").expect("Error while reading file");
    let filelines = filecontents.split("\n").collect::<Vec<&str>>();

    {
        println!("\n---------- Part 1 ----------");
        let mut unique_segments: u16 = 0;
        for line in &filelines {
            if line == &"" {
                continue;
            }
            let output = line.split("|").collect::<Vec<&str>>()[1];
            let digits = output.split(" ").collect::<Vec<&str>>();

            for element in digits {
                match element.len() {
                    2 | 3 | 4 | 7 => unique_segments += 1,
                    _ => (),
                }
            }
        }
        println!("{} 1's, 4's, 7's or 8's found", unique_segments);
    }

    // Test
    {
        let test_map: Vec<&str> = vec![
            "ecgabfd", "gfbe", "dgbeaf", "aeg", "gfbda", "eg", "bgdcaf", "efgdca", "abced", "eadgb",
        ];
        let test_output: Vec<&str> = vec!["begf", "decgfa", "aeg", "eg"];
        assert_eq!(4071, segment_decoder(&test_map, &test_output));
    }

    println!("\n---------- Part 2 ----------\n");
    let mut result: usize = 0;
    for line in &filelines {
        if line == &"" {
            continue;
        }
        let mapping = line.split("|").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>();
        let output = line.split("|").collect::<Vec<&str>>()[1]
            .split(" ")
            .collect::<Vec<&str>>();
        result += segment_decoder(&mapping, &output) as usize;
    }
    println!("Final value: {}", result);
}
