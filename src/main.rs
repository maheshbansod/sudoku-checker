use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("Usage: program <input-file>");
        return Ok(());
    }

    let _ = args.next();
    let file_path = args.next().unwrap();
    let data = fs::read_to_string(file_path)?;
    let numbers = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    check_sudoku(&numbers);
    Ok(())
}

/// Checks the sudoku rules and either prints out the rule
/// that is not satisfied or prints that it's all good
fn check_sudoku(numbers: &Vec<Vec<u32>>) {
    let mut has_error = false;
    for i in 0..9 {
        let mut horizontal_numbers = HashSet::new();
        let mut vertical_numbers = HashSet::new();
        let mut block_numbers = HashSet::new();

        let block_start_i = (i / 3) * 3;
        let block_start_j = (i % 3) * 3;
        for j in 0..9 {
            let nh = numbers[i][j];
            if horizontal_numbers.contains(&nh) {
                println!("Row {} contains {nh} multiple times.", i + 1);
                has_error = true;
            }
            horizontal_numbers.insert(nh);
            let nv = numbers[j][i];
            if vertical_numbers.contains(&nv) {
                println!("Column {} contains {nv} multiple times.", i + 1);
                has_error = true;
            }
            vertical_numbers.insert(nv);
            let bi = block_start_i + j / 3;
            let bj = block_start_j + j % 3;
            let nb = numbers[bi][bj];
            if block_numbers.contains(&nb) {
                println!(
                    "Block at {},{} contains {nb} multiple times.",
                    block_start_j + 1,
                    block_start_i + 1
                );
                has_error = true;
            }
            block_numbers.insert(nb);
        }
        for j in 1..10 {
            if !horizontal_numbers.contains(&j) {
                println!("The row {} is missing {j}", i + 1);
                has_error = true;
            }
            if !vertical_numbers.contains(&j) {
                println!("The column {} is missing {j}", i + 1);
                has_error = true;
            }
            if !block_numbers.contains(&j) {
                println!("The block number {} is missing {j}", i + 1);
                has_error = true;
            }
        }
    }
    if !has_error {
        println!("It's all good man")
    }
}
