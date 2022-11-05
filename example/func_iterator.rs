fn main() {
    iterator_sum();
    iterator_map();
    filters_by_size();
    calling_next_directly();
    using_other_iterator_trait_methods()
}
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    println!("total: {}", total);
    assert_eq!(total, 6);
}

fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("in_my_size: {:?}", in_my_size);
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn calling_next_directly() {
    let mut counter = Counter::new();

    println!("count: {:?}", counter.next());
    println!("count: {:?}", counter.next());
    println!("count: {:?}", counter.next());
    println!("count: {:?}", counter.next());
    println!("count: {:?}", counter.next());
    println!("count: {:?}", counter.next());
}

fn using_other_iterator_trait_methods() {
    println!("zip: {:?}", Counter::new().zip(Counter::new().skip(1)));
    let zip_map:Vec<_> = Counter::new().zip(Counter::new().skip(1)).map(|(a,b)| a*b).collect();
    println!("zip_map: {:?}", zip_map);
    // let sum: u32 = Counter::new().zip(Counter::new().skip(1))
    //                              .map(|(a, b)| a * b)
    //                              .filter(|x| x % 3 == 0)
    //                              .sum();
    // assert_eq!(18, sum);
}
