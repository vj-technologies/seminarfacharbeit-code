#![allow(unused)]
use cpu_time::ProcessTime;
use random::Source;

fn main() {
    println!("Bubble sort:");
    test_algorithm(&bubble_sort);

    println!("Selection sort:");
    test_algorithm(&selection_sort);
}

fn test_algorithm(f: &dyn Fn(&mut Vec<i32>)) {
    let length = 10_000;
    let mut array = generate_random_numbers(length, None);
    
    let start = ProcessTime::now();
    f(&mut array);
    let cpu_time = start.elapsed();

    if verify_sorted_ascending(&array) {
        println!("{} ms for {:?} elements", cpu_time.as_millis(), length);
    }
    else {
        println!("Array is not sorted!");
        println!("{:?}", array);
    }
}

fn generate_random_numbers(length: usize, seed: Option<u64>) -> Vec<i32> {
    let mut rng = random::default(seed.unwrap_or(42));

    rng.iter().take(length).collect::<Vec<i32>>()
}

fn verify_sorted_ascending(array: &Vec<i32>) -> bool {
    let mut prev = array[0];
    for curr in array.iter().skip(1) {
        if prev > *curr {
            return false
        }
        prev = *curr;
    }

    true
}

fn bubble_sort(array: &mut Vec<i32>) {
    for _ in 1..array.len() {
        for i in 1..array.len() {
            if array[i-1] < array[i] {
                continue;
            }
            array.swap(i-1, i);
        }
    }
}

fn selection_sort(array: &mut Vec<i32>) {
    for i in 0..array.len()-1 {
        let mut min: usize = i;
        for j in i+1..array.len() {
            if array[min] > array[j] {
                min = j;
            }
        }
        array.swap(min, i);
    }
}
