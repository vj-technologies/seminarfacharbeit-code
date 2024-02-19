#include <stdio.h>

int main() {
    int array[] = { 2, 4, 3, 9, 5, 2, 0, 8, 1, 7 };
    printf("%i", array[0]);
    print_array(array, 10);
    // bubble_sort(&array); // FIXME
    // print_array(array, 10);

    return 0;
}


void print_array(int *array[], int length) { // FIXME
    for (int i = 0; i < length; ++i) {
        printf("%i", array[i]);
    }
}

// void bubble_sort(int* array, int length) {
//     for (int i = 1; i < length; ++i) {
//         for (int i = 1; i < length; ++i) {
//             if (array[i-1] > array[i]) {
//                 swap_ij(&array[i], &array[i-1]); // FIXME
//             }
//         }
//     }
// }

// void swap_ij(int *i, int *j) { // FIXME
//     int temp = *i;
//     *i = *j;
//     *j = temp;
// }
