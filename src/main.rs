use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("python") // Executing Anaylze.py
        .arg("./Anaylze/analyze.py")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        // Convert the stdout bytes to a string
        let stdout_str = String::from_utf8_lossy(&output.stdout);

        // Parsing
        let mut sudoku: [[i32; 9]; 9] = parse_sudoku(&stdout_str);

        //println!("{:?}", sudoku);
        if sudoku_validation(sudoku) {
            if solve(&mut sudoku) {
                for row in sudoku.iter() { // Formatting before printing
                    let row_str: Vec<String> = row.iter().map(|&cell| cell.to_string()).collect();
                    println!("{}", row_str.join(" "));
                }
            }
        }
    } else {
        eprintln!("Error running Python script: {:?}", output.status);
    }
}

fn parse_sudoku(sudoku_str: &str) -> [[i32; 9]; 9] {
    // Split the input string into lines
    let lines: Vec<&str> = sudoku_str.lines().collect();

    // Init the array
    let mut sudoku: [[i32; 9]; 9] = [[0; 9]; 9];

    for (i, line) in lines.iter().enumerate() {
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect(); // Split each line into a vector of integers

        // Assign the parsed values to the corresponding row in the Sudoku array
        for (j, &value) in values.iter().enumerate() {
            sudoku[i][j] = value;
        }
    }

    sudoku
}

fn sudoku_validation(sudoku: [[i32; 9]; 9]) -> bool {
    let mut count = 0;

    for r in 0..9 {
        let mut col = [0; 9];
        for c in 0..9 {
            if sudoku[r][c] != 0 {
                count += 1;
            }
            col[c] = sudoku[c][r]; // Making a column array
        }

        col.sort(); // Sorting the row and checking for duplicates
        for i in 0..8 {
            if col[i] != 0 {
                if col[i] == col[i+1] {
                    return false;
                }
            }
        }

        let row = &sudoku[0]; // Same thing but for rows
        let mut cloned_row = row.to_vec();
        cloned_row.sort();
        for i in 0..8 {
            if cloned_row[i] != 0 {
                if cloned_row[i] == cloned_row[i+1] {
                    return false;
                }
            }
        }
    }

    if count < 17 { // You need a minimum of 17 values to solve a sudoku
        return false;
    }

    true
}

fn next_empty_space(sudoku: [[i32; 9]; 9], start: (usize, usize)) -> (usize, usize) {
    let (l_r, l_c) = start; // Having a start pos to reduce computing
    for r in l_r..9{
        for c in l_c..9{
            if sudoku[r][c] == 0{
                return (r, c); // Returned the found next empty space
            }
        }
    }

    (10, 10) // If sudoku is full
}

fn is_placement_valid(sudoku: [[i32; 9]; 9], guess: &i32, r: usize, c: usize) -> bool{
    if sudoku[r].contains(guess) { // Checking for same number in the same row
        return false;
    }

    for i in 0..9 { // Same thing but for columns
        if *guess == sudoku[i][c] {
            return false;
        }
    }

    let row_start: usize = (((r / 3) as f32).floor() as usize) * 3; // Getting the row/columns start pos
    let col_start: usize = (((c / 3) as f32).floor() as usize) * 3;

    for i in row_start..row_start + 3 {
        for j in col_start..col_start + 3 {
            if *guess == sudoku[i][j] {
                return false;
            }
        }
    }

    true
}

fn solve(sudoku: &mut [[i32; 9]; 9]) -> bool { // Solve function. Uses backtracking
    let (mut r, mut c) = (0, 0);
    (r, c) = next_empty_space(*sudoku, (r, c));

    if r == 10 {
        return true; // Solved (hopefully)
    }
    for guess in 1..10 {
        if is_placement_valid(*sudoku, &guess, r, c) {
            sudoku[r][c] = guess; // Replaces the value if its valid
            if solve(sudoku) {
                return true;
            }
        }
        sudoku[r][c] = 0; // Backtracking
    }

    false
}