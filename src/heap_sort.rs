

pub fn heap_sort(array: &mut Vec<i32>) {
    let length = array.len();
    let half = length / 2;

    for i in (0..=half).rev() {
        heapify(array, i, length.clone());
    }

    for i in (1..length).rev() {
        array.swap(0, i);
        heapify(array, 0, i);
    }
}

fn heapify(array: &mut Vec<i32>, i: usize, n: usize) {
    let mut max = i;
    let child_left = 2*i +1;
    let child_right = 2*i +2;

    if child_left < n && array[i] < array[child_left] {
        max = child_left;
    }
    if child_right < n && array[max] < array[child_right] {
        max = child_right;
    }

    if max != i {
        array.swap(i, max);
        heapify(array, max, n);
    }
}
