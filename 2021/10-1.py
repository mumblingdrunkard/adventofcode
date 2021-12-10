def parse(s):
    pairs = {'[': ']', '{': '}', '(': ')', '<': '>'}
    value = {')': 3, ']': 57, '}': 1197, '>': 25137}

    openings = []
    closings = []
    for c in s:
        if c in pairs:
            openings.append(c)
            closings.append(pairs[c])
            continue

        if len(closings) == 0:
            return 0

        if c != closings[-1]:
            return value[c]

        openings.pop()
        closings.pop()

    return 0

total = 0

while True:
    try:
        s = input()
        total += parse(s)
    except EOFError:
        break

print(total)
