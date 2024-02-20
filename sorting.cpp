#include <stdio.h>
#include<array>
#include<cstdlib>
#include<string>
#include <chrono>
using namespace std;
const int Len = 1000;
array<std::chrono::nanoseconds, Len> Selection_sort_l;
array<std::chrono::nanoseconds, Len> bubble_sort_l;
array<std::chrono::nanoseconds, Len> heapsort_l;





void print_array_time(array<std::chrono::nanoseconds, Len> barray) { // FIXME
    for (int i = 0; i < Len; ++i) {
        printf("%i ", barray[i]);
    }
}

void print_array_int(array<int, Len> barray) { // FIXME
    for (int i = 0; i < Len; ++i) {
        printf("%i ", barray[i]);
    }
}

array<int, Len> genarray() {
    array<int, Len> ret_array;
    for (int i = 0; i < Len; i++)
    {
        int random = rand() % Len;
        ret_array[i] = random;

    }
    return ret_array;

}


array<int, Len> selectionsort(array<int, Len> ret_array, int b)
{
    auto begin = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < Len; ++i)
    {
        int jmin = i;
        for (int j = i + 1; j < Len; ++j)
        {
            if (ret_array[j] < ret_array[jmin]) {

                jmin = j;
            }
        }
        int temp = ret_array[jmin];
        ret_array[jmin] = ret_array[i];
        ret_array[i] = temp;



    }
    auto end = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(end - begin);
    Selection_sort_l[b] = elapsed;
    return ret_array;

}

array<int, Len> bubble_sort(array<int, Len> ret_array, int b) {
    auto begin = std::chrono::high_resolution_clock::now();
    for (int i = 1; i < Len; ++i) {
        for (int i = 1; i < Len; ++i) {
            if (ret_array[i - 1] > ret_array[i]) {
                int temp = ret_array[i - 1];
                ret_array[i - 1] = ret_array[i];
                ret_array[i] = temp;
            }
        }
    }
    auto end = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(end - begin);
    bubble_sort_l[b] = elapsed;
    return ret_array;
}


void heapify(array<int, Len>& ret_array, int i, int n) {

    int max = i;
    int leftchild = 2 * i + 1;
    int rightchild = 2 * i + 2;

    if (leftchild < n && ret_array[i] < ret_array[leftchild])
    {
        max = leftchild;
    }
    if (rightchild < n && ret_array[max] < ret_array[rightchild])
    {
        max = rightchild;

    }
    if (max != i)
    {
        int temp = ret_array[max];
        ret_array[max] = ret_array[i];
        ret_array[i] = temp;
        heapify(ret_array, max, n);
    }

}
array<int, Len> heapsort(array<int, Len> ret_array, int b) {
    auto begin = std::chrono::high_resolution_clock::now();
    for (int i = Len / 2; i > -1; --i)
    {
        heapify(ret_array, i, Len);

    }
    for (int i = Len - 1; i > 0; --i) {
        int temp = ret_array[0];
        ret_array[0] = ret_array[i];
        ret_array[i] = temp;
        heapify(ret_array, 0, i);

    }
    auto end = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(end - begin);
    heapsort_l[b] = elapsed;
    return ret_array;


}



int main() {

    // bubble_sort(&array); // FIXME

    for (int i = 0; i < Len; i++) {

        array<int, Len> arrays = genarray();
        bubble_sort(arrays, i);
        selectionsort(arrays, i);
        heapsort(arrays, i);

    }
    printf("arraysize: %i", Len);
    printf("\n");
    printf("test_count: %i", Len);
    printf("\n");
    printf("timen in : nanoseconds");
    printf("\n");
    printf("time_bubble: ");
    print_array_time(bubble_sort_l);
    printf("\n");
    printf("time_selection: ");
    print_array_time(Selection_sort_l);
    printf("\n");
    printf("time_heap: ");
    print_array_time(heapsort_l);

    return 0;
}