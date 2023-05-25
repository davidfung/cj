use std::fs::{self, File};
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
