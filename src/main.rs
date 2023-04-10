use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_db() -> HashMap<String, String> {
    let mut h = HashMap::new();
    if let Ok(lines) = read_lines("./data/cj.csv") {
        for line in lines {
            if let Ok(buf) = line {
                let parts: Vec<&str> = buf.split(",").collect();
                if parts.len() >= 3 {
                    let chincode = parts[0].trim().to_string();
                    let chinchar = parts[1].trim().to_string();
                    h.insert(chinchar, chincode);
                }
            }
        }
    }
    print!("{} records imported", h.len());
    return h;
}

fn main() {
    loop {
        run()
    }
}

fn run() {
    let mut score = 0;
    let mut count = 0;
    let max_count = 10;
    let db = load_db();
    let now = Instant::now();

    println!("\n======== T E S T  B E G I N ========");

    for (chinchar, chincode) in &db {
        count = count + 1;
        if count > max_count {
            break;
        }

        if ask("", chinchar) {
            score = score + 1;
            println!("Correct! Score: {}/{}", score, count);
        } else {
            println!(
                "===> Wrong! {} should be \"{}\"!  Score:{}/{}",
                chinchar, chincode, score, count
            );
            while !ask("Practice:", chinchar) {}
        }
    }

    let elapsed_time = now.elapsed();
    println!("Time taken: {} seconds", elapsed_time.as_secs());
}

fn ask(prompt: &str, chinchar: &str) -> bool {
    println!("{}[{}]?", prompt, chinchar);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    chinchar == line.trim()
}
