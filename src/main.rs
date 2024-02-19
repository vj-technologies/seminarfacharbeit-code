mod bubble_sort;
mod selection_sort;
mod instertion_sort;
mod heap_sort;

use random::Source;
use serde::{ Serialize, Deserialize };
use serde_json;

use bubble_sort::bubble_sort;
use selection_sort::selection_sort;
use instertion_sort::insertion_sort;
use heap_sort::heap_sort;

#[derive(Serialize, Deserialize, Default)]
struct Benchmark {
    array_size: usize,
    array_seed: u64,
    time_bubble   : Vec<u64>,
    time_selection: Vec<u64>,
    time_insertion: Vec<u64>,
    time_heap     : Vec<u64>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let array_size = args[1].trim().parse()
        .expect("First argument must be a positive integer.");

    let array_seed = args[2].trim().parse()
        .expect("Second argument mmust be a positive integer.");

    let array = generate_random_numbers(array_size, array_seed);

    let bench = Benchmark {
        array_size,
        array_seed,
        time_bubble:    test_algorithm(&bubble_sort,    array.clone()),
        time_selection: test_algorithm(&selection_sort, array.clone()),
        time_insertion: test_algorithm(&insertion_sort, array.clone()),
        time_heap:      test_algorithm(&heap_sort,      array.clone()),
    };

    let contents = serde_json::to_string(&bench)
        .expect("Failed to parse as JSON.");

    let filename = format!("bench_{}_{}.json", array_size, array_seed);
    std::fs::write(filename, contents)
        .expect("Failed to write file.");
}


fn test_algorithm(f: &dyn Fn(&mut Vec<i32>), mut array: Vec<i32>) -> Vec<u64> {
    let mut durations: Vec<u64> = vec![];
    for _ in 0..1000 {
        let start = unsafe{ core::arch::x86_64::_rdtsc() };
        f(&mut array);
        let cpu_cycles = unsafe{ core::arch::x86_64::_rdtsc() } - start;
        if !verify_sorted_ascending(&array) {
            panic!("Algorithm is not sorting correctly!");
        }

        durations.push(cpu_cycles);
    }
    durations
}


fn generate_random_numbers(length: usize, seed: u64) -> Vec<i32> {
    let mut rng = random::default(seed);

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
