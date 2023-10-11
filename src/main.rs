use std::time::{Duration, Instant};

use console::Term;

use database::{CJDatabase, Chinese};
use owo_colors::OwoColorize;

mod database;

const QUESTION_COUNT: usize = 10; // at least 10 questions

// Ask user to enter a chinese char.
// Return true if correct, false otherwise.
fn ask(prompt: &str, chinchar: &String) -> bool {
    println!("{}[{}]?", prompt, chinchar);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    //println!("{}={}", chinchar, line.trim());
    chinchar == line.trim()
}

fn run(items: Vec<Chinese>) -> Vec<Chinese> {
    let mut mark = 0; // for this challenge
    let mut count = 0;
    let qcount = items.len();
    let mut results: Vec<Chinese> = Vec::new();
    let mut score; // character statistics
    let mut prefix;

    println!("\n======== C H A L L E N G E   B E G I N S ========");
    let now = Instant::now();

    for mut chin in items.into_iter() {
        count = count + 1;
        if count > qcount {
            break;
        }

        println!("");
        prefix = format!("#{}/{} ", count, qcount);

        if ask(&prefix, &chin.char) {
            score = 1;
            mark = mark + 1;
            println!("Correct! Score: {}", mark);
        } else {
            score = -1;
            println!(
                "Wrong! {} should be \"{}\"!  Score: {}",
                chin.char, chin.code, mark
            );
            while !ask("Practice:", &chin.char) {}
        }

        chin.rating += score;
        if chin.rating == 0 {
            chin.rating += score;
        }
        results.push(chin);
    }

    let elapsed_time = now.elapsed();
    show_score(mark, qcount, elapsed_time);

    results
}

fn show_banner() {
    println!(
        "{}",
        "
****************************
*                          *
*      Welcome to the      *
*                          *
*           C  J           *
*                          *
*        Challenges        *
*                          *
****************************
"
        .bright_green()
    );
}

fn show_score(score: i16, max_score: usize, time_taken: Duration) {
    let msg = format!(
        "
***************************
*                         *
*   Score: {:4.0} %         *
*    Time: {: >4} seconds   *
*                         *
***************************
",
        (score as usize * 100 / max_score) as u16,
        time_taken.as_secs()
    );
    println!("{}", msg.bright_yellow());
}

fn ask_continue(msg: &str) -> bool {
    println!("{}\n", msg);
    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(ch) = stdout.read_char() {
            match ch.to_ascii_uppercase() {
                'C' => return true,
                'Q' => return false,
                _ => continue,
            }
        }
    }
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
        println!("");
        if !ask_continue("Press C to continue, Q to quit.") {
            return;
        }
        let items = db.get_items_smart(QUESTION_COUNT);
        let results = run(items);
        db.update(results);
        db.save();
    }
}

#[test]
fn test_show_banner() {
    show_banner();
}
