use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn get_map(key: &str, len:usize) -> HashMap<String, Progress> {
    use Progress::*;

    let mut map: HashMap<String,Progress> = HashMap::new();
    for i in 0..len+1 {
        let progress = if i > (3 * len / 4) { // > 25%
            Complete
        } else if i > (len / 2) {
            Some
        } else {
            None
        };
        map.insert(format!("{}{}", key, i), progress);
    }
    map
}

fn count_iterator(map: &HashMap<String, Progress>, value:Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    map.iter().filter(|(_,progress)| *progress == &value ).count()
}

fn main() {
    println!("Hello, world!");
    println!("{}", get_map("a", 100).iter()
        .filter(|(_,progress)| *progress == &Progress::Complete)
        .count());
    println!("{}", count_iterator(&get_map("a", 100), Progress::Complete));
}
