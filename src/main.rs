static KNOWN: u16 = 0b1000_0000_0000_0000;

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
    k: [bool; 81],
    cells: [u16; 81],
    rows: [u16; 9],
    cols: [u16; 9],
    blocks: [u16; 9],
    mins: u128,
    min_num: u16,
    num_solved: u8,
}

impl Sudoku {
    fn create(mut m: [u16; 81]) -> Self {
        let mut k = [false; 81];
        let mut cells: [u16; 81] = [0; 81];
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
                let shift = 1 << m[i];
                rows[x] |= shift;
                cols[y] |= shift;
                blocks[z] |= shift;
                num_solved += 1;
            } else {
                cells[i] = 0b1000_0011_1111_1110;
            }
        }

        let mut min_num: u16 = 10;
        for i in 0..81 {
            let (x, y) = coords_from_ind(i);
            let z = block_num_from_coords(x, y);
            cells[i] ^= cells[i] & rows[x];
            cells[i] ^= cells[i] & cols[y];
            cells[i] ^= cells[i] & blocks[z];
            let num_ones = cells[i].count_ones();
            if num_ones == 2 {
                m[i] = cells[i].trailing_zeros() as u16;
                cells[i] = KNOWN;
                num_solved += 1;
                println!("setting cell to known");
            } else if num_ones == 1 {
                continue;
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
        let mut count_solved = 0;
        for i in 0..81 {
            if cells[i].count_ones() == 10 {
                count_solved += 1;
            }
        }
        println!("count_solved: {}", count_solved);
        println!("num_solved: {}", num_solved);

        for i in 0..81 {
            println!("cell: {:b}, num cands: {}, coords: {:?}", cells[i], cells[i].count_ones() - 1, coords_from_ind(i));
        }
        let ind = (mins.trailing_zeros()) as usize;
        let (x, y) = coords_from_ind(ind);
        println!("min coords are {}, {}", x, y);
        println!("first block is {:b}", blocks[0]);
        println!("second col is {:b}", cols[1]);
        println!("second row is {:b}", rows[1]);
        Self {
            m,
            k,
            cells,
            rows,
            cols,
            blocks,
            mins,
            min_num,
            num_solved,
        }
    }

    fn update(&mut self, i: usize, val: u16) {
        self.cells[i] = KNOWN;
        let shift = 1 << val;
        let (x, y) = coords_from_ind(i);
        let z = block_num_from_coords(x, y);
        println!("Filling in ({},{}) with value {}", x, y, val);
        self.rows[x] |= shift;
        self.cols[y] |= shift;
        self.blocks[z] |= shift;
        self.m[i] = val;
        self.num_solved += 1;
        
        self.min_num = 10;
        println!("Mins before shift: {:b}", self.mins);
        self.mins ^= self.mins & shift as u128;
        println!("Mins after shift:  {:b}", self.mins);
        for j in 0..81 {
            let (x2, y2) = coords_from_ind(j);
            let z2 = block_num_from_coords(x2, y2);
            let old_val = self.cells[j];
            if x == x2 || y == y2 || z == z2 {
                self.cells[j] ^= self.cells[j] & self.rows[x2];
                self.cells[j] ^= self.cells[j] & self.cols[y2];
                self.cells[j] ^= self.cells[j] & self.blocks[z2];
            }
            if self.cells[j].count_ones() == 1 && old_val != self.cells[j] {
                self.min_num = 0;
                println!("\n\n ---- Breaking ---- \n\n");
                return ();
            }
            let num_ones = self.cells[j].count_ones();
            if num_ones != 1 {
                let curr_num = (self.cells[j].count_ones() - 1) as u16;
                if curr_num < self.min_num {
                    self.min_num = curr_num;
                    self.mins = 1 << j;
                } else if curr_num == self.min_num {
                    self.mins |= 1 << j;
                }
            }
        }

        // for i in 0..81 {
        //     println!("cell: {:b}, num cands: {}, coords: {:?}", self.cells[i], self.cells[i].count_ones() - 1, coords_from_ind(i));
        // }
    }
}

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
    // while s.min_num == 1 {
    //     let ind: usize = s.mins.trailing_zeros() as usize; 
    //     s.update(ind, s.cells[ind].trailing_zeros() as u16);
    //     println!("update inside s min num 1");
    // }
    println!("{}", s);
    if s.min_num > 0 && s.min_num != 10 {
        let ind: usize = s.mins.trailing_zeros() as usize;
        println!("mins before: {:b}", s.mins);
        s.mins ^= s.mins & (1 << ind);
        println!("mins after: {:b}", s.mins);
        let mut q = s.clone();
        println!("mins are {:b}", s.mins);
        println!("ind is {}", ind);
        let mut test_cell = s.cells[ind];
        println!("cell is {:b}", test_cell);
        let n = test_cell.count_ones() - 1;
        
        for _ in 0..n {
            let val: u16 = test_cell.trailing_zeros() as u16;
            test_cell ^= test_cell & (1 << val);
            println!("cell changed to {:b}", test_cell);
            q.update(ind, val);
            let res = solve(q.clone());
            return res
        }
        println!("returning none");
        None
    } else if s.min_num == 0 {
        println!("min num is 0");
        None
    } else {
        println!("No idea");
        Some(s)
    }
}

