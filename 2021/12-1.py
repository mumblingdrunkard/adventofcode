def is_small(s):
    return s == s.lower()

def explore(caves, f, visited) -> int:
    res = 0

    if f == "end":
        return 1

    new_visited = dict(visited)

    if is_small(f):
        new_visited[f] = True

    for c in caves[f]:
        if c not in visited:
            res += explore(caves, c, new_visited)

    return res

caves = {}

while True:
    try:
        line = input().split('-')
        if line[0] not in caves:
            caves[line[0]] = []

        if line[1] not in caves:
            caves[line[1]] = []

        caves[line[0]].append(line[1])
        caves[line[1]].append(line[0])
    except EOFError:
        break

print(explore(caves, "start", {}))
