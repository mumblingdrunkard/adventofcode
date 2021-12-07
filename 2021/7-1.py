import math

def abs(a):
    if a < 0:
        return -a
    else:
        return a

crabs = [int(s) for s in input().split(',')]
crabs.sort()

center = crabs[len(crabs)//2]

cost = sum([abs(c-center) for c in crabs])

print(cost)
