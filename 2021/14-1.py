template = input()

input() # blank line

rules = {}

while True:
    try:
        line = input().split(" -> ")
        rules[line[0]] = line[1]
    except EOFError:
        break

for _ in range(10):
    result = ""
    for i in range(len(template) - 1):
        result += template[i]
        pair = f"{template[i]}{template[i + 1]}"
        result += rules[pair]
    result += template[-1]
    template = result

counts = {}

for c in template:
    if c in counts:
        counts[c] += 1
    else:
        counts[c] = 1



mn = min(counts, key=counts.get)
mx = max(counts, key=counts.get)

print(counts[mn])
print(counts[mx])
