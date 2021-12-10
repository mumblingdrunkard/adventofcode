def parse(s):
    pairs = {'[': ']', '{': '}', '(': ')', '<': '>'}

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
            return 0

        openings.pop()
        closings.pop()

    # corrupted lines should now be discarded
    value = {')': 1, ']': 2, '}': 3, '>': 4}

    total = 0

    while len(closings) > 0:
        total = total * 5 + value[closings.pop()]

    return total

total = 0

scores = []

while True:
    try:
        s = input()
        score = parse(s)
        if score != 0:
            scores.append(score)
    except EOFError:
        break

scores.sort()

print(scores[len(scores)//2])
