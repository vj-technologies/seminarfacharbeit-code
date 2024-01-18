#![allow(unused)]
use std::time::Duration;
use cpu_time::ProcessTime;
use random::Source;


fn main() {
    let length = 10_000;
    let mut array = generate_random_list(length);
    
    print!("Bubble sorting...");
    let start = ProcessTime::now();
    bubble_sort(&mut array);
    let cpu_time: Duration = start.elapsed();
    println!("Done in {} milliseconds for {} elements.", cpu_time.as_millis(), length);
}

fn generate_random_list(length: i32) -> Vec<i32> {
    let mut rng = random::default(42);

    rng.iter().take(length as usize).collect::<Vec<i32>>()
}

fn bubble_sort(array: &mut Vec<i32>) {
    for _ in 0..array.len() - 1 {
        for i in 0..array.len() - 1 {
            if array[i] <= array[i + 1] {
                continue;
            }
            array.swap(i, i + 1);
        }
    }
}
