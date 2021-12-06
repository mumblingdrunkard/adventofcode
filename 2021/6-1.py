fish = [int(s) for s in input().split(',')]

count = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}

for f in fish:
    count[f] += 1

print(count)

for _ in range(80):
    new_count = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}

    for k in count.keys():
        if k == 0:
            new_count[6] += count[k]
            new_count[8] += count[k]
        else:
            new_count[k-1] += count[k]

    count = new_count

total = sum(count[k] for k in count.keys())

print(total)
