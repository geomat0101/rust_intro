
// Given a list of integers, use a vector and return the median 
//      (when sorted, the value in the middle position) and mode 
//      (the value that occurs most often; a hash map will be 
//      helpful here) of the list.

fn median(list: &mut Vec<i32>) -> i32 {
    list.sort_unstable();
    list[list.len()/2]
}


use std::collections::HashMap;

fn mode(list: &Vec<i32>) -> i32 {
    let mut freq = HashMap::new();
    for i in list {
        *freq.entry(*i).or_insert(0) += 1;
    }
    println!("frequency hash: {:#?}", freq);

    let top_val = 
        freq.iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
            
    *top_val.0
}


pub fn run () {
    let mut list = vec![5, 3, 1, 2, 4, 5, 1, 5, 2];
    println!("orig list: {:?}", list);

    let median = median(&mut list);
    println!("new list: {:?}", list);

    let mode = mode(&list);

    println!("median: {}, mode: {}", median, mode);
}