pub fn selection_sort(array: &mut Vec<i32>) {
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
