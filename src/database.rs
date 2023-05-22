use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

const QUESTION_COUNT: i32 = 2;
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
        print!("{} records imported", self.v.len());
    }

    // Save the current database to disk.
    pub fn save(&mut self) {
        let mut file = File::create(TEMP_FILE).expect("create failed");
        file.write("hello world".as_bytes()).expect("write failed");
    }

    // Given a set of chinese characters, return a subset of it
    // as the questions.  The selection process is based on some
    // pre-defined criteria.
    pub fn get_questions(&self) -> Vec<Chinese> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut q = Vec::new();

        let mut count = 0;
        while count < QUESTION_COUNT {
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
}
