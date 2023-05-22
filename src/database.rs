use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Chinese {
    pub char: String,
    pub code: String,
    pub score: i16,
}

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
        if let Ok(lines) = self.read_lines("./data/cj.csv") {
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

    // Given a set of chinese characters, return a subset of it
    // as the questions.  The selection process is based on some
    // pre-defined criteria.
    pub fn get_questions(&self) -> Vec<Chinese> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut q = Vec::new();

        let mut count = 0;
        while count < 5 {
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
