use std::{
    fs::{self, File, read},
    os::windows::fs::FileExt,
};

fn main() {
    find_password();
}

//
fn find_password() {
    let file: Vec<String> = read_file("C:/Users/Manolis/Desktop/adventofcode.txt");

    let mut pos: i32 = 50;
    let mut total_wraps: i32 = 0;

    for item in file {
        if let Some((dir, n)) = parse_move(&item) {
            let signed: i32 = match dir {
                'L' => n * -1,
                'R' => n * 1,
                _ => n,
            };
            let result = (((pos + signed) % 100) + 100) % 100;
            pos = result;


            if result == 0 {
                total_wraps += 1;
                println!("  Dial position at {} and the password is {}  ",
                result,
                total_wraps)
            }
        }
    }
}

// Splits the direction from the steps
fn parse_move(s: &str) -> Option<(char, i32)> {
    let s = s.trim();
    let (dir, num) = s.split_at(1);

    let dir = dir.chars().next()?;
    let n: i32 = num.parse().ok()?;

    Some((dir, n))
}

//Read the given file and returns its content in a string vector
fn read_file(path: &str) -> Vec<String> {
    let result = fs::read_to_string(path).expect("msg");
    result.lines().map(String::from).collect()
}
