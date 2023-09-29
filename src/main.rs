use std::fs;
use std::io;
use std::io::stdin;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use std::env::args;
use std::path::Path;
use std::ffi::OsString;
use std::collections::HashMap;
extern crate tokenize;
use tokenize::tokenize::words;
extern crate rfts;
use rfts::rfts::{indexing, ftsearch, update};

fn helper() {
    println!("usage: main.py [-h] [file_path file_words output_file]\nsearch engine\noptional arguments: \n-h, --help show this help message\n");
}

fn indexer(fpaths:String, fwords:String, ofile:String) {
    let mut upd: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    let mut contents = String::new();
    let mut contents3 = String::new();
    let mut directories = vec![];
    let mut xlist = vec![];
    let mut tok2:String = Default::default();
    let paths = std::fs::read_dir(fpaths.clone()).unwrap();
    for path in paths {
        directories.push(path.expect("REASON").file_name());
    }
    for i in 0..directories.len() {
        let mut contents2 = String::new();
        let mut tok:String = Default::default();
        let file2 = File::open(fpaths.clone() + &directories[i].clone().into_string().unwrap());
        file2.expect("REASON").read_to_string(&mut contents2).unwrap();
        {
            let x = &mut tok;
            *x = words(contents2);
        }
        let v = tok.split(" ").map(String::from).collect::<Vec<String>>();
        let ind = indexing(v, i.try_into().unwrap());
        if i == 0 {
            upd = ind;
        } else {
            upd = update(ind, upd);
        }
    }
    let file3 = File::open(fwords);
    file3.expect("REASON").read_to_string(&mut contents3).unwrap();
    tok2 = words(contents3);
    let v2 = tok2.split(" ").map(String::from).collect::<Vec<String>>();
    for i in v2 {
        if !xlist.contains(&i) {
            xlist.push(i);
        }
    }
    let var = ftsearch(xlist, upd.clone());
    for (k, v) in var {
        println!("{}", k)
    }
}

fn main() {
    let file_path = args();
    let mut namefile = vec![];
    for arg in file_path {
        if args().len() == 2 && arg == "-h" {
            helper();
            break;
        }else if args().len() == 4 {
            namefile.push(arg);
        }
    }
    indexer(namefile[1].to_string(), namefile[2].to_string(), namefile[3].to_string());
}