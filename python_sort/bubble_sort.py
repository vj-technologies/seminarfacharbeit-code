def bubble_sort(array: list[int]):
    for _ in range(len(array)-1):
        for i in range(len(array)-1):
            if array[i] > array[i+1]:
                array[i], array[i+1] = array[i+1], array[i]
