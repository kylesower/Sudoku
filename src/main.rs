use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;


fn get_num_nonzero(&vals: &[i8; 9]) -> i8 {
    let mut count = 0;
    for val in vals {
        if val >= 0 {
            count += 1;
        }
    }
    count
}

#[derive(Debug, Clone)]
struct Sudoku {
    m: [[i8; 9]; 9],
    k: [[bool; 9]; 9],
    c: [[[i8; 9]; 9]; 9],
    n: [[bool; 81]; 10],
    l: i8,
    num_solved: u8,
    num_updates: u16,
}

impl Sudoku {
    fn create(m: [[i8; 9]; 9]) -> Self {
        let mut k = [[false; 9]; 9];
        let mut c = [[[-1; 9]; 9]; 9];
        let mut num_solved = 0;
        for item in m.iter().enumerate() {
            let (i, row): (usize, &[i8; 9]) = item;
            for square in row.iter().enumerate() {
                let (j, number): (usize, &i8) = square;
                if *number > 0 {
                    k[i][j] = true;
                    num_solved += 1;
                } else {
                    for num in 0..9 {
                        c[i][j][num as usize] = num;
                    }
                }
            }
        }
        for i in 0..9 {
            for j in 0..9 {
                for x in 0..9 {
                    for y in 0..9 {
                        if i == x || j == y || (i / 3 == x / 3 && j / 3 == y / 3) {
                            let num_to_remove = m[x][y] - 1;
                            if k[x][y] {
                               c[i][j][num_to_remove as usize] = -1;
                            }
                        }
                    }
                }
            }
        }
        // println!("initial c is {:?}", c);

        let n = Self::get_n(&c);
        let l = Self::get_l(&n);
        // println!("initial c: {:?}", c);
        // println!("initial k: {:?}", k);
        let num_updates = 0;
        Self { m, k, c, n, l, num_solved, num_updates}
    }

    fn get_n(&c: &[[[i8; 9]; 9]; 9]) -> [[bool; 81]; 10] {
        let mut n = [[false; 81]; 10];
        for i in 0..9 {
            for j in 0..9 {
                let num = get_num_nonzero(&c[i][j]);
                // println!("num nonzero guesses at {},{} is {}", i, j, num);
                n[num as usize][(i*9 + j) as usize] = true;
            }
        }
        // println!("n is {:?}", n);
        n
    }

    fn get_l(&n: &[[bool; 81]; 10]) -> i8 {
        for num_can in 1..10 {
            for i in 0..81 {
                if n[num_can][i]{
                    // println!("l from get_l is {}", num_can);
                    return num_can as i8;
                }
            }
        }

        11
    }

    fn clear_candidates(&mut self, i: usize, j: usize) {
        for k in 0..9 {
            self.c[i][j][k] = -1;
        }
    }

    fn clear_candidate_val(&mut self, i: usize, j:usize, val: i8) -> bool {
        let old_val = self.c[i][j][val as usize];
        if old_val == -1 {
            return false
        }
        self.c[i][j][val as usize] = -1;
        true
    }

    fn update(&mut self, i: usize, j: usize, val: i8) {
        self.clear_candidates(i, j);
        self.num_updates += 1;
        self.k[i][j] = true;
        self.num_solved += 1;

        // println!("num_solved: {}", self.num_solved);
        self.m[i][j] = val + 1;
        // if self.num_solved == 81 {
        //     self.l = 0;
        //     return ()
        // }
        for n in 0..9 {
            for m in 0..9 {
                let row = i == n;
                let col = j == m;
                let block = i/3 == n/3 && j/3 == m/3;
                if row || col || block {
                    let guess_changed = self.clear_candidate_val(n,m,val);
                    let num = get_num_nonzero(&self.c[n][m]);
                    if num == 0 && guess_changed{
                        self.l = 0;
                        return ()
                    }
                }
            }
        }

        self.l = Self::get_l(& self.n);
    }

    fn get_first_nonzero_guess(&mut self, i: usize, j: usize) -> i8 {
        let vals = self.c[i][j];
        for val in vals {
            if val > -1 {
                // self.clear_candidate_val(i, j, val);
                return val;
            }
        }

        -1
    }

    fn get_rand_nonzero_guess(&mut self, i: usize, j: usize) -> i8 {
        let vals = self.c[i][j];

        for val in vals {
            if val > -1 {
                // self.clear_candidate_val(i, j, val);
                return val;
            }
        }

        -1
    }

