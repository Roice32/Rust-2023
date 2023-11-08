use anyhow::Result;
use std::{fs, thread, time};

struct GameBoard {
    cells: [[bool; 34]; 34],
    size: usize,
}

fn read_initial_state(gb: &mut GameBoard, path: &str) -> Result<()> {
    let initial_state: String = fs::read_to_string(path)?;
    let mut i: usize = 1;
    for line in initial_state.lines() {
        if i == 1 {
            gb.size = line.len();
        }
        for j in 1..gb.size + 1 {
            match line.chars().nth(j - 1) {
                Some(c) => gb.cells[i][j] = c == 'x',
                None => {}
            }
        }
        i += 1;
    }
    Ok(())
}

fn new_cell_state(ps: [[bool; 34]; 34], x: usize, y: usize) -> bool {
    let mut neighbors_count = 0;
    for dx in 0..3 {
        for dy in 0..3 {
            if dx * dy != 1 && ps[x + dx - 1][y + dy - 1] == true {
                neighbors_count += 1;
            }
        }
    }
    if ps[x][y] == true && (neighbors_count == 2 || neighbors_count == 3) {
        return true;
    } else if ps[x][y] == false && neighbors_count == 3 {
        return true;
    } else {
        return false;
    }
}

fn next_state(gb: &mut GameBoard) {
    let prev_state: [[bool; 34]; 34] = gb.cells;
    for i in 1..gb.size + 1 {
        for j in 1..gb.size + 1 {
            gb.cells[i][j] = new_cell_state(prev_state, i, j);
        }
    }
}

fn print_state(gb: &GameBoard) {
    for _ in 1..gb.size + 3 {
        print!("#");
    }
    print!("\n");
    for i in 1..gb.size + 1 {
        print!("#");
        for j in 1..gb.size + 1 {
            if gb.cells[i][j] == true {
                print!("x");
            } else {
                print!(" ");
            }
        }
        print!("#\n");
    }
    for _ in 1..gb.size + 3 {
        print!("#");
    }
    print!("\n");
}

fn main() -> Result<()> {
    let mut running_game: GameBoard = GameBoard {
        cells: [[false; 34]; 34],
        size: 34,
    };
    read_initial_state(&mut running_game, "src/life2.game")?;
    print_state(&running_game);
    for _ in 1..11 {
        thread::sleep(time::Duration::from_millis(500));
        next_state(&mut running_game);
        print_state(&running_game);
    }
    Ok(())
}
