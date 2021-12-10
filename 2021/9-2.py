height_map = []
marked_map = []

while True:
    try:
        line = [int(c) for c in input()]
        marked = [0 for c in line]

        height_map.append(line)
        marked_map.append(marked)
    except EOFError:
        break

total = 0

low_points = []
basin_sizes = []

def fill_from(height_map, marking, y, x, n):
    if height_map[y][x] == 9:
        return

    marking[y][x] = n

    if y > 0:
        if marked_map[y-1][x] == 0:
            fill_from(height_map, marking, y-1, x, n)

    if y < len(height_map)-1:
        if marked_map[y+1][x] == 0:
            fill_from(height_map, marking, y+1, x, n)

    if x > 0:
        if marked_map[y][x-1] == 0:
            fill_from(height_map, marking, y, x-1, n)

    if x < len(height_map[y])-1:
        if marked_map[y][x+1] == 0:
            fill_from(height_map, marking, y, x+1, n)


for y in range(len(height_map)):
    for x in range(len(height_map[y])):
        #print(f"Checking {y}, {x}")
        if y > 0:
            if height_map[y-1][x] <= height_map[y][x]:
                continue

        if y < len(height_map)-1:
            if height_map[y+1][x] <= height_map[y][x]:
                continue

        if x > 0:
            if height_map[y][x-1] <= height_map[y][x]:
                continue

        if x < len(height_map[y])-1:
            if height_map[y][x+1] <= height_map[y][x]:
                continue

        total += 1 + height_map[y][x]
        low_points.append((y, x))

i = 1
for y in range(len(marked_map)):
    for x in range(len(marked_map[y])):
        if marked_map[y][x] == 0 and height_map[y][x] != 9:
            fill_from(height_map, marked_map, y, x, i)
            i += 1

for y in range(len(marked_map)):
    print(marked_map[y])

sums = {}
for y in range(len(marked_map)):
    for x in range(len(marked_map[y])):
        v = marked_map[y][x]
        if v in sums:
            sums[v] += 1
        else:
            sums[v] = 1

res = {k: v for k, v in sorted(sums.items(), key=lambda item: item[1])}

print(res)

print(total)
