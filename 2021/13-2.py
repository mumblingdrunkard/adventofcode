def fold_x(paper, value):
    result = {}
    for dot in paper:
        (x, y) = dot
        if x > value:
            offset = x - value
            new_x = value - offset
            result[(new_x, y)] = 1
        else:
            result[(x, y)] = 1
    return result

def fold_y(paper, value):
    result = {}
    for dot in paper:
        (x, y) = dot
        if y > value:
            offset = y - value
            new_y = value - offset
            result[(x, new_y)] = 1
        else:
            result[(x, y)] = 1
    return result

def count_dots(paper: list[str]) -> int:
    result = 0
    for dot in paper:
        result += 1
    return result

paper = {}

while True:
    line = input()
    if len(line) == 0:
        break
    x = int(line.split(',')[0])
    y = int(line.split(',')[1])
    paper[(x, y)] = 1

while True:
    try:
        instruction = input().split()
        instruction = instruction[2].split('=')

        fold_along = instruction[0]
        value = int(instruction[1])

        if fold_along == "x":
            paper = fold_x(paper, value)

        if fold_along == "y":
            paper = fold_y(paper, value)
    except EOFError:
        break

final = []
for i in range(6):
    final.append([' '] * 81)

for dot in paper:
    (x, y) = dot
    final[y][x] = '#'

for row in final:
    print(''.join(row))

print(count_dots(paper))
