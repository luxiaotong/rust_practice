fn main() {
    let s1 = String::from("hello");
    let x = compute_len(&s1);
    println!("len of string: {}", x);
}

fn compute_len(s: &String) -> usize {
    s.len()
}