fn solve_puzzle(puzzle: [u16; 81]) -> Sudoku {
    let s = Sudoku::create(puzzle);
    let res = solve(s);
    println!("{:?}", res);
    return res.unwrap();
}

fn main() {
    println!("{}", block_num_from_coords(6, 6));
    let mut res: u16 = 0;
    res |= 1 << 9;
    res |= 1 << 7;
    let mut x: u16 = 0b1000_0010_0000_0000;
    println!("{}", x.trailing_zeros());
    println!("{}", x.count_ones());
    // let y = 0b0000_0001_1001_0000;
    // let z = 0b0000_0000_1001_0000;
    // x ^= x & y;
    // x ^= x & z;
    // println!("{:b}", x);
    
    let puzzle = [
        8, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 3, 6, 0, 0, 0, 0, 0,
        0, 7, 0, 0, 9, 0, 2, 0, 0,
        0, 5, 0, 0, 0, 7, 0, 0, 0,
        0, 0, 0, 0, 4, 5, 7, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 3, 0,
        0, 0, 1, 0, 0, 0, 0, 6, 8,
        0, 0, 8, 5, 0, 0, 0, 1, 0,
        0, 9, 0, 0, 0, 0, 4, 0, 0,
    ]; // Everest puzzle
       // let puzzle = [
       //     1, 0, 0, 0, 0, 7, 0, 9, 0,
       //     0, 3, 0, 0, 2, 0, 0, 0, 8,
       //     0, 0, 9, 6, 0, 0, 5, 0, 0,
       //     0, 0, 5, 3, 0, 0, 9, 0, 0,
       //     0, 1, 0, 0, 8, 0, 0, 0, 2,
       //     6, 0, 0, 0, 0, 4, 0, 0, 0,
       //     3, 0, 0, 0, 0, 0, 0, 1, 0,
       //     0, 4, 0, 0, 0, 0, 0, 0, 7,
       //     0, 0, 7, 0, 0, 0, 3, 0, 0,
       // ]; // Al Escargot puzzle
       // let puzzle = [
       //     [0, 2, 0, 0, 3, 0, 0, 4, 0],
       //     [6, 0, 0, 0, 0, 0, 0, 0, 3],
       //     [0, 0, 4, 0, 0, 0, 5, 0, 0],
       //     [0, 0, 0, 8, 0, 6, 0, 0, 0],
       //     [8, 0, 0, 0, 1, 0, 0, 0, 6],
       //     [0, 0, 0, 7, 0, 5, 0, 0, 0],
       //     [0, 0, 7, 0, 0, 0, 6, 0, 0],
       //     [4, 0, 0, 0, 0, 0, 0, 0, 8],
       //     [0, 3, 0, 0, 4, 0, 0, 2, 0],
       // ]; // https://www.mathworks.com/company/newsletters/articles/solving-sudoku-with-matlab.html

    // let mut sol = Sudoku::create(puzzle);
    let sol = solve_puzzle(puzzle);
    println!("{}", sol);
    //println!("{:b}", sol.cells);
    /*
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
    */
}
