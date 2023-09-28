use std::io;
use std::io::Write;
use std::io::stdin;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ffi::OsString;
use std::collections::HashMap;
extern crate tokenize;
use tokenize::tokenize::words;
extern crate rfts;
use rfts::rfts::{indexing, ftsearch, update};

fn main() {
    let mut upd: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    let paths = std::fs::read_dir("./my_folder").unwrap();
    let mut vf = vec![];
    let mut inptok:String = Default::default();
    let folder = "./my_folder/".to_string();
    for path in paths {
        vf.push(path.expect("REASON").file_name());
    }
    for i in 0..vf.len() {
        let mut contents = String::new();
        let mut tok:String = Default::default();
        let file = File::open(folder.clone() + &vf[i].clone().into_string().unwrap());
        file.expect("REASON").read_to_string(&mut contents).unwrap();
        {
            let x = &mut tok;
            *x = words(contents);
        }
        let v = tok.split(" ").map(String::from).collect::<Vec<String>>();
        let ind = indexing(v, i.try_into().unwrap());
        if i == 0 {
            upd = ind;
        } else {
            upd = update(ind, upd);
        }
    }
    loop {
        let mut xlist = vec![];
        print!("Enter words: ");
        io::stdout().flush();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        inptok = words(line.trim_end().to_string());
        let v2 = inptok.split(" ").map(String::from).collect::<Vec<String>>();
        for i in v2 {
            if !xlist.contains(&i) {
                xlist.push(i);
            }
        }
        let var = ftsearch(xlist, upd.clone());
        println!("{:?}", var);
    }
}