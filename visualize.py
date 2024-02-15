import matplotlib.pyplot as plt
import json

GROUPING_SIZE = 500_000


# BUBBLE SORT
with open("bubble_sort.json", "r") as file:
    bubble = json.load(file)

bubble_1k_x = []
bubble_1k_y = []
for b in bubble["1k"]:
    b = round(b/GROUPING_SIZE)
    try:
        index = bubble_1k_x.index(b)
    except ValueError:
        bubble_1k_x.append(b)
        bubble_1k_y.append(1)
    else:
        bubble_1k_y[index] += 1

plt.bar(bubble_1k_x, bubble_1k_y, label="Bubble Sort")


# SELECTION SORT
with open("selection_sort.json", "r") as file:
    selection = json.load(file)

selection_1k_x = []
selection_1k_y = []
for b in selection["1k"]:
    b = round(b/GROUPING_SIZE)
    try:
        index = selection_1k_x.index(b)
    except ValueError:
        selection_1k_x.append(b)
        selection_1k_y.append(1)
    else:
        selection_1k_y[index] += 1

plt.bar(selection_1k_x, selection_1k_y, label="Selection Sort")

plt.ylabel("CPU Zyklen")
plt.title("Zeit um 1000 Elemente zu sortieren.")
plt.legend()
plt.show()
