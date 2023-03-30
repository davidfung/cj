use std::collections::HashMap;
use std::time::Instant;

fn load_db() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("報", "gjsle"),
        ("姊", "vlxh"),
        ("題", "aombc"),
        ("決", "edk"),
        ("粵", "hwmvs"),
        ("與", "hxyc"),
        ("求", "ije"),
        ("永", "ine"),
        ("慶", "ixe"),
        ("寶", "jmuc"),
        ("兆", "lmuo"),
        ("哥", "mrnr"),
        ("佛", "olln"),
        ("修", "oloh"),
        ("挑", "qlmo"),
        ("呀", "rmvh"),
        ("改", "suok"),
        ("英", "tlbk"),
        ("甚", "tmmv"),
        ("旋", "ysono"),
        ("蠅", "lirxu"),
        ("畜", "yviw"),
        ("殺", "kchne"),
        ("變", "vfok"),
        ("切", "psh "),        
    ])
}

fn main() {
    let mut score = 0;
    let mut count = 0;
    let max_count = 10;
    let db = load_db();
    let now = Instant::now();

    for (chinchar, chincode) in &db {
        count = count +1;
        if count > max_count {break;}

        println!("{}?", chinchar);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        
        // println!("line={}", line);
        // println!("answer={}", chinchar);
        
        if line == *chinchar {
            score = score + 1;
            println!("Correct! Score: {}/{}", score, count);
        } else {
            println!("===> Wrong! Should be {}!  Score:{}/{}", chincode, score, count);
        }
    }

    let elapsed_time = now.elapsed();
    println!("Time taken: {} seconds", elapsed_time.as_secs());
}
