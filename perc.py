
import random

N = 1000000

sum = 0
equal = [0]*19
more = [0]*19
print("This script will calculate the percentage chances for 4d6 take best 3:")
for i in range(0,N):
    a = random.randint(1,6)
    b = random.randint(1,6)
    c = random.randint(1,6)
    d = random.randint(1,6)
    minimum = min(a,b,c,d)
    partial_sum = (a+b+c+d)-minimum
    for n in range(1,19):
        if(partial_sum==n):
            equal[n] += 1
        if(partial_sum>=n):
            more[n] += 1
    sum += partial_sum
print("This is the average value when you roll 4d6 and take the sum of the best 3:", round(sum/N))
for i in range(1,19):
    print("Chance that its exactly %d: %0.2f%% | Chance that its bigger or equal than %d: %0.2f%%" % (i, equal[i]/N*100, i, more[i]/N*100))

    