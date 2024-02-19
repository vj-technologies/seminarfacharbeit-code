def bubble_sort(array: list[int]) -> list[int]:
    for _ in range(1, len(array)):
        for i in range(1, len(array)):
            if array[i-1] < array[i]:
                continue
            array[i], array[i-1] = array[i-1], array[i]
    
    return array
