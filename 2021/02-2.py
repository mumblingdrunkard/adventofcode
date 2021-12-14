h, d, a = 0, 0, 0

while True:
    args = input().split(' ')
    cmd = args[0]

    if cmd == "forward":
        n = int(args[1])
        h += n
        d += n * a
    elif cmd == "down":
        n = int(args[1])
        a += n
    elif cmd == "up":
        n = int(args[1])
        a -= n
    elif cmd == "exit":
        break

print(f"{h}, {d}, {a}")
print(h*d)
