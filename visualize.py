import matplotlib.pyplot as plt
import json

GROUPING_SIZE = 600_000

def load_data(filename: str):
    with open(f"data/{filename}.json", "r") as file:
        data = json.load(file)
    return data

def bar_time_distribution(data, label, color=None):
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

    plt.bar(x, y, label=label, color=color)

# cpp = load_data(   "pc_1_cpp_1000_0_1000")
# rs  = load_data(  "pc_1_rust_1000_0_1000")
# py  = load_data("pc_1_python_1000_0_1000")

# bar_time_distribution(cpp["time_bubble"], "C++")
# bar_time_distribution( rs["time_bubble"], "Rust")
# bar_time_distribution( py["time_bubble"], "CPython")

# plt.title(f"Bubble Sort mit {rs["array_size"]} Elementen und {rs["test_count"]} Tests")
plt.ylabel("Häufigkeit")
plt.xlabel(f"Zeit in 600 ms")

# pc1 = load_data("pc_1_rust_1000_0_1000")
# plt.title(f"Rust, {pc1["array_size"]} Elemente, 1000 Tests, AMD Ryzen 5 2600")
# bar_time_distribution(pc1["time_bubble"], "Bubble Sort")#, "firebrick")
# bar_time_distribution(pc1["time_heap"], "Heap Sort")#, "darkorange")
# bar_time_distribution(pc1["time_selection"], "Selection Sort")#, "gold")

pc2 = load_data("pc_2_python_1000_0_1000")
plt.title(f"CPython, {pc2["array_size"]} Elemente, 1000 Tests, CPU: Intel Core i7 7700K")
bar_time_distribution(pc2["time_bubble"], "Bubble Sort")#, "dodgerblue")
bar_time_distribution(pc2["time_heap"], "Heap Sort")#, "turquoise")
bar_time_distribution(pc2["time_selection"], "Selection Sort")#, "mediumpurple")


plt.legend()
plt.show()
