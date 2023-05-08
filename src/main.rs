use std::fmt;

#[derive(Debug, Clone)]
struct Sudoku {
    m: [[i8; 9]; 9],
    k: [[bool; 9]; 9],
    c: [[Vec<i8>; 9]; 9],
    n: [Vec<i8>; 10],
    l: i8,
    num_solved: u8,
    num_updates: u16,
}

impl Sudoku {
    fn create(m: [[i8; 9]; 9]) -> Self {
        let mut k = [[false; 9]; 9];
        let mut c: [[Vec<i8>; 9]; 9] = Default::default();
        let mut num_solved = 0;
        for item in m.iter().enumerate() {
            let (i, row): (usize, &[i8; 9]) = item;
            for square in row.iter().enumerate() {
                let (j, number): (usize, &i8) = square;
                if *number > 0 {
                    k[i][j] = true;
                    num_solved += 1;
                } else {
                    for num in 1..=9 {
                        c[i][j].push(num);
                    }
                }
            }
        }
        for i in 0..9 {
            for j in 0..9 {
                for x in 0..9 {
                    for y in 0..9 {
                        if i == x || j == y || (i / 3 == x / 3 && j / 3 == y / 3) {
                            let num_to_remove = m[x][y];
                            if k[x][y] {
                                let res = c[i][j].binary_search(&num_to_remove);
                                if let Ok(ind) = res {
                                    c[i][j].remove(ind);
                                }
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
        Self {
            m,
            k,
            c,
            n,
            l,
            num_solved,
            num_updates,
        }
    }

    fn get_n(c: &[[Vec<i8>; 9]; 9]) -> [Vec<i8>; 10] {
        let mut n: [Vec<i8>; 10] = Default::default();
        for i in 0..9 {
            for j in 0..9 {
                let num = c[i][j].len() as i8;
                n[num as usize].push((i * 9 + j) as i8);
            }
        }
        // println!("n is {:?}", n);
        n
    }

    fn get_l(n: &[Vec<i8>; 10]) -> i8 {
        for num_can in 1..10 {
            if n[num_can].len() > 0 {
                return num_can as i8;
            }
        }

        11
    }

    fn update(&mut self, i: usize, j: usize, val: i8) {
        // self.clear_candidates(i, j);
        self.c[i][j] = Vec::new();
        self.num_updates += 1;
        self.k[i][j] = true;
        self.num_solved += 1;

        // println!("num_solved: {}", self.num_solved);
        self.m[i][j] = val;
        // if self.num_solved == 81 {
        //     self.l = 0;
        //     return ()
        // }
        for n in 0..9 {
            for m in 0..9 {
                let row = i == n;
                let col = j == m;
                let block = i / 3 == n / 3 && j / 3 == m / 3;
                if row || col || block {
                    let mut guess_changed = false;
                    let res = self.c[n][m].binary_search(&val);
                    if let Ok(ind) = res {
                        self.c[n][m].remove(ind);
                        guess_changed = true;
                    }
                    let num = self.c[n][m].len(); //get_num_nonzero(&self.c[n][m]);
                    if num == 0 && guess_changed {
                        // println!("Leaving early");
                        self.l = 0;
                        return ();
                    }
                }
            }
        }

        self.l = Self::get_l(&self.n);
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
        let one_cells: Vec<_> = s.n[1].iter().copied().collect();
        for cell in one_cells {
            let j = cell % 9;
            let i = (cell - j) / 9;
            // s.clear_candidates(i, j);
            let res = s.c[i as usize][j as usize].pop();
            if let Some(guess) = res {
                s.update(i as usize, j as usize, guess);
            }
        }
    }

    if s.l > 0 && s.l != 11 {
        // let (i, j) = s.get_next_coordinates();
        // println!("i: {i}");
        // s.n[s.l as usize][(i*9 + j) as usize] = false;
        let res = s.n[s.l as usize].pop();
        let mut vals: Vec<i8> = Default::default();
        let mut i = 0;
        let mut j = 0;
        if let Some(cell_num) = res {
            j = cell_num % 9;
            i = (cell_num - j) / 9;
            vals = s.c[i as usize][j as usize].iter().copied().collect();
        }

        // let mut rng = thread_rng();
        // vals.shuffle(&mut rng);
        for val in vals {
            let mut q = s.clone();
            q.update(i as usize, j as usize, val);
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
    let puzzle = [
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 6, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 9, 0, 2, 0, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 0, 4, 5, 7, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 3, 0],
        [0, 0, 1, 0, 0, 0, 0, 6, 8],
        [0, 0, 8, 5, 0, 0, 0, 1, 0],
        [0, 9, 0, 0, 0, 0, 4, 0, 0],
    ]; // Everest puzzle
       // let puzzle = [
       //     [1, 0, 0, 0, 0, 7, 0, 9, 0],
       //     [0, 3, 0, 0, 2, 0, 0, 0, 8],
       //     [0, 0, 9, 6, 0, 0, 5, 0, 0],
       //     [0, 0, 5, 3, 0, 0, 9, 0, 0],
       //     [0, 1, 0, 0, 8, 0, 0, 0, 2],
       //     [6, 0, 0, 0, 0, 4, 0, 0, 0],
       //     [3, 0, 0, 0, 0, 0, 0, 1, 0],
       //     [0, 4, 0, 0, 0, 0, 0, 0, 7],
       //     [0, 0, 7, 0, 0, 0, 3, 0, 0],
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
    println!(
        "Avg time to solve puzzle over {} iterations: {:.2?}",
        n,
        elapsed / n
    )
}
