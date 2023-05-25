use std::time::Instant;

use database::{CJDatabase, Chinese};

mod database;

fn ask(prompt: &str, chinchar: &String) -> bool {
    println!("{}[{}]?", prompt, chinchar);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}={}", chinchar, line.trim());
    chinchar == line.trim()
}

fn run(items: Vec<Chinese>) -> Vec<Chinese> {
    let mut score = 0;
    let mut count = 0;
    let max_count = 10;
    let mut results: Vec<Chinese> = Vec::new();
    let mut mark;

    println!("\n\n\n======== T E S T  B E G I N ========");
    let now = Instant::now();

    for chin in items.iter() {
        count = count + 1;
        if count > max_count {
            break;
        }

        if ask("", &chin.char) {
            mark = 1;
            score = score + 1;
            println!("Correct! Score: {}/{}", score, count);
        } else {
            mark = -1;
            println!(
                "===> Wrong! {} should be \"{}\"!  Score:{}/{}",
                chin.char, chin.code, score, count
            );
            while !ask("Practice:", &chin.char) {}
        }

        let result = Chinese {
            char: chin.char.clone(),
            code: chin.code.clone(),
            score: chin.score + mark,
        };
        results.push(result);
    }

    let elapsed_time = now.elapsed();
    println!("Time taken: {} seconds", elapsed_time.as_secs());

    results
}

fn main() {
    let mut db = CJDatabase { v: Vec::new() };
    db.load();

    loop {
        let items = db.get_items();
        let results = run(items);
        db.update(results);
        db.save();
    }
}
