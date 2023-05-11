static KNOWN: u16 = 0b1000_0000_0000_0000;
static EMPTY: u16 = 0b1100_0000_0000_0000;

fn coords_from_ind(ind: usize) -> (usize, usize) {
    let j = ind % 9;
    let i = (ind - j) / 9;
    return (i, j)
}

fn block_num_from_coords(i: usize, j: usize) -> usize {
    return (i/3)*3 + j/3
}

#[derive(Debug, Clone)]
struct Sudoku {
    m: [u16; 81],
    cells: [u16; 81],
    // rows: [u16; 9],
    // cols: [u16; 9],
    // blocks: [u16; 9],
    mins: u128,
    min_num: u16,
}

impl Sudoku {
    fn create(m: [u16; 81]) -> Self {
        let mut cells: [u16; 81] = [0; 81];
        // let mut nums: [u16; 81] = [9; 81];
        let mut mins: u128 = 0;
        let mut rows: [u16; 9] = [0; 9];
        let mut cols: [u16; 9] = [0; 9];
        let mut blocks: [u16; 9] = [0; 9];
        let mut num_solved = 0;

        for i in 0..81 {
            // Set all positive values to known
            if m[i] > 0 {
                cells[i] = KNOWN;
                let (x, y) = coords_from_ind(i);
                let z = block_num_from_coords(x, y);
                // OR the rows/cols/blocks with the value represented as a
                // bit shift. e.g. rows[0] = 0b0000_0000_0100_1000 means that a
                // 3 and a 6 are present in row 0.
                let shift = 1 << m[i];
                rows[x] |= shift;
                cols[y] |= shift;
                blocks[z] |= shift;
            } else {
                // First two bits describe whether the cell is
                // known or empty. The rest of the ones represent
                // potential candidates.
                cells[i] = 0b1100_0011_1111_1110;
            }
        }

        let mut min_num: u16 = 10;
        for i in 0..81 {
            let (x, y) = coords_from_ind(i);
            let z = block_num_from_coords(x, y);
            // "Subtract" the values from the rows, cols, and blocks
            // from each cell.
            // Ex.        If cells[0] = 0b1100_0000_0111_1000, then candidates are
            // 3, 4, 5, 6. If rows[0] = 0b0000_0000_1101_0000, then
            // cells[0] becomes         0b1100_0000_0010_1000 because the row
            // contained 4, 6, and 7.
            cells[i] ^= cells[i] & rows[x];
            cells[i] ^= cells[i] & cols[y];
            cells[i] ^= cells[i] & blocks[z];
            if cells[i] == KNOWN {
                continue;
            } else if cells[i] == EMPTY {
                panic!("Puzzle is unsolvable.");
            } else {
                let curr_num = (cells[i].count_ones() - 1) as u16;
                if curr_num < min_num {
                    min_num = curr_num;
                    mins = 1 << i;
                } else if curr_num == min_num {
                    mins |= 1 << i;
                }
            }
        }

        Self {
            m,
            cells,
            // rows,
            // cols,
            // blocks,
            // nums,
            mins,
            min_num,
        }
    }

    fn get_mins(&mut self) {
        self.min_num = 10;
        for i in 0..81 {
            if (((self.cells[i].count_ones() - 1) as u16) < self.min_num) && (self.cells[i] != KNOWN) {
                self.min_num = (self.cells[i].count_ones() - 2) as u16;
                self.mins = 1 << i;
            } else if ((self.cells[i].count_ones() - 1) as u16) == self.min_num && self.cells[i] != KNOWN {
                self.mins |= 1 << i;
            }
        }
    }

    fn update(&mut self, i: usize, val: u16) {
        self.cells[i] = KNOWN;
        let shift = 1 << val;
        let (x, y) = coords_from_ind(i);
        let z = block_num_from_coords(x, y);
        self.m[i] = val;
        
        for j in 0..81 {
            let (x2, y2) = coords_from_ind(j);
            let z2 = block_num_from_coords(x2, y2);
            let old_val = self.cells[j];
            if x == x2 || y == y2 || z == z2 && old_val != KNOWN {
                // "Subtract" value from each of the cells in corresponding
                // rows/cols/blocks
                self.cells[j] ^= self.cells[j] & shift;
            }
            if self.cells[j] == EMPTY {
                self.min_num = 0;
                return ()
            }
        }
        
        self.get_mins();
    }
}

