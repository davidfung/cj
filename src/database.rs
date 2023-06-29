use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;

const DATA_FILE: &str = r"./data/cj.csv";
const TEMP_FILE: &str = r"./data/cjtemp.csv";

pub struct Chinese {
    pub char: String,
    pub code: String,
    pub score: i16,
}

// A Chinese characters database implemented as a vector.
pub struct CJDatabase {
    pub v: Vec<Chinese>,
}

impl CJDatabase {
    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn load(&mut self) {
        self.load_from(DATA_FILE);
    }

    pub fn load_from(&mut self, filepath: &str) {
        if let Ok(lines) = self.read_lines(filepath) {
            for line in lines {
                if let Ok(buf) = line {
                    let parts: Vec<&str> = buf.split(",").collect();
                    if parts.len() >= 3 {
                        let code = parts[0].trim().to_string().to_lowercase();
                        let char = parts[1].trim().to_string();
                        let score = parts[2].trim().parse::<i16>().unwrap();
                        self.v.push(Chinese {
                            char: char,
                            code: code,
                            score: score,
                        });
                    }
                }
            }
        }
        println!("{} records imported", self.v.len());
    }

    // Save the current database with the default filename.
    pub fn save(&mut self) {
        self.save_as(DATA_FILE);
    }

    // Save the current database to disk in a safe way.
    pub fn save_as(&mut self, filepath: &str) {
        // save to a temp file
        let mut file = File::create(TEMP_FILE).expect("create failed");
        for x in &self.v {
            let s = format!("{},{},{}\n", x.code, x.char, x.score);
            file.write(s.as_bytes()).expect("data file write failed");
        }

        // delete original file
        if std::path::Path::new(filepath).exists() {
            fs::remove_file(filepath).expect("unable to remove old data file");
        }

        // rename temp file to original file
        fs::rename(TEMP_FILE, filepath).expect("unable to rename data file")
    }

    // Given a set of chinese characters, return a random subset of it.
    // This implementation allows duplicates in the subset.
    pub fn get_items_random(&self, item_count: i32) -> Vec<Chinese> {
        let mut q = Vec::new();

        let mut count = 0;
        while count < item_count {
            count = count + 1;
            let question = self.v.choose(&mut thread_rng()).unwrap();
            let c = Chinese {
                char: question.char.clone(),
                code: question.code.clone(),
                score: question.score,
            };
            q.push(c);
        }
        return q;
    }

    // Given a set of chinese characters, return a subset of it
    // based on the scores:
    // at most 33%: score < 0      [difficult]
    // at most 33%: score == 0     [new]
    // at most 33%: 0 < score <= 3 [easy]
    // the remaining: score > 3    [very easy]
    // rest                        [random]
    pub fn get_items_score(&self, item_count: usize) -> Vec<Chinese> {
        let mut items = Vec::new();
        let quota = item_count / 3; // 33%
        let mut rest;

        // difficult
        for q in self
            .v
            .iter()
            .filter(|x| x.score < 0)
            .choose_multiple(&mut thread_rng(), quota)
            .iter()
        {
            if items.len() >= item_count {
                break;
            }
            let c = Chinese {
                char: q.char.clone(),
                code: q.code.clone(),
                score: q.score,
            };
            items.push(c);
        }

        // new
        for q in self
            .v
            .iter()
            .filter(|x| x.score == 0)
            .choose_multiple(&mut thread_rng(), quota)
            .iter()
        {
            if items.len() >= item_count {
                break;
            }
            let c = Chinese {
                char: q.char.clone(),
                code: q.code.clone(),
                score: q.score,
            };
            items.push(c);
        }

        // easy
        for q in self
            .v
            .iter()
            .filter(|x| x.score > 0 && x.score <= 3)
            .choose_multiple(&mut thread_rng(), quota)
            .iter()
        {
            if items.len() >= item_count {
                break;
            }
            let c = Chinese {
                char: q.char.clone(),
                code: q.code.clone(),
                score: q.score,
            };
            items.push(c);
        }

        // very easy
        rest = item_count - items.len();
        if rest > 0 {
            for q in self
                .v
                .iter()
                .filter(|x| x.score > 3)
                .choose_multiple(&mut thread_rng(), rest)
                .iter()
            {
                let c = Chinese {
                    char: q.char.clone(),
                    code: q.code.clone(),
                    score: q.score,
                };
                items.push(c);
            }
        }

        // random
        rest = item_count - items.len();
        if rest > 0 {
            for q in self
                .v
                .iter()
                .choose_multiple(&mut thread_rng(), rest)
                .iter()
            {
                let c = Chinese {
                    char: q.char.clone(),
                    code: q.code.clone(),
                    score: q.score,
                };
                items.push(c);
            }
        }

        items.shuffle(&mut thread_rng());
        items
    }

