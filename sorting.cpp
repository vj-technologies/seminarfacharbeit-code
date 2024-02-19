#include <stdio.h>
#include<array>
#include<cstdlib>
using namespace std;
const int Len = 10000;

array<int, Len> stupid = { 9, 4, 8, 3, 6, 5, 1, 2, 7, 0 };


void print_array(array<int, Len> barray) { // FIXME
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


array<int, Len> selectionsort(array<int, Len> ret_array)
{
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
    return ret_array;

}

array<int, Len> bubble_sort(array<int, Len> ret_array) {
    for (int i = 1; i < Len; ++i) {
        for (int i = 1; i < Len; ++i) {
            if (ret_array[i - 1] > ret_array[i]) {
                int temp = ret_array[i - 1];
                ret_array[i - 1] = ret_array[i];
                ret_array[i] = temp;
            }
        }
    }
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
    if (rightchild < n&& ret_array[max] < ret_array[rightchild])
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
array<int, Len> heapsort(array<int, Len> ret_array) {
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
    return ret_array;

}



int main() {

    // bubble_sort(&array); // FIXME

    array<int, Len> Fug = genarray();
    print_array(Fug);
    printf("\n");
    print_array(bubble_sort(Fug));
    printf("\n");
    print_array(selectionsort(Fug));
    printf("\n");
    print_array(heapsort(Fug));


    return 0;
}

