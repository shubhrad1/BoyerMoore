use std::{
    char,
    collections::HashMap,
    env::{self},
    fs::File,
    io::Read,
    path::Path,
    process::exit,
};

fn shift_table(pattern: &str) -> HashMap<char, usize> {
    let mut table: HashMap<char, usize> = HashMap::new();
    let len: usize = pattern.to_string().len();
    for (i, ch) in pattern.chars().enumerate() {
        table.insert(ch, (len - i - 1).max(1));
    }
    table
}

fn file_open(path_string: &str) -> String {
    let path = Path::new(path_string);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(err) => panic!("Cannot open file {}:{}", display, err),
        Ok(file) => file,
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Err(err) => panic!("Cannot read file {}:{}", display, err),
        Ok(_) => file_content,
    }
}

// fn batching(lines:Vec<String>)-> Vec<Vec<String>>{
//     let lines_len:usize=lines.len();
//     
//     let mut pack:Vec<String>=vec![];
//     let mut batch_count=&lines_len;
//
//
// }

fn search(shift: HashMap<char, usize>, pattern: &str, path: &str) -> Vec<(usize, String)> {
    let mut candidate: Vec<(usize, String)> = Vec::new();
    let pattern_len: usize = pattern.to_string().len();

    let text: String = file_open(path);

    // let f_len: usize = text.to_string().len();

    let p_vec: Vec<char> = pattern.chars().collect();

    for (line_idx, line) in text.lines().enumerate() {
        let t_vec: Vec<char> = line.chars().collect();

        let t_len = t_vec.len();
        if t_len < pattern_len {
            continue;
        }
        let mut i = pattern_len - 1;

        while i < t_len {
            let mut matched: bool = true;

            for j in (0..pattern_len).rev() {
                let p_char: char = p_vec[j];
                let t_char: char = t_vec[i + j + 1 - pattern_len];
                if p_char != t_char {
                    let shift_val = shift.get(&p_char).unwrap();
                    i += shift_val;
                    matched = false;
                    break;
                }
            }
            if matched {
                candidate.push((line_idx + 1, line.to_string()));    //Adds first match word only
                break;
            }
        }
    }
    candidate
}
fn usage() {
    println!("Usage:");
    println!("cargo run <target> <filepath>");
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("\x1b[31mErr\x1b[0m:Insufficient Args");
        usage();
        exit(1);
    }
    // println!("{:?}",args);
    let path = &args[2].trim();
    let pattern = &args[1].trim();

    if path.is_empty() || pattern.is_empty() {
        println!("Err: Invalid Args");
        usage();
    }

    /* let pattern: &str = "Hello"; */
    let table: HashMap<char, usize> = shift_table(pattern);
    let res: Vec<(usize,String)> = search(table, pattern, path);
    
    for (l_no,text) in res{
        println!("Line {}: {}",l_no,text);
    }

}
