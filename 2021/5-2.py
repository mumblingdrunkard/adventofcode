def abs(a):
    if a < 0:
        a = -a
    return a

grid = [[]] * 1000
for r in range(1000):
    grid[r] = [0] * 1000

while True:
    try:
        line = input().split(" -> ")

        p0 = [int(s) for s in line[0].split(',')]
        p1 = [int(s) for s in line[1].split(',')]

        x0 = p0[0]
        y0 = p0[1]

        x1 = p1[0]
        y1 = p1[1]

        x_slope = 1 if x1-x0 > 0 else -1
        y_slope = 1 if y1-y0 > 0 else -1

        if x0 == x1:
            for y in range(abs(y1 - y0) + 1):
                grid[y0 + y * y_slope][x0] += 1

        if y0 == y1:
            for x in range(abs(x1 - x0) + 1):
                grid[y0][x0 + x * x_slope] += 1

        if abs(x1 - x0) == abs(y1 - y0):
            for i in range(abs(x1 - x0) + 1):
                x = x0 + i * x_slope
                y = y0 + i * y_slope
                # print(f"{x}, {y}")
                grid[y][x] += 1

    except EOFError:
        break

total = 0
for row in grid:
    for col in row:
        if col > 1:
            total += 1

print(total)
