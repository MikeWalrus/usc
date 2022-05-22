#![feature(let_chains)]
use std::{io::{BufRead, stdin}, collections::HashMap};
use unicode_script::{UnicodeScript, Script};

fn main() {
    count(stdin().lock());
}

fn count<R: BufRead>(mut r: R) {
    let mut buf = String::with_capacity(1024);
    let mut script_count: HashMap<Script, usize> = HashMap::new();
    while let Ok(n) = r.read_line(&mut buf) && n > 0{
        for c in buf.chars() {
            let script = c.script();
            let count = script_count.entry(script).or_insert(0);
            *count += 1;
        }
    }
    for (script, count) in script_count {
        println!("{script}\t{count}")
    }
}