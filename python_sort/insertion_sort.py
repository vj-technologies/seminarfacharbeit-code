def selection_sort(array: list[int]):
    for i in range(len(array)):
        jmin = i
        for j in range(i+1, len(array)):
            if array[j] < array[jmin]:
                jmin = j
        array[i], array[jmin] = array[jmin], array[i]
