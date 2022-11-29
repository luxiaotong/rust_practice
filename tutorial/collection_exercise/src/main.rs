use std::collections::HashMap;
use std::io;

fn main() {
    let mut vv = vec![10, 9, 8, 7, 6];
    // println!("vv: {:?}, len: {}", vv, vv.len());
    bubble(&mut vv);
    println!("vv: {:?}, midean: {}", vv, vv[vv.len() / 2]);

    let vv2 = vec![10, 9, 8, 7, 6, 10];
    find_mode(&vv2);

    let s1 = String::from("first");
    pig_latin(&s1);
    let s1 = String::from("apple");
    pig_latin(&s1);

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    add_user(&mut company);
    add_user(&mut company);

    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

    let k = String::from("k1");
    if hash_map.contains_key(&k) {
        hash_map.get_mut(&k).unwrap().push(String::from("v1"));
    } else {
        let mut vv: Vec<String> = Vec::new();
        vv.push(String::from("v1"));
        hash_map.insert(k, vv);
    }
    let k = String::from("k1");
    if hash_map.contains_key(&k) {
        hash_map.get_mut(&k).unwrap().push(String::from("v2"));
    } else {
        let mut vv: Vec<String> = Vec::new();
        vv.push(String::from("v2"));
        hash_map.insert(k, vv);
    }
    println!("hash_map: {:?}", hash_map);
}

fn bubble(vv: &mut Vec<i32>) {
    let n = vv.len();
    for i in 0..n {
        for j in (i + 1)..n {
            // println!("i: {}, j: {}", i, j);
            if vv[i] > vv[j] {
                let tmp = vv[j];
                vv[j] = vv[i];
                vv[i] = tmp;
            }
        }
    }
}

fn find_mode(vv: &Vec<i32>) {
    let mut mm: HashMap<i32, i32> = HashMap::new();
    for v in vv {
        // println!("v: {}", v);
        let count = mm.entry(*v).or_insert(0);
        *count += 1;
    }
    let mut max = -1;
    let mut mode = -1;
    for (&k, &v) in mm.iter() {
        if v > max {
            mode = k;
            max = v;
        }
    }
    // for (k, v) in &mm {
    //     if *v > max {
    //         mode = *k;
    //         max = *v;
    //     }
    // }
    // let (mode, _max) = mm
    //     .clone()
    //     .into_iter()
    //     .max_by_key(|(_k, v)| *v)
    //     .unwrap_or((-1, -1));
    println!("vv: {:?}, mm: {:?}, mode: {}", vv, mm, mode)
}

fn pig_latin(s: &String) {
    let mut ss = String::new();
    let mut suffix = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i == 0 {
            if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
                suffix.push_str("-hay");
                ss.push(ch);
            } else {
                suffix.push('-');
                suffix.push(ch);
                suffix.push_str("ay");
            }
        } else {
            ss.push(ch);
        }
    }
    ss.push_str(&suffix);
    println!("pig: {}", ss);
}

fn add_user(company: &mut HashMap<String, Vec<String>>) {
    let mut user = String::new();
    let mut dept = String::new();

    println!("Enter user name:");
    io::stdin()
        .read_line(&mut user)
        .expect("failed to read line");
    let user = user.trim_end().to_string();

    println!("Enter department name:");
    io::stdin()
        .read_line(&mut dept)
        .expect("failed to read line");
    let dept = dept.trim_end().to_string();
    println!("Add {user} To {dept}, user len: {}", user.len());

    company
        .entry(dept.clone())
        .or_insert_with(|| Vec::new())
        .push(user.clone());

    println!("company: {:?}", company);
}
