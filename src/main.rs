extern crate walkdir;
extern crate regex;

use walkdir::WalkDir;
use regex::Regex;

use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::str;
use std::string::String;
use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;
use std::thread;

fn search_in_file(re: Regex, entry: walkdir::DirEntry, tx: Sender<String>) {
    let mut data = String::new();

    let f = File::open(entry.path())
        .expect("Unable to open file");

    let mut br = BufReader::new(f);
    let mut line_text = String::new();
    let mut line_no: u32 = 0;

    while br.read_line(&mut line_text).unwrap_or(0) > 0 {
        line_no += 1;
        if re.is_match(&line_text) {
            if 0 == data.len() {
                data.push('\n');
                data.push_str(entry.path().to_str().unwrap());
                data.push('\n');
            }

            data.push_str(&line_no.to_string());
            data.push_str(": ");
            data.push_str(&line_text);
        }
        line_text.clear()
    }

    let _ = tx.send(data);
}

fn search_entry(re: &Regex, entry: &walkdir::DirEntry) -> Receiver<String> {
    let (tx, rx) = mpsc::channel();

    if !entry.path().is_file() {
        let _ = tx.send(String::new());
    } else {
        let re = re.clone();
        let e = entry.clone();
        let sender = tx.clone();
        thread::spawn(move || {
            search_in_file(re, e, sender);
        });
    }

    rx
}


fn main() {
    let mut args = env::args_os();
    let text_to_find = args.nth(1).unwrap();
    let re = Regex::new(text_to_find.to_str().unwrap()).unwrap();

    let mut results = Vec::new();

    for entry in WalkDir::new(".").follow_links(true) {
        let entry = entry.unwrap();
        let rx = search_entry(&re, &entry);
        results.push(rx);
    }

    for rx in &results {
        print!("{}", rx.recv().unwrap());
    }
}
