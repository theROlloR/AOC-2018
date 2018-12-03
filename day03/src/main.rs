use std::collections::{HashSet, HashMap};

fn main() {
    let data = include_str!("input.txt");
    let c = data.lines().collect::<Vec<_>>();
    let mut claims = HashMap::new();
    let mut claim_names = HashMap::new();
    let mut intersecting = HashSet::new();
    let mut all = HashSet::new();
    for i in c.iter() {
        let r = i.split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#').filter_map(|c| c.parse::<usize>().ok()).collect::<Vec<_>>();
        for i in r[1]..r[1]+r[3] {
            for j in r[2]..r[2]+r[4] {
                *claims.entry((i,j)).or_insert(0) += 1;
                all.insert(r[0]);
                if !claim_names.contains_key(&(i,j)) {
                    claim_names.insert((i,j), r[0]);
                } else {
                    intersecting.insert(claim_names[&(i,j)]);
                    intersecting.insert(r[0]);
                }
            }
        }
    }
    let out1 = claims.values().filter(|v| **v > 1).count();
    println!("I: {}", out1);
    let out2 = all.difference(&intersecting).next();
    println!("II: {:?}", out2);
}
