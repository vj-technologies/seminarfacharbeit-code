pub fn insertion_sort(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        let j = binary_search_in_range(&array, 0, i-1, array[i]);
        array.swap(i, j);
    }
}

fn binary_search_in_range(array: &Vec<i32>, lo: usize, hi: usize, target: i32) -> usize {
    let mut lo: usize = lo;
    let mut hi: usize = array.len() - 1;
    let mid = hi >> 1 + lo >> 1;

    while hi - lo > 1 {
        let mid = (hi >> 1) + (lo >> 1);
        if array[mid] > target {
            hi = mid;
        }
        else if target > array[mid] {
            lo = mid;
        }
    }

    mid
}
