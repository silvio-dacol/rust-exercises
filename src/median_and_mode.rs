// Given a list of integers, use a vector and return
// the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; 
// a hash map will be helpful here) of the list.

use rand::Rng;
use std::collections::HashMap;

pub fn median_and_mode () {
    let mut rng = rand::thread_rng();

    let mut list_of_integers: Vec<i32> = Vec::new();
    let mut number_of_elements: i32 = 20;

    while number_of_elements > 0 {
        list_of_integers.push(rng.gen_range(1..10));
        number_of_elements-=1;
    }

    println!("The vector is {:?}", &list_of_integers);
    println!("The median is {} and the mode is {}", median(&list_of_integers), mode(&list_of_integers, 0));
}

fn median (list_of_integers: &[i32]) -> f32 {
    let mut copy = list_of_integers.to_vec(); // We work on a copy of the vector

    copy.sort_unstable();

    let length = copy.len();
    
    if length % 2 == 1 {
        copy[length / 2] as f32
    } else {
        (((copy[(length / 2) - 1] as f32) + (copy[length / 2] as f32)) / 2.0) as f32
    }
}

fn mode(list_of_integers: &[i32], default: i32) -> i32 {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    
    for &x in list_of_integers {
        *freq.entry(x).or_insert(0) += 1;
    }
    
    freq.into_iter()
        .max_by_key(|&(_k, v)| v)
        .map(|(k, _v)| k)
        .unwrap_or(default)
}
