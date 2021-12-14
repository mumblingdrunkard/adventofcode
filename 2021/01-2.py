WINDOW_SIZE = 3

window = [0, 0, 0]
index = 0
sum = 0

for i in range(WINDOW_SIZE):
    window[i] = int(input())
    sum += window[i]

count = 0
prev = sum
curr = 0
while True:
    try:
        n = int(input())
        sum -= window[index]
        window[index] = n
        sum += window[index]
        index += 1
        index %= WINDOW_SIZE
        curr = sum
    except EOFError:
        break;

    if curr > prev:
        count += 1

    prev = curr

print(count)

