# need to get more clever here

template = input()

input() # blank line

rules = {}

while True:
    try:
        line = input().split(" -> ")
        rules[line[0]] = line[1]
    except EOFError:
        break

# count pairs in template
# figure out that one pair becomes two pairs in the next iteration
# CH → B ⇒ CB, BH

counts = {}


last = ""

for i in range(len(template)-1):
    pair = f"{template[i]}{template[i+1]}"
    if pair in counts:
        counts[pair] += 1
    else:
        counts[pair] = 1
    last = pair

for _ in range(40):
    new_counts = {}
    for c in counts:
        r = rules[c]
        p1 = f"{c[0]}{r}" # left pair
        p2 = f"{r}{c[1]}" # right pair

        if p1 in new_counts:
            new_counts[p1] += counts[c]
        else:
            new_counts[p1] = counts[c]

        if p2 in new_counts:
            new_counts[p2] += counts[c]
        else:
            new_counts[p2] = counts[c]

    counts = new_counts

letter_counts = {}

# counts the first letter in each pair * occurences of that pair
for c in counts:
    a = c[0]
    if a in letter_counts:
        letter_counts[a] += counts[c]
    else:
        letter_counts[a] = counts[c]

letter_counts[last[1]] += 1 # count last letter an extra time

mn = min(letter_counts, key=letter_counts.get)
mx = max(letter_counts, key=letter_counts.get)

print(letter_counts[mn])
print(letter_counts[mx])
print(letter_counts[mx] - letter_counts[mn])
