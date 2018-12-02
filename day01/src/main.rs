use std::collections::HashSet;

fn main() {
    let data = include_str!("input.txt");
    let c = data.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>();
    println!("I: {}", c.iter().sum::<i64>());
    let mut cache = HashSet::new();
    let mut sum = 0;
    let v = c.into_iter().cycle().find_map(|c| {
        sum += c;
        cache.replace(sum)
    }).unwrap();
    println!("II: {}", v);
}