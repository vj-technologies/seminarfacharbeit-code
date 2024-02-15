import matplotlib.pyplot as plt
import json

GROUPING_SIZE = 500_000


with open("bubble_sort.json", "r") as file:
    bubble = json.load(file)

with open("selection_sort.json", "r") as file:
    selection = json.load(file)


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


plt.title("Zeit um 1000 Elemente zu sortieren (1000 mal pro Algorithmus)")
bar_time_distribution(bubble["1k"],       "Bubble Sort")
bar_time_distribution(selection["1k"], "Selection Sort")
plt.ylabel("CPU Zyklen")



plt.legend()
plt.show()
