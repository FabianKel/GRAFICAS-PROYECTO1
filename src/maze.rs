use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(usize, usize)>>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut special_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut row: Vec<char> = Vec::new();

        for (x, ch) in line.chars().enumerate() {
            row.push(ch);

            if ch == '(' || ch == ')' || ch == '[' || ch == ']' {
                special_positions.entry(ch).or_insert(Vec::new()).push((x, y));
            }
        }

        maze.push(row);
    }

    (maze, special_positions)
}
