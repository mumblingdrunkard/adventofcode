def abs(a):
    if a < 0:
        return -a
    else:
        return a

crabs = [int(s) for s in input().split(',')]

average = round(sum(crabs)//len(crabs))

costs = []
for i in range(average-1, average+2):
    cost = sum([(abs(c - i)*(abs(c-i)+1))//2 for c in crabs])
    costs.append(cost)

print(min(costs))
