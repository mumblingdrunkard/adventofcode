count = 0
prev = int(input())
curr = 0
while True:
    try:
        curr = int(input())
    except EOFError:
        break;

    if curr > prev:
        count += 1

    prev = curr

print(count)

