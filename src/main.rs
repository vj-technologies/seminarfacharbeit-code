mod bubble_sort;
mod selection_sort;
mod heap_sort;

use random::Source;
use serde::{ Serialize, Deserialize };
use serde_json;

use bubble_sort::bubble_sort;
use selection_sort::selection_sort;
use heap_sort::heap_sort;

#[derive(Serialize, Deserialize, Default)]
struct Benchmark {
    array_size: usize,
    array_seed: u64,
    test_count: u32,
    time_bubble   : Vec<u128>,
    time_selection: Vec<u128>,
    time_heap     : Vec<u128>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 4 {
        println!("3 arguments required: array_size array_seed test_count");
        return
    }
    
    let array_size = args[1].trim().parse()
        .expect("First argument must be a positive integer.");
    let array_seed = args[2].trim().parse()
        .expect("Second argument must be a positive integer.");
    let test_count = args[3].trim().parse()
        .expect("Third argument must be a positive integer.");

    let array = generate_random_numbers(array_size, array_seed);

    let bench = Benchmark {
        array_size,
        array_seed,
        test_count,
        time_bubble:    test_algorithm(&bubble_sort,    array.clone(), test_count),
        time_selection: test_algorithm(&selection_sort, array.clone(), test_count),
        time_heap:      test_algorithm(&heap_sort,      array.clone(), test_count),
    };

    let contents = serde_json::to_string(&bench)
        .expect("Failed to parse as JSON.");

    let filename = format!("data/rust_{}_{}_{}.json", array_size, array_seed, test_count);
    std::fs::write(filename, contents)
        .expect("Failed to write file.");
}


fn test_algorithm(f: &dyn Fn(&mut Vec<i32>), mut array: Vec<i32>, test_count: u32) -> Vec<u128> {
    let mut durations: Vec<u128> = vec![];
    for _ in 0..test_count {
        // let start = unsafe{ core::arch::x86_64::_rdtsc() }; // cpu cycles
        let start = std::time::Instant::now();
        f(&mut array);
        // let time = unsafe{ core::arch::x86_64::_rdtsc() } - start; // cpu cycles
        let time = start.elapsed();
        if !verify_sorted_ascending(&array) {
            panic!("Algorithm is not sorting correctly!");
        }
        durations.push(time.as_nanos());
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
