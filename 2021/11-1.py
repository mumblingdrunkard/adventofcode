def count_flashes(octopi: list[list[int]]) -> bool:
    res = 0
    for y in range(10):
        for x in range(10):
            if octopi[y][x] == 0:
                res += 1
    return res

def has_flashes(octopi: list[list[int]]) -> bool:
    for y in range(10):
        for x in range(10):
            if octopi[y][x] > 9:
                return True
    return False


def next(octopi: list[list[int]]) -> list[list[int]]:
    flashed = []

    state = octopi
    for y in range(10):
        for x in range(10):
            state[y][x] += 1

    while has_flashes(octopi):
        for f in flashed:
            (y, x) = f
            state[y][x] = 0

        for y in range(10):
            for x in range(10):
                if state[y][x] > 9 and (y, x) not in flashed:
                    flashed.append((y, x))
                    if y > 0:
                        state[y-1][x] += 1
                    if y < 9:
                        state[y+1][x] += 1
                    if x > 0:
                        state[y][x-1] += 1
                    if x < 9:
                        state[y][x+1] += 1
                    if y > 0 and x > 0:
                        state[y-1][x-1] += 1
                    if y < 9 and x > 0:
                        state[y+1][x-1] += 1
                    if y > 0 and x < 9:
                        state[y-1][x+1] += 1
                    if y < 9 and x < 9:
                        state[y+1][x+1] += 1

    for f in flashed:
        (y, x) = f
        state[y][x] = 0

    return state


octopi = []

for _ in range(10):
    row = [int(c) for c in input()]
    octopi.append(row)

total = 0

for i in range(1000000):
    n = next(octopi)
    if count_flashes(n) == 100:
        print(i)
        break
    octopi = n

