use std::collections::HashMap;

pub fn indexing(v:Vec<String>, ind:u32) -> HashMap<String, Vec<(u32, u32)>> {
    let mut dict = HashMap::new();
    for i in &v {
        if !dict.contains_key(i) {
            let mut tv2 = vec![];
            let c:u32 = v.iter().filter(|n| n == &i).count().try_into().unwrap();
            tv2.push((ind, c));
            dict.insert(i.to_string(), tv2);
        }
    }
    return dict;
}

pub fn ftsearch(xv:Vec<String>, xdict:HashMap<String, Vec<(u32, u32)>>) -> HashMap<u32, u32> {
    let mut dict = HashMap::new();
    let mut v = vec![];
    for i in &xv {
        if xdict.contains_key(i) {
            v.push(xdict.get(i).unwrap());
        }
    }
    for i in v {
        for j in i {
            if !dict.contains_key(&j.0) {
                dict.insert(j.0, j.1);
            }else{
                dict.insert(j.0, j.0 + j.1);
            }
        }
    }
    return dict;
}

pub fn update<'a>(indh:HashMap<String, Vec<(u32, u32)>>, oldh: HashMap<String, Vec<(u32, u32)>>) -> HashMap<String, Vec<(u32, u32)>>{
    let mut newh:HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    for (k, v) in indh {
        if oldh.contains_key(&k) {
            let mut t = vec![];
            let w = oldh.get(&k).unwrap();
            for i in w {
                t.push(*i);
            }
            for i in &v {
                t.push(*i);
            }
            newh.insert(k, t);
        }
        else if !oldh.contains_key(&k) {
            newh.insert(k, v);
        }
    }
    return newh
}