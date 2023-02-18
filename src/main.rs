#![allow(dead_code)]
mod lexical_analysis;
mod synthesis;
mod tests_lexical_analysis;

fn count(curr: u32) {
    println!("{}", curr);
    if curr < 10 {
        count(curr + 1)
    }
}

fn main() {
    count(0);
}

