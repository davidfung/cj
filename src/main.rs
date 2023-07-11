use std::time::Instant;

use database::{CJDatabase, Chinese};

mod database;

const QUESTION_COUNT: usize = 10; // at least 10 questions

fn ask(prompt: &str, chinchar: &String) -> bool {
    println!("{}[{}]?", prompt, chinchar);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    //println!("{}={}", chinchar, line.trim());
    chinchar == line.trim()
}

fn run(items: Vec<Chinese>) -> Vec<Chinese> {
    let mut mark = 0;
    let mut count = 0;
    let max_count = 10;
    let mut results: Vec<Chinese> = Vec::new();
    let mut score;

    println!("\n\n\n======== T E S T  B E G I N ========");
    let now = Instant::now();

    for mut chin in items.into_iter() {
        count = count + 1;
        if count > max_count {
            break;
        }

        if ask("", &chin.char) {
            score = 1;
            mark = mark + 1;
            println!("Correct! Mark: {}/{}", mark, count);
        } else {
            score = -1;
            println!(
                "===> Wrong! {} should be \"{}\"!  Mark:{}/{}",
                chin.char, chin.code, mark, count
            );
            while !ask("Practice:", &chin.char) {}
        }

        chin.score += score;
        results.push(chin);
    }

    let elapsed_time = now.elapsed();
    println!("Time taken: {} seconds", elapsed_time.as_secs());

    results
}

fn show_banner() {
    println!(
        "
****************************
*                          *
*      Welcome to the      *
*                          *
*          C    J          *
*                          *
*        Challenges        *
*                          *
****************************
"
    );
}
fn main() {
    println!("Initiating CJ Challenges...");

    let mut db = CJDatabase { v: Vec::new() };
    db.load();
    db.sort();
    db.dedup();
    db.save();

    show_banner();

    loop {
        let items = db.get_items_score(QUESTION_COUNT);
        let results = run(items);
        db.update(results);
        db.save();
    }
}

#[test]
fn test_show_banner() {
    show_banner();
}