    // Update the database with the scores
    pub fn update(&mut self, items: Vec<Chinese>) {
        for y in items {
            let index = self.v.iter().position(|x| x.code == y.code).unwrap();
            self.v[index].score = y.score;
        }
    }

    // Sort the database records by code
    pub fn sort(&mut self) {
        self.v.sort_by_key(|x| format!("{} {}", x.code, x.char));
    }

    // De-duplication the database records by code+char.
    // code+char because one code can represent multiple chars.
    // Assume the records are already sorted by code+char.
    pub fn dedup(&mut self) {
        let mut lastcode = "-1".to_string();
        let mut lastchar = "".to_string();
        let mut v2 = Vec::<Chinese>::new();
        for ch in self.v.iter() {
            if ch.code == lastcode && ch.char == lastchar {
                continue;
            }
            v2.push(Chinese {
                char: ch.char.clone(),
                code: ch.code.clone(),
                score: ch.score,
            });
            lastcode = ch.code.clone();
            lastchar = ch.char.clone();
        }
        self.v = v2;
    }
}

#[test]
fn test_db_update() {
    let mut db = CJDatabase { v: Vec::new() };
    db.load_from("./unittest/cj01.csv");
    let items = db.get_items_random(2);
    for i in &items {
        println!("{} {} {}", i.char, i.code, i.score);
    }
    db.update(items);
    db.save();
}

#[test]
fn test_db_get_items_score() {
    let mut db = CJDatabase { v: Vec::new() };
    db.load_from("./unittest/cj04.csv");
    let items = db.get_items_score(10);
    for (i, ch) in items.iter().enumerate() {
        println!("#{} {} {} {}", i, ch.char, ch.code, ch.score);
    }
}

#[test]
fn test_db_sort() {
    let mut db1 = CJDatabase { v: Vec::new() };
    db1.load_from("./unittest/cj02a.csv");

    let mut db2 = CJDatabase { v: Vec::new() };
    db2.load_from("./unittest/cj02b.csv");
    db2.sort();

    println!("db1 len={}, db2 len={}", db1.v.len(), db2.v.len());
    assert!(db1.v.len() == db2.v.len());

    let matched = db1
        .v
        .iter()
        .zip(&db2.v)
        .filter(|(a, b)| a.code == b.code && a.char == b.char && a.score == b.score)
        .count();

    assert!(db1.v.len() == matched);
}

#[test]
fn test_db_dedup() {
    let data = [
        ("./unittest/cj03a.csv", "./unittest/cj03b.csv"),
        ("./unittest/cj03c.csv", "./unittest/cj03d.csv"),
    ];

    for (a, b) in data {
        let mut db1 = CJDatabase { v: Vec::new() };
        db1.load_from(a);

        let mut db2 = CJDatabase { v: Vec::new() };
        db2.load_from(b);
        db2.dedup();
        println!("{}", db2.v.len());

        let matched = db1
            .v
            .iter()
            .zip(&db2.v)
            .filter(|(a, b)| a.code == b.code && a.char == b.char && a.score == b.score)
            .count();

        assert!(db1.v.len() == matched);
    }
}
