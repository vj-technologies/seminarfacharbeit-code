def heapify(array: list[int], i: int, n: int):
    max = i
    child_left = 2*i +1
    child_right = 2*i +2

    if child_left < n and array[i] < array[child_left]:
        max = child_left
    
    if child_right < n and array[max] < array[child_right]:
        max = child_right

    if max != i:
        array[i], array[max] = array[max], array[i]
        heapify(array, max, n)


def heap_sort(array: list[int]):
    n = len(array)
    for i in range(n//2, -1, -1):
        heapify(array, i, n)
    
    for i in range(n-1, 0, -1):
        array[0], array[i] = array[i], array[0]
        heapify(array, 0, i)
