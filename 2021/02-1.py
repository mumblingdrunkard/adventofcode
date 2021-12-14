h, d = 0, 0

while True:
    args = input().split(' ')
    cmd = args[0]

    if cmd == "forward":
        n = int(args[1])
        h += n
    elif cmd == "down":
        n = int(args[1])
        d += n
    elif cmd == "up":
        n = int(args[1])
        d -= n
    elif cmd == "exit":
        break

print(f"{h}, {d}")
print(h*d)
