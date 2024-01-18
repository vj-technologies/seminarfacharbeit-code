import random,time,threading
runtimes = []

lol =[]

def gen_ran_list(lens = 1000, maxvalue = 10000):
    return_list = []
    for i in range(lens):
        return_list.append(random.randint(0,maxvalue))
    return return_list

def buubble_sort(In:list[int]):
    for _ in range(len(In)-1):
        for i in range (len(In)-1):
            if In[i] >= In[i+1]:
                In[i],In[i+1] = In[i+1] , In[i]

    
    runtimes.append(time.thread_time())
  

for i in range(100):
    lol.append(gen_ran_list())

for k in range(len(lol)):
    t = threading.Thread(target=buubble_sort, args=[lol[k]])
    t.start()
    t.join()



avg_run = sum(runtimes)/len(runtimes)



print(f"the average runtime is: {avg_run} seconds")
print(f"the highest runtime is: {max(runtimes)} seconds")
print(f"the lowest runtime is: {min(runtimes)} seconds")


ask = input("do you want to see the full list of runtimes N/Y \n")
if ask == "Y" or ask == "y" or ask == "yes" or ask == "Yes":
    print(runtimes)