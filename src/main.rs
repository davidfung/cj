use std::collections::HashMap;
use std::time::Instant;

fn load_db() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("題", "aombc"),
        ("決", "edk"),
        ("報", "gjsle"),
        ("粵", "hwmvs"),
        ("與", "hxyc"),
        ("求", "ije"),
        ("永", "ine"),
        ("慶", "ixe"),
        ("寶", "jmuc"),
        ("殺", "kchne"),
        ("蠅", "lirxu"),
        ("兆", "lmuo"),
        ("哥", "mrnr"),
        ("佛", "olln"),
        ("修", "oloh"),
        ("切", "psh"),
        ("挑", "qlmo"),
        ("呀", "rmvh"),
        ("改", "suok"),
        ("英", "tlbk"),
        ("甚", "tmmv"),
        ("差", "tqm"),
        ("變", "vfok"),
        ("姊", "vlxh"),
        ("旋", "ysono"),
        ("畜", "yviw"),
        ("；", "zxag"),
        ("：", "zxah"),
        ("？", "zxai"),
        ("！", "zxaj"),
        ("《", "zxbu"),
        ("》", "zxbv"),
        ("「", "zxcd"),
        ("」", "zxce"),
    ])
}

fn main() {
    loop {
        run()
    }
}
fn run() {
    let mut score = 0;
    let mut count = 0;
    let max_count = 3;
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

fn ask(prompt: &str, chinchar: &&str) -> bool {
    println!("{}[{}]?", prompt, chinchar);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    *chinchar == line.trim()
}
