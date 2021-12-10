res = 0
while True:
    try:
        line = input().split(" | ")
        output = line[1].split()
        total = sum([1 if len(s) == 2 or len(s) == 3 or len(s) == 4 or len(s) == 7 else 0 for s in output])
        res += total
    except EOFError:
        break

print(res)
