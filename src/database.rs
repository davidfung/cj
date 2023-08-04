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

    fn load_from(&mut self, filepath: &str) {
        // if filepath does not exist, create it with pristine data.
        if !Path::new(filepath).exists() {
            self.create_datafile(filepath);
        }

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
        println!("Records loaded: {}", self.v.len());
    }

    fn create_datafile(&mut self, filepath: &str) {
        println!("Creating database:  {}", filepath);

        let path = Path::new(filepath);
        let display = path.display();

        // create directory structure if necessary
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).unwrap();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the pristine data to `file`, returns `io::Result<()>`
        match file.write_all(PRISTINE.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
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
    #[allow(dead_code)]
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
        println!("Records sorted")
    }

    // De-duplication the database records by code+char.
    // code+char because one code can represent multiple chars.
    // Assume the records are already sorted by code+char.
    pub fn dedup(&mut self) {
        let mut counter = 0;
        let mut last = Chinese {
            code: "-1".to_string(),
            char: "".to_string(),
            score: 0,
        };
        let mut v2 = Vec::<Chinese>::new();
        for ch in self.v.iter() {
            if ch.code == last.code && ch.char == last.char {
                counter = counter + 1;
                if ch.score < last.score {
                    v2.pop();
                    v2.push(Chinese {
                        char: ch.char.clone(),
                        code: ch.code.clone(),
                        score: ch.score,
                    });
                    last.score = ch.score;
                }
                continue;
            }
            v2.push(Chinese {
                char: ch.char.clone(),
                code: ch.code.clone(),
                score: ch.score,
            });
            last.code = ch.code.clone();
            last.char = ch.char.clone();
            last.score = ch.score;
        }
        self.v = v2;
        println!("Duplicates removed: {}", counter);
    }
}

#[test]
fn test_db_update() {
    let mut db = CJDatabase { v: Vec::new() };
    db.load_from("./tests/cj01.csv");
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
    db.load_from("./tests/cj04.csv");
    let items = db.get_items_score(10);
    for (i, ch) in items.iter().enumerate() {
        println!("#{} {} {} {}", i, ch.char, ch.code, ch.score);
    }
}

#[test]
fn test_db_sort() {
    let mut db1 = CJDatabase { v: Vec::new() };
    db1.load_from("./tests/cj02a.csv");

    let mut db2 = CJDatabase { v: Vec::new() };
    db2.load_from("./tests/cj02b.csv");
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
        ("./tests/cj03a.csv", "./tests/cj03b.csv"),
        ("./tests/cj03c.csv", "./tests/cj03d.csv"),
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

#[test]
// test dedup keep lowest score
fn test_db_dedup_2() {
    let datafile = "./tests/cj03e.csv";

    let mut db = CJDatabase { v: Vec::new() };
    db.load_from(datafile);
    db.dedup();

    let x = db.v.iter().find(|x| x.code == "aombc");
    match x {
        Some(ch) => assert_eq!(ch.score, -9),
        None => panic!("unable to find target character"),
    }

    let x = db.v.iter().find(|x| x.code == "cvmi");
    match x {
        Some(ch) => assert_eq!(ch.score, -5),
        None => panic!("unable to find target character"),
    }

    let x = db.v.iter().find(|x| x.code == "ybog");
    match x {
        Some(ch) => assert_eq!(ch.score, -2),
        None => panic!("unable to find target character"),
    }

    // db.save_as("./tests/output.csv"); //remove afterward
    println!("{}", db.v.len());
}

#[test]
//zzz
fn test_db_create_datafile() {
    let datafile0 = "./tests/cj05.csv";
    let parent = "./tests/temp";
    let datafile1 = format!("{}/{}", parent, "cj05_temp.csv");
    let mut db = CJDatabase { v: Vec::new() };

    if Path::new(parent).is_dir() {
        fs::remove_dir(parent).unwrap();
    }

    db.create_datafile(datafile1.as_str());

    let mut db1 = CJDatabase { v: Vec::new() };
    db1.load_from(datafile0);

    let mut db2 = CJDatabase { v: Vec::new() };
    db2.load_from(datafile1.as_str());

    let matched = db1
        .v
        .iter()
        .zip(&db2.v)
        .filter(|(a, b)| a.code == b.code && a.char == b.char && a.score == b.score)
        .count();

    assert!(db1.v.len() == matched);
    fs::remove_file(datafile1).unwrap();
    fs::remove_dir(parent).unwrap();
}

static PRISTINE: &str = "
aombc,題,0
cvmi,鏘,0
edk,決,0
ehsk,淚,0
emhf,鴻,0
etct,溢,0
fog,雀,0
gjsle,報,0
hjhne,段,0
hodqn,衛,0
hohgn,衝,0
hqmqj,拜,0
hqu,毛,0
hwmvs,粵,0
hxyc,與,0
hycr,船,0
ijcc,麻,0
ije,求,0
ine,永,0
irp,感,0
ixe,慶,0
jchwk,窗,0
jcmvh,穿,0
jmuc,寶,0
joni,麥,0
kchne,殺,0
lirxu,蠅,0
lmuo,兆,0
lmyyy,非,0
mgnbe,瓊,0
mgph,瑟,0
miia,晉,0
mrnr,哥,0
mtn,邢,0
mvdh,牙,0
ni,夕,0
ni,弘,0
nk,又,0
ohce,傻,0
ohqi,俄,0
olln,佛,0
oloh,修,0
omh,乒,0
omi,乓,0
pd,也,0
psh,切,0
qlmo,挑,0
qsmb,掃,0
qtbk,撒,0
qybk,撤,0
rmvh,呀,0
sip,忍,0
sip,慰,0
smsim,羽,0
suok,改,0
tcno,歉,0
tjks,勒,0
tlbk,英,0
tmmv,甚,0
tnfd,蘇,0
tqm,差,0
ujmf,崇,0
vfhvp,紙,0
vfok,變,0
vlxh,姊,0
yapmm,韵,0
yarbc,韻,0
ybhg,望,0
ybmcu,睿,0
ybog,離,0
ybok,敵,0
ybou,遙,0
ybr,迥,0
yhhqm,產,0
yhus,邊,0
yhyu,遞,0
ymihh,歲,0
ymy,卡,0
yonk,夜,0
ysono,旋,0
ysyq,遲,0
ytsmm,翊,0
yviw,畜,0
zxag,；,0
zxah,：,0
zxai,？,0
zxaj,！,0
zxbu,《,0
zxbv,》,0
zxcd,「,0
zxce,」,0
";
