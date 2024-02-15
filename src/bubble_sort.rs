pub fn bubble_sort(array: &mut Vec<i32>) {
    for _ in 1..array.len() {
        for i in 1..array.len() {
            if array[i-1] < array[i] {
                continue;
            }
            array.swap(i-1, i);
        }
    }
}