    fn get_next_coordinates(&mut self) -> (usize, usize){
        let possible = self.n[self.l as usize];
        for cell in 0..81 {
            if possible[cell] {
                let j = cell % 9;
                let i = (cell - j) / 9;
                return (i, j)
            }
        }
        // println!("possible cells are {:?}", possible);

        (82, 82)
    }

}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        let top_line = format!("{}{}{}", "┌", "───┬".repeat(8), "───┐\n");
        let mid_line = format!("{}{}{}", "│", "───┼".repeat(8), "───┤\n");
        let bot_line = format!("{}{}{}", "└", "───┴".repeat(8), "───┘");
        s.push_str(&top_line[..]);
        for i in 0..9 {
            s.push_str("│");
            for j in 0..9 {
                s = format!("{} {} {}", s, self.m[i][j], "│");
            }
            s.pop();
            s.push_str("│\n");
            if i != 8 {
                s.push_str(&mid_line[..]);
            }
        }

        s.push_str(&bot_line[..]);
        write!(f, "{}", s)
    }
}



fn solve(mut s: Sudoku) -> Option<Sudoku> {
    // Eliminate all the cells with only one guess
    while s.l == 1 {
        let mut one_cells: Vec<usize> = Vec::new();
        for cell in 0..81 {
            if s.n[1][cell] {
                one_cells.push(cell);
                s.n[1][cell] = false;
            }
        }
        for cell in one_cells {
            let j = cell % 9;
            let i = (cell - j) / 9;
            // s.clear_candidates(i, j);
            let guess = s.get_first_nonzero_guess(i, j);
            s.update(i, j, guess);

        }
    }

    if s.l > 0 && s.l != 11{
        let (i, j) = s.get_next_coordinates();
        // println!("i: {i}");
        s.n[s.l as usize][(i*9 + j) as usize] = false;
        let vals = s.c[i][j];
        // let mut rng = thread_rng();
        // vals.shuffle(&mut rng);
        for val in vals {
            let mut q = s.clone();
            if val > -1 {
                q.update(i, j, val);
                let p = solve(q);
                let mut is_none = false;
                match p {
                    None => {
                        is_none = true;
                    }
                    _ => {}
                }

                if !is_none {
                    // println!("{}", p.as_ref().unwrap());
                    // println!("num_solved: {}", p.as_ref().unwrap().num_solved);
                    return p;
                }
            }
        }
        None
    } else if s.l == 0 {
        None
    } else {
        Some(s)
    }
}

fn solve_puzzle(puzzle: [[i8; 9]; 9]) -> Sudoku {
    let s = Sudoku::create(puzzle);
    let res = solve(s);
    return res.unwrap();
}

fn main() {
    // let puzzle = [
    //     [8, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 3, 6, 0, 0, 0, 0, 0],
    //     [0, 7, 0, 0, 9, 0, 2, 0, 0],
    //     [0, 5, 0, 0, 0, 7, 0, 0, 0],
    //     [0, 0, 0, 0, 4, 5, 7, 0, 0],
    //     [0, 0, 0, 1, 0, 0, 0, 3, 0],
    //     [0, 0, 1, 0, 0, 0, 0, 6, 8],
    //     [0, 0, 8, 5, 0, 0, 0, 1, 0],
    //     [0, 9, 0, 0, 0, 0, 4, 0, 0],
    // ]; // Everest puzzle
    let puzzle = [
        [1, 0, 0, 0, 0, 7, 0, 9, 0],
        [0, 3, 0, 0, 2, 0, 0, 0, 8],
        [0, 0, 9, 6, 0, 0, 5, 0, 0],
        [0, 0, 5, 3, 0, 0, 9, 0, 0],
        [0, 1, 0, 0, 8, 0, 0, 0, 2],
        [6, 0, 0, 0, 0, 4, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 1, 0],
        [0, 4, 0, 0, 0, 0, 0, 0, 7],
        [0, 0, 7, 0, 0, 0, 3, 0, 0],
    ]; // Al Escargot puzzle
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

    let mut sol = Sudoku::create(puzzle);
    use std::time::Instant;
    let now = Instant::now();
    let n = 10;
    for _ in 0..n {
        sol = solve_puzzle(puzzle);
        // println!("num updates is {}", sol.num_updates);
    }
    println!("{}", sol);
    let elapsed = now.elapsed();
    println!("Avg time to solve puzzle over {} iterations: {:.2?}", n, elapsed/n)
}