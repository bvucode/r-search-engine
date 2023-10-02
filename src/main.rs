use std::fs;
use std::io;
use std::io::stdin;
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Read;
use std::env::args;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsString;
use std::collections::HashMap;
extern crate tokenize;
use tokenize::tokenize::words;
extern crate rfts;
use rfts::rfts::{indexing, ftsearch, update};

fn helper() {
    println!("usage: main.py [-h] [file_paths file_words output_file]\nsearch engine\noptional arguments: \n-h, --help show this help message\n");
}

fn pathwalk(listpaths:Vec<String>, listmemo: &mut Vec<String>) -> Vec<String> {
    let mut listdirs = vec![];
    for i in 0..listpaths.len() {
        let paths = std::fs::read_dir(listpaths[i].trim_end()).unwrap();
        for entry in paths {
            let p = entry.expect("REASON").file_name();
            let path = PathBuf::from(listpaths[i].to_owned() + &p.clone().into_string().unwrap());
            if path.is_dir() {
                listdirs.push(listpaths[i].to_owned() + &p.clone().into_string().unwrap() + "/");
            }
            if path.is_file() {
                listmemo.push(listpaths[i].to_owned() + &p.clone().into_string().unwrap());
            }
        }
    }
    if listdirs.len() == 0 {
        return listmemo.to_vec()
    }
    pathwalk(listdirs, listmemo)
}

fn indexer(fpaths:String, fwords:String, ofile:String) {
    let mut upd: &mut HashMap<String, Vec<(u32, u32)>> = &mut HashMap::new();
    let mut contents3 = String::new();
    let mut directories = vec![];
    let mut xlist = vec![];
    let mut ylist = vec![];
    let mut listmemo = vec![];
    let mut tok2:String = Default::default();
    let mut data = String::new();
    let file = File::open(fpaths.clone()).expect("Unable to open file");
    let file = BufReader::new(file);
    for line in file.lines() {
        let line = line.expect("REASON");
        ylist.push(line.to_string());
    }
    directories = pathwalk(ylist, &mut listmemo);
    for i in 0..directories.len() {
        let mut contents2 = String::new();
        let mut tok:String = Default::default();
        let file2 = File::open(directories[i].to_owned());
        file2.expect("REASON").read_to_string(&mut contents2).unwrap();
        {
            let x = &mut tok;
            *x = words(contents2);
        }
        let v = tok.split(" ").map(String::from).collect::<Vec<String>>();
        let ind = indexing(v, i.try_into().unwrap());
        if i == 0 {
            *upd = ind;
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
        data += &(directories[k as usize].clone() + "\n");
    }
    fs::write(ofile, data).expect("Unable to write file");
}

fn main() {
    let mut flag = 0;
    let file_path = args();
    let mut namefile = vec![];
    for arg in file_path {
        if args().len() == 2 && arg == "-h" {
            flag = 1;
            helper();
            break;
        }else if args().len() == 4 {
            namefile.push(arg);
        }
    }
    if flag == 0 {
        indexer(namefile[1].to_string(), namefile[2].to_string(), namefile[3].to_string());
    }
}