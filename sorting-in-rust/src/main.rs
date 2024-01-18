use cpu_time::ProcessTime;
use random::Source;

fn main() {
    let length = 10_000;
    let mut array = generate_random_numbers(length, None);

    print!("Bubble sort ");
    let start = ProcessTime::now();
    bubble_sort(&mut array);
    let cpu_time = start.elapsed();
    println!("takes {} milliseconds to sort {:?} elements.", cpu_time.as_millis(), length);
}

fn generate_random_numbers(length: usize, seed: Option<u64>) -> Vec<i32> {
    let mut rng = random::default(seed.unwrap_or(42));

    rng.iter().take(length).collect::<Vec<i32>>()
}

fn bubble_sort(array: &mut Vec<i32>) {
    for _ in 1..array.len() {
        for i in 1..array.len() {
            if array[i-1] >= array[i] {
                continue;
            }
            array.swap(i-1, i);
        }
    }
}