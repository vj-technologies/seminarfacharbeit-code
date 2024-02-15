#![allow(unused)]
mod bubble_sort;
mod selection_sort;
mod block_sort;

use bubble_sort::bubble_sort;
use selection_sort::selection_sort;
use block_sort::block_sort;

// third party crate(s)
use random::Source;
// ------------------

fn main() {
    // create random arrays
    let length = 1_000;
    let mut array = generate_random_numbers(length, None);

    let durations = test_algorithm(&bubble_sort, array.clone());
    std::fs::write("bubble_sort.txt", format!("{:?}", durations))
        .expect("Failed to write to file.");

    let durations = test_algorithm(&selection_sort, array.clone());
    std::fs::write("selection_sort.txt", format!("{:?}", durations))
        .expect("Failed to write to file.");
}


fn test_algorithm(f: &dyn Fn(&mut Vec<i32>), mut array: Vec<i32>) -> Vec<u64> {
    let mut durations: Vec<u64> = vec![];
    for _ in 0..1000 {
        let start = unsafe{ core::arch::x86_64::_rdtsc() };
        f(&mut array);
        let cpu_cycles = unsafe{ core::arch::x86_64::_rdtsc() } - start;
        if !verify_sorted_ascending(&array) {
            panic!("Halt! Algorithm is not sorting correctly!");
        }

        durations.push(cpu_cycles);
    }
    durations
}


fn generate_random_numbers(length: usize, seed: Option<u64>) -> Vec<i32> {
    let mut rng = random::default(seed.unwrap_or(0));

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