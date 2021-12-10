height_map = []

while True:
    try:
        line = [int(c) for c in input()]
        height_map.append(line)
    except EOFError:
        break

print(height_map)

total = 0

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

        print(f"low point: {height_map[y][x]}")
        total += 1 + height_map[y][x]

print(total)