// Print the object nicely
impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "".to_string();
        let top_line = format!("{}{}{}", "┌", "───┬".repeat(8), "───┐\n");
        let mid_line = format!("{}{}{}", "│", "───┼".repeat(8), "───┤\n");
        let bot_line = format!("{}{}{}", "└", "───┴".repeat(8), "───┘");
        s.push_str(&top_line[..]);
        for i in 0..81 {
            if i % 9 == 0 {
                s.push_str("|");
            }
            s = format!("{} {} {}", s, self.m[i], "│");
            if i % 9 == 8 {
                s.push_str("\n");
            }
            if i % 9 == 8 && i != 80 {
                s.push_str(&mid_line[..]);
            }
        }

        s.push_str(&bot_line[..]);
        write!(f, "{}", s)
    }
}

fn solve(mut s: Sudoku) -> Option<Sudoku> {
    // Eliminate all the cells with only one guess
    while s.min_num == 1 {
        let ind: usize = s.mins.trailing_zeros() as usize; 
        s.update(ind, s.cells[ind].trailing_zeros() as u16);
    }

    if s.min_num > 0 && s.min_num != 10 {
        let ind: usize = s.mins.trailing_zeros() as usize;
        let mut test_cell = s.cells[ind];
        let n = test_cell.count_ones() - 1;
        
        for _ in 0..n {
            let mut q = s.clone();
            let val: u16 = test_cell.trailing_zeros() as u16;
            test_cell ^= test_cell & (1 << val);
            q.update(ind, val);
            let res = solve(q);
            let mut is_none = false;
            match res {
                None => {is_none = true;}
                _ => {}
            }
            if !is_none {
                return res
            }
        }
        None
    } else if s.min_num == 0 {
        None
    } else {
        Some(s)
    }
}

fn solve_puzzle(puzzle: [u16; 81]) -> Sudoku {
    let s = Sudoku::create(puzzle);
    let res = solve(s);
    return res.unwrap();
}

fn main() {
    // let puzzle = [
    //     8, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 3, 6, 0, 0, 0, 0, 0,
    //     0, 7, 0, 0, 9, 0, 2, 0, 0,
    //     0, 5, 0, 0, 0, 7, 0, 0, 0,
    //     0, 0, 0, 0, 4, 5, 7, 0, 0,
    //     0, 0, 0, 1, 0, 0, 0, 3, 0,
    //     0, 0, 1, 0, 0, 0, 0, 6, 8,
    //     0, 0, 8, 5, 0, 0, 0, 1, 0,
    //     0, 9, 0, 0, 0, 0, 4, 0, 0,
    // ]; // Everest puzzle
       let puzzle = [
           1, 0, 0, 0, 0, 7, 0, 9, 0,
           0, 3, 0, 0, 2, 0, 0, 0, 8,
           0, 0, 9, 6, 0, 0, 5, 0, 0,
           0, 0, 5, 3, 0, 0, 9, 0, 0,
           0, 1, 0, 0, 8, 0, 0, 0, 2,
           6, 0, 0, 0, 0, 4, 0, 0, 0,
           3, 0, 0, 0, 0, 0, 0, 1, 0,
           0, 4, 0, 0, 0, 0, 0, 0, 7,
           0, 0, 7, 0, 0, 0, 3, 0, 0,
       ]; // Al Escargot puzzle
       // let puzzle = [
       //     0, 2, 0, 0, 3, 0, 0, 4, 0,
       //     6, 0, 0, 0, 0, 0, 0, 0, 3,
       //     0, 0, 4, 0, 0, 0, 5, 0, 0,
       //     0, 0, 0, 8, 0, 6, 0, 0, 0,
       //     8, 0, 0, 0, 1, 0, 0, 0, 6,
       //     0, 0, 0, 7, 0, 5, 0, 0, 0,
       //     0, 0, 7, 0, 0, 0, 6, 0, 0,
       //     4, 0, 0, 0, 0, 0, 0, 0, 8,
       //     0, 3, 0, 0, 4, 0, 0, 2, 0,
       // ]; // https://www.mathworks.com/company/newsletters/articles/solving-sudoku-with-matlab.html

    // let mut sol = Sudoku::create(puzzle);
    let mut sol = Sudoku::create(puzzle);
    println!("{}", sol);
    //println!("{:b}", sol.cells);
    
    use std::time::Instant;
    let now = Instant::now();
    let n = 10;
    for _ in 0..n {
        sol = solve_puzzle(puzzle);
        // println!("num updates is {}", sol.num_updates);
    }
    println!("{}", sol);
    let elapsed = now.elapsed();
    println!(
        "Avg time to solve puzzle over {} iterations: {:.2?}",
        n,
        elapsed / n
    )
    
}
