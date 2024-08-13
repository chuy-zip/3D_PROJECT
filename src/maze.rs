
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn find_start_position(maze: &Vec<Vec<char>>, block_size: usize) -> Option<(usize, usize)> {
    for (row, row_data) in maze.iter().enumerate() {
        for (col, &cell) in row_data.iter().enumerate() {
            if cell == 's' {
                let x = col * block_size - block_size / 2;
                let y = row * block_size + block_size / 2;
                return Some((x, y));
            }
        }
    }
    None
}
