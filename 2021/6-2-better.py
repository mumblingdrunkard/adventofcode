fish = [int(s) for s in input().split(',')]

count = [0] * 9

for f in fish:
    count[f] += 1

for _ in range(256):
    count = count[1:] + count[:1]
    count[6] += count[8]

total = sum(count)

print(total)
