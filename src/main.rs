use itertools::iproduct;
use std::fs::File;
use std::io::{self, Read, Write};


fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn words_by_line<'a>(s: &'a str) -> Vec<&'a str> {
    s.lines()
        .flat_map(|line| line.split_whitespace().collect::<Vec<&'a str>>())
        .collect()
}

fn main() {
    let whole_file = filename_to_string("words.txt").unwrap();
    let wbyl = words_by_line(&whole_file);

    let combinations: String = iproduct!(wbyl.iter(), wbyl.iter(), wbyl.iter(), wbyl.iter())
        .map(|tuple| format!("{} {} {} {}\n", tuple.0, tuple.1, tuple.2, tuple.3))
        .collect();

    let mut output = File::create("wordlist.txt").unwrap();

    write!(output, "{}", &combinations).unwrap();
}
