import time, random
from bubble_sort import bubble_sort
from selection_sort import selection_sort
from heap_sort import heap_sort

def generate_random_numbers(length: int, seed: int) -> list[int]:
    array: list[int] = list(range(length))
    random.seed(seed)
    random.shuffle(array)
    return array


def verify_sorted_ascending(array: list[int]) -> bool:
    prev = array[0]
    for curr in array:
        if prev > curr:
            return False
        prev = curr
    
    return True


def test_algorithm(algorithm, array: list[int], test_count: int) -> list[int]:
    durations = []
    for _ in range(test_count):
        start = time.time_ns()
        result = algorithm(array)
        nanosecs = time.time_ns() - start
        res = verify_sorted_ascending(result)
        if not res:
            raise Exception("Algorithm is not sorting correctly!")
        durations.append(nanosecs)
    
    return durations


if __name__ == "__main__":
    import json

    data: dict = {
        "array_size": int(input("Array size: ")),
        "array_seed": int(input("Seed: ")),
        "test_count": int(input("Test count: "))
    }

    array: list[int] = generate_random_numbers(data["array_size"], data["array_seed"])

    data["time_bubble"]    = test_algorithm(bubble_sort,    array, data["test_count"])
    data["time_selection"] = test_algorithm(selection_sort, array, data["test_count"])
    data["time_heap"]      = test_algorithm(heap_sort,      array, data["test_count"])

    with open(f"./data/python_{data["array_size"]}_{data["array_seed"]}_{data["test_count"]}.json", "w") as file:
        json.dump(data, file)
