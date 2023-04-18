use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

struct Chinese {
    char: String,
    code: String,
    score: i16,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_db() -> Vec<Chinese> {
    let mut v = Vec::new();
    if let Ok(lines) = read_lines("./data/cj.csv") {
        for line in lines {
            if let Ok(buf) = line {
                let parts: Vec<&str> = buf.split(",").collect();
                if parts.len() >= 3 {
                    let code = parts[0].trim().to_string();
                    let char = parts[1].trim().to_string();
                    let score = parts[2].trim().parse::<i16>().unwrap();
                    v.push(Chinese {
                        char: char,
                        code: code,
                        score: score,
                    });
                }
            }
        }
    }
    print!("{} records imported", v.len());
    return v;
}

// Given a set of chinese characters, return a subset of it
// as the questions.  The selection process is based on some
// pre-defined criteria.
fn get_questions(db: Vec<Chinese>) -> Vec<Chinese> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut v = Vec::new();

    let mut count = 0;
    while count < 5 {
        count = count + 1;
        let question = db.choose(&mut thread_rng()).unwrap();
        let c = Chinese {
            char: question.char.clone(),
            code: question.code.clone(),
            score: question.score,
        };
        v.push(c);
    }
    return v;
}

fn ask(prompt: &str, chinchar: &String) -> bool {
    println!("{}[{}]?", prompt, chinchar);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}={}", chinchar, line.trim());
    chinchar == line.trim()
}

fn run() {
    let mut score = 0;
    let mut count = 0;
    let max_count = 10;
    let db = load_db();

    let questions = get_questions(db);

    println!("\n\n\n======== T E S T  B E G I N ========");
    let now = Instant::now();

    for chin in questions.iter() {
        count = count + 1;
        if count > max_count {
            break;
        }

        if ask("", &chin.char) {
            score = score + 1;
            println!("Correct! Score: {}/{}", score, count);
        } else {
            println!(
                "===> Wrong! {} should be \"{}\"!  Score:{}/{}",
                chin.char, chin.code, score, count
            );
            while !ask("Practice:", &chin.char) {}
        }
    }

    let elapsed_time = now.elapsed();
    println!("Time taken: {} seconds", elapsed_time.as_secs());
}

fn main() {
    loop {
        run()
    }
}
