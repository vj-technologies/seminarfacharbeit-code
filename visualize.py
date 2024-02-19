import matplotlib.pyplot as plt
import json

GROUPING_SIZE = 500_000

filename = input("Filename: ")
with open(f"data/{filename}.json", "r") as file:
    data = json.load(file)


def bar_time_distribution(data, label):
    x = []
    y = []
    for cycles in data:
        cycles = round(cycles / GROUPING_SIZE)
        try:
            index = x.index(cycles)
        except ValueError:
            x.append(cycles)
            y.append(1)
        else:
            y[index] += 1

    plt.bar(x, y, label=label)

def line_complexity_over_input_size(data, label):
    pass # TODO


plt.title(f"Zeit in ns um {data["array_size"]} Elemente zu sortieren ({data["test_count"]}/Algorithmus) Seed: {data["array_seed"]}")
bar_time_distribution(data["time_bubble"], "Bubble Sort")
bar_time_distribution(data["time_selection"], "Selection Sort")
bar_time_distribution(data["time_heap"], "Heap Sort")
plt.ylabel("Zeit in ns")



plt.legend()
plt.show()
