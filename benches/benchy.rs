use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::HashMap;
use rayon::prelude::*;


#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_collection_flatten(collection: &[HashMap<String, Progress>], value:Progress) -> usize {
    collection.iter()
        .flatten()
        .filter(|(_,progress)| *progress == &value)
        .count()
}

fn count_collection_flatten2(collection: &[HashMap<String, Progress>], value:Progress) -> usize {
    collection.iter()
        .flatten()
        .filter(|(_,progress)| if *progress == &value {true} else {false})
        .count()
}

fn count_collection_kyranFix(collection: &[HashMap<String,Progress>], value:Progress) -> usize {
    collection.iter()
        .flatten()
        .map(|(_,progress)| if *progress == value { 1 } else { 0 })
        .sum()
}

fn count_collection_fold(collection: &[HashMap<String, Progress>], value:Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, .
    collection.iter()
        .fold(0, |acc,hash|
            acc + hash.iter().filter(|(_,progress)| *progress == &value).count()
        )
}


fn count_collection_map(collection: &[HashMap<String, Progress>], value:Progress) -> usize {
    collection.iter()
        .map(|hash| count_iterator(hash,value))
        .sum()
}

fn count_iterator(map: &HashMap<String, Progress>, value:Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    map.iter().filter(|(_,progress)| *progress == &value ).count()
}

fn count_collection_parralel(collection: &[HashMap<String, Progress>], value:Progress) -> usize {
    collection
        .par_iter()
        .flat_map_iter(HashMap::values)
        .filter(|progress| *progress == &value)
        .count()
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


fn get_vec_map(lenx: usize, leny:usize) -> Vec<HashMap<String, Progress>> {
    use Progress::*;
    let mut vec: Vec<HashMap<String,Progress>> = Vec::with_capacity(lenx);
    for i in 0..vec.len(){
        vec[i] = get_map(&format!("vec,{},{}", lenx,leny), leny);
    };
    vec
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections");
    //let hashCollection = get_vec_map(1);
    let limity = 1001; // How large hash map can be
    for i in 900..limity {
            let val = get_vec_map(10,i);
            group.bench_with_input(BenchmarkId::new("Flatten",i),
                                   &(&val,Progress::Complete),
                                   |f,(vec,val)|
                                       f.iter(||count_collection_flatten(vec, *val))
            );
            group.bench_with_input(BenchmarkId::new("Flatten2",i),
                                   &(&val,Progress::Complete),
                                   |f,(vec,val)|
                                       f.iter(||count_collection_flatten2(vec, *val))
            );
            group.bench_with_input(BenchmarkId::new("Kyran Map",i),
                                   &(&val,Progress::Complete),
                                   |f,(vec,val)|
                                       f.iter(||count_collection_kyranFix(vec, *val))
            );
            group.bench_with_input(BenchmarkId::new("fold",i),
                                   &(&val,Progress::Complete),
                                   |f,(vec,val)|
                                       f.iter(||count_collection_fold(vec, *val))
            );
            group.bench_with_input(BenchmarkId::new("Map",i),
                                   &(&val,Progress::Complete),
                                   |f,(vec,val)|
                                       f.iter(||count_collection_map(vec, *val))
            );
            group.bench_with_input(BenchmarkId::new("Parralel", i),
                                   &(&val,Progress::Complete),
                                   |f,(vec,val)|
                                       f.iter(||count_collection_parralel(vec, *val))
            );
        };
    for i in 0..100 {
        let val = get_vec_map(10,i);
        group.bench_with_input(BenchmarkId::new("Flatten",i),
                               &(&val,Progress::Complete),
                               |f,(vec,val)|
                                   f.iter(||count_collection_flatten(vec, *val))
        );
        group.bench_with_input(BenchmarkId::new("Flatten2",i),
                               &(&val,Progress::Complete),
                               |f,(vec,val)|
                                   f.iter(||count_collection_flatten2(vec, *val))
        );
        group.bench_with_input(BenchmarkId::new("Kyran Map",i),
                               &(&val,Progress::Complete),
                               |f,(vec,val)|
                                   f.iter(||count_collection_kyranFix(vec, *val))
        );
        group.bench_with_input(BenchmarkId::new("fold",i),
                               &(&val,Progress::Complete),
                               |f,(vec,val)|
                                   f.iter(||count_collection_fold(vec, *val))
        );
        group.bench_with_input(BenchmarkId::new("Map",i),
                               &(&val,Progress::Complete),
                               |f,(vec,val)|
                                   f.iter(||count_collection_map(vec, *val))
        );
        group.bench_with_input(BenchmarkId::new("Parralel", i),
                               &(&val,Progress::Complete),
                               |f,(vec,val)|
                                   f.iter(||count_collection_parralel(vec, *val))
        );
    };
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);