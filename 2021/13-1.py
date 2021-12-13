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

for dot in paper:
    print(dot)

instruction = input().split()
instruction = instruction[2].split('=')

fold_along = instruction[0]
value = int(instruction[1])

if fold_along == "x":
    paper = fold_x(paper, value)

if fold_along == "y":
    paper = fold_y(paper, value)

print()
print("After fold:")
for dot in paper:
    print(dot)

print(count_dots(paper))
