use std::time::Instant;

use database::CJDatabase;

mod database;

fn ask(prompt: &str, chinchar: &String) -> bool {
    println!("{}[{}]?", prompt, chinchar);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}={}", chinchar, line.trim());
    chinchar == line.trim()
}

fn run(db: &mut CJDatabase) {
    let mut score = 0;
    let mut count = 0;
    let max_count = 10;

    let questions = db.get_questions();

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
    let mut db = database::CJDatabase { v: Vec::new() };
    db.load();

    loop {
        run(&mut db);
        db.save();
    }
}
