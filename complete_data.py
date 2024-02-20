import json, os

def complete_data(data: dict, algo: str):
    data[f"avg_{algo}"] = sum(data[f"time_{algo}"]) / len(data[f"time_{algo}"])
    data[f"min_{algo}"] = min(data[f"time_{algo}"])
    data[f"max_{algo}"] = max(data[f"time_{algo}"])


for filename in os.listdir("./data/"):
    path = f"./data/{filename}"
    if not os.path.isfile(path) or filename == "language_size_seed_tests.json":
        continue
    
    with open(path, "r") as file:
        data = json.load(file)
    
    complete_data(data, "bubble")
    complete_data(data, "selection")
    complete_data(data, "heap")

    with open(path, "w") as file:
        json.dump(data, file, indent=4)
