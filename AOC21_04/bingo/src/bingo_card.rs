use std::fmt;

pub struct BingoCard {
    fields: [u8; 25],
    active: [bool; 25],
    bingo_direction: char,
    bingo_start_coord: usize,
}

// Add pretty-print functionality for the bingoCard
impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bingo Card 5x5\n").expect("Error in formatter");
        for index in 0..25 {
            write!(f, "{:2}", self.fields[index]).expect("Error in formatter"); // Write value in field
            if self.active[index] {
                write!(f, "* ").expect("Error in formatter"); // Write '* ' if the field is active
            } else {
                write!(f, "  ").expect("Error in formatter"); // Write '  ' if not
            }
            if (index + 1) % 5 == 0 {
                write!(f, "\n").expect("Error in formatter"); // Write a newline at the end of a line (duh)
            }
        }
        if self.bingo_direction != 'X' {
            write!(f, "This card has bingo! ").expect("Error in formatter");
        }
        if self.bingo_direction == 'H' {
            write!(f, "Horizontally, on line {}", self.bingo_start_coord + 1)
        } else if self.bingo_direction == 'V' {
            write!(f, "Vertically, on line {}", self.bingo_start_coord + 1)
        } else if self.bingo_direction == 'D' {
            // if bingo_direction == 'D'
            if self.bingo_start_coord == 0 {
                write!(f, "Diagonally, from top left to bottom right")
            } else {
                write!(f, "Diagonally, from top right to bottom left")
            }
        } else {
            write!(f, "")
        }
    }
}

impl BingoCard {
    pub fn new(fields: [u8; 25]) -> Self {
        Self {
            fields,
            active: [false; 25],
            bingo_direction: 'X',
            bingo_start_coord: 0,
        }
    }

    pub fn check_for_bingo(&mut self) -> bool {
        // There are 5 horizontal lines and 5 vertical lines to get bingo with
        for i in 0..5 {
            let mut horizontal_match = true;
            let mut vertical_match = true;
            // And each line has 5 different spots to check
            for j in 0..5 {
                // Check horizontal line
                if !self.active[i * 5 + j] {
                    horizontal_match = false;
                }
                // Check vertical line
                if !self.active[i + j * 5] {
                    vertical_match = false;
                }
            }
            if horizontal_match {
                self.bingo_direction = 'H';
                self.bingo_start_coord = i;
                return true;
            } else if vertical_match {
                self.bingo_direction = 'V';
                self.bingo_start_coord = i;
                return true;
            }
        }
        // There are also two diagonal lines to consider
        let mut tl_bingo = true;
        let mut tr_bingo = true;
        for i in 0..5 {
            // Top left to bottom right
            if !self.active[i * 6] {
                tl_bingo = false;
            }
            // Top right to bottom left
            if !self.active[4 * i + 4] {
                tr_bingo = false;
            }
        }
        if tl_bingo {
            self.bingo_direction = 'D';
            self.bingo_start_coord = 0;
            return true;
        }
        if tr_bingo {
            self.bingo_direction = 'D';
            self.bingo_start_coord = 4;
            return true;
        }
        // If no bingo was found
        return false;
    }

    pub fn activate_field(&mut self, value: u8) {
        for index in 0..25 {
            if self.fields[index] == value {
                self.active[index] = true;
            }
        }
    }

    pub fn reset(&mut self) {
        self.active = [false; 25];
        self.bingo_direction = 'X';
        self.bingo_start_coord = 0;
    }

    pub fn calculate_score(&mut self, last_number: u8) -> usize {
        if self.bingo_direction == 'X' {
            if !self.check_for_bingo() {
                return 0; // If there is no bingo, return a score of 0
            }
        }
        // Otherwise, if there is bingo, check for it
        let mut non_active_sum: usize = 0;
        for i in 0..25 {
            if !self.active[i] {
                non_active_sum += self.fields[i] as usize;
            }
        }
        return non_active_sum * last_number as usize;
    }
}
