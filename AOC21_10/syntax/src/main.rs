use std::fs;

fn main() {
    let filecontent = fs::read_to_string("./../input.txt").expect("Error while reading file");
    let filelines = filecontent.lines().collect::<Vec<&str>>();
    let mut assholes: Vec<char> = Vec::new();
    let mut scores: Vec<u64> = Vec::new();
    println!("\n---------- Part 1 ----------\n");
    'a: for line in &filelines {
        let mut characters: Vec<char> = Vec::new();
        for c in line.chars() {
            if "[{<(".contains(c) {
                characters.push(c);
            } else if "]}>)".contains(c) {
                let last_char = characters.pop();
                if (last_char == Some('[') && c == ']')
                    || (last_char == Some('{') && c == '}')
                    || (last_char == Some('<') && c == '>')
                    || (last_char == Some('(') && c == ')')
                {
                    continue;
                } else {
                    assholes.push(c);
                    continue 'a; // Holy shit I love Rust
                }
            }
        }
        // Part 2: If it reaches this, the line is incomplete
        let mut total_score = 0;
        loop {
            let next_character = characters.pop();
            if !next_character.is_none() {
                total_score = total_score * 5;
                if next_character == Some('(') {
                    total_score += 1;
                } else if next_character == Some('[') {
                    total_score += 2;
                } else if next_character == Some('{') {
                    total_score += 3;
                } else if next_character == Some('<') {
                    total_score += 4;
                }
            } else {
                scores.push(total_score);
                break;
            }
        }
    }
    println!("Found {} corrupted lines", assholes.len());
    let mut score = 0;
    for ahole in assholes {
        if ahole == ')' {
            score += 3;
        } else if ahole == ']' {
            score += 57;
        } else if ahole == '}' {
            score += 1197;
        } else if ahole == '>' {
            score += 25137;
        }
    }
    println!("Final syntax error score: {}", score);

    println!("\n---------- Part 2 ----------\n");
    scores.sort();
    println!("Middle score: {}", scores[scores.len() / 2]);
}
