use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

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
        if let Ok(lines) = self.read_lines(DATA_FILE) {
            for line in lines {
                if let Ok(buf) = line {
                    let parts: Vec<&str> = buf.split(",").collect();
                    if parts.len() >= 3 {
                        let code = parts[0].trim().to_string();
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

    // Save the current database to disk in a safe way.
    pub fn save(&mut self) {
        // save to a temp file
        let mut file = File::create(TEMP_FILE).expect("create failed");
        for x in &self.v {
            let s = format!("{},{},{}\n", x.code, x.char, x.score);
            file.write(s.as_bytes()).expect("data file write failed");
        }

        // delete original file
        fs::remove_file(DATA_FILE).expect("unable to remove old data file");

        // rename temp file to original file
        fs::rename(TEMP_FILE, DATA_FILE).expect("unable to rename data file")
    }

    // Given a set of chinese characters, return a random subset of it.
    // This implementation allows duplicates in the subset.
    pub fn get_items_random(&self, item_count: i32) -> Vec<Chinese> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
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
    // based on the scores.
    pub fn get_items_score(&self, item_count: i32) -> Vec<Chinese> {
        let mut q = Vec::new();
        let mut count;

        count = 0;
        for i in self.v.iter().filter(|x| x.score < 0) {
            println!("{} {} {}", i.char, i.code, i.score);
            count += 1;
            let c = Chinese {
                char: i.char.clone(),
                code: i.code.clone(),
                score: i.score,
            };
            q.push(c);
            if count >= item_count {
                break;
            }
        }
        q
    }

    // Update the database with the scores
    pub fn update(&mut self, items: Vec<Chinese>) {
        for y in items {
            let index = self.v.iter().position(|x| x.code == y.code).unwrap();
            self.v[index].score = y.score;
        }
    }
}

#[test]
fn test_db_update() {
    let mut db = CJDatabase { v: Vec::new() };
    db.load();
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
    db.load();
    let items = db.get_items_score(2);
    for i in &items {
        println!("{} {} {}", i.char, i.code, i.score);
    }
}
