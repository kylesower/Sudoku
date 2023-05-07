use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Hash, Clone, Copy)]
struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    fn create(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Cell {}

#[derive(Debug, Clone)]
struct Sudoku {
    m: [[u8; 9]; 9],
    k: HashMap<Cell, u8>,
    c: HashMap<Cell, HashSet<u8>>,
    n: HashMap<usize, HashSet<Cell>>,
    l: usize,
}

impl Sudoku {
    fn create(m: [[u8; 9]; 9]) -> Self {
        let mut k: HashMap<Cell, u8> = HashMap::new();
        let mut c: HashMap<Cell, HashSet<u8>> = HashMap::new();
        for item in m.iter().enumerate() {
            let (i, row): (usize, &[u8; 9]) = item;
            for square in row.iter().enumerate() {
                let (j, number): (usize, &u8) = square;
                let curr_cell = Cell::create(i.into(), j.into());
                if *number > 0 {
                    k.entry(curr_cell).or_insert(*number);
                    ();
                } else {
                    let mut guesses = HashSet::new();
                    for num in 1..=9 {
                        guesses.insert(num);
                    }
                    c.entry(curr_cell).or_insert(guesses);
                }
            }
        }

        for i in 0..9 {
            for j in 0..9 {
                for n in 0..9 {
                    for m in 0..9 {
                        if i == n || j == m || (i / 3 == n / 3 && j / 3 == m / 3) {
                            let ccell = Cell::create(i.into(), j.into());
                            let kcell = Cell::create(n, m);
                            let num_to_remove = k.get(&kcell);
                            if let Some(x) = num_to_remove {
                                let cell_to_mod = c.get_mut(&ccell);
                                if let Some(guesses) = cell_to_mod {
                                    guesses.remove(x);
                                }
                            }
                        }
                    }
                }
            }
        }

        let n = Self::get_n(&c);
        let l = Self::get_l(&n, c.is_empty());

        Self { m, k, c, n, l }
    }

    fn get_n(c: &HashMap<Cell, HashSet<u8>>) -> HashMap<usize, HashSet<Cell>> {
        let mut n: HashMap<usize, HashSet<Cell>> = HashMap::new();
        for (key, val) in c.iter() {
            let mut hashkey = HashSet::new();
            hashkey.insert(*key);
            match n.entry(val.len()) {
                Entry::Occupied(_) => {
                    n.get_mut(&val.len()).unwrap().extend(&hashkey);
                }
                Entry::Vacant(_) => {
                    n.insert(val.len(), hashkey);
                }
            };
        }

        n
    }

    fn get_l(n: &HashMap<usize, HashSet<Cell>>, c_is_empty: bool) -> usize {
        let res = n.keys().min();
        if let Some(l) = res {
            return *l;
        } else {
            if c_is_empty {
                return 11;
            } else {
                return 0;
            }
        }
    }

    fn update(&mut self, cell: Cell, val: u8) {
        self.c.remove(&cell);
        self.k.entry(cell).or_insert(val);
        self.m[cell.x][cell.y] = val;
        let mut to_remove: HashMap<Cell, u8> = HashMap::new();
        for &ccell in self.c.keys() {
            let row = ccell.x == cell.x;
            let col = ccell.y == cell.y;
            let block = ccell.x / 3 == cell.x / 3 && ccell.y / 3 == cell.y / 3;

            if row || col || block {
                to_remove.insert(ccell, val);
            }
        }

        for (k, v) in &to_remove {
            self.c.get_mut(k).unwrap().remove(v);
            if self.c.get_mut(k).unwrap().len() == 0 {
                self.c.remove(k);
            }
        }

        self.n = Self::get_n(&self.c);
        self.l = Self::get_l(&self.n, self.c.is_empty());
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        let top_line = format!("{}{}{}", "┌", "───┬".repeat(8), "───┐\n");
        let mid_line = format!("{}{}{}", "│", "───┼".repeat(8), "───┤\n");
        let bot_line = format!("{}{}{}", "└", "───┴".repeat(8), "───┘");
        s.push_str(&top_line[..]);
        for item in self.m.iter().enumerate() {
            let (i, row): (usize, &[u8; 9]) = item;
            s.push_str("│");
            for number in row {
                s = format!("{} {} {}", s, number, "│");
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
    let res = s.n.get(&1);
    let mut one_cells: HashSet<Cell> = HashSet::new();
    if let Some(set) = res {
        one_cells = set.iter().copied().collect();
    }
    for cell in one_cells {
        let res = s.n.get_mut(&1);
        if let Some(set) = res {
            set.remove(&cell);
        }
        let val = s.c.get(&cell);
        if let Some(value) = val {
            s.update(cell, *value.iter().next().unwrap());
        }
    }

    if s.l > 0 && s.l != 11 {
        let res = s.n.get(&s.l);
        let mut next_cell: Cell = Cell::create(10, 10);
        if let Some(set) = res {
            next_cell = set.iter().copied().next().unwrap();
        }
        let res = s.n.get_mut(&s.l);
        if let Some(set) = res {
            set.remove(&next_cell);
        }

        let res = s.c.get(&next_cell);
        let mut guesses: HashSet<u8> = HashSet::new();
        if let Some(set) = res {
            guesses = set.clone();
        }

        for guess in guesses {
            let mut q = s.clone();
            q.update(next_cell, guess);
            let p = solve(q.clone());
            let mut is_none = false;
            match p {
                None => {
                    is_none = true;
                }
                _ => {}
            }

            if !is_none {
                return p;
            }
        }
        None
    } else if s.l == 0 {
        None
    } else {
        for row in s.m {
            for val in row {
                if val == 0 {
                    return None;
                }
            }
        }

        Some(s)
    }
}

fn main() {
    let everest = [
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 6, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 9, 0, 2, 0, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 0, 4, 5, 7, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 3, 0],
        [0, 0, 1, 0, 0, 0, 0, 6, 8],
        [0, 0, 8, 5, 0, 0, 0, 1, 0],
        [0, 9, 0, 0, 0, 0, 4, 0, 0],
    ];
    let s = Sudoku::create(everest);
    println!("Original puzzle:");
    println!("{}", s);
    let res = solve(s.clone());
    return match res {
        Some(sol) => {
            println!("Solution:");
            println!("{}", sol)
        }
        None => println!("No solution found."),
    };
}
