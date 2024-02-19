import random, time, math,threading
runtimes = []

lol = []

def gen_ran_list(lens = 10000, maxvalue = 10000):
    return_list = []
    for _ in range(lens):
        return_list.append(random.randint(0, maxvalue))
    return return_list


def bubble_sort(array: list[int]):
    for _ in range(len(array)-1):
        for i in range(len(array)-1):
            if array[i] >= array[i+1]:
                # Elemente tauschen
                array[i], array[i+1] = array[i+1], array[i]

    runtimes.append(time.thread_time())


def selection_sort(array: list[int]):
    for i in range(len(array)):
        jmin = i
        for j in range(i+1, len(array)):
            if array[j] < array[jmin]:
                jmin = j
        array[i], array[jmin] = array[jmin], array[i]

    runtimes.append(time.thread_time())


def heapify(array : list[int],i : int, n : int):
    max = i
    leftchild = 2 * i + 1
    rightchild = 2 * i + 2

    if leftchild < n and array[i] < array[leftchild]:
        max = leftchild
    
    if rightchild < n and array[max] < array[rightchild] :
        max = rightchild

    if max != i:
        array[i],array[max] = array[max],array[i]
        heapify(array,max,n)    

def heap_sort(array: list[int]):
    n = len(array)
    for i in range(n//2,-1,-1):
        heapify(array,i,n)
    
    for i in range(n-1,0,-1):
        array[0],array[i]=array[i],array[0]
        heapify(array,0,i)

    runtimes.append(time.thread_time())
    return array




for i in range(1000):
    lol.append(gen_ran_list())

for k in range(len(lol)):
    t = threading.Thread(target=heap_sort, args=[lol[k]])
    t.start()
    t.join()


avg_run = sum(runtimes)/len(runtimes)


print(f"the average runtime is: {avg_run} seconds")
print(f"the highest runtime is: {max(runtimes)} seconds")
print(f"the lowest runtime is: {min(runtimes)} seconds")


ask = input("do you want to see the full list of runtimes N/Y \n")
if ask == "Y" or ask == "y" or ask == "yes" or ask == "Yes":
    print(runtimes)

