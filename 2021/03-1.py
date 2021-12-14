bits = []
while True:
    try:
        string = input()
        if len(bits) < len(string):
            bits = [0] * len(string)
        for i, c in enumerate(string):
            bits[i] += 1 if c == '1' else -1
    except EOFError:
        break

bits.reverse()
gamma = sum([2**i if x > 0 else 0 for i,x in enumerate(bits)])
epsilon = sum([2**i if x <= 0 else 0 for i,x in enumerate(bits)])

print(gamma)
print(epsilon)

print(gamma*epsilon)
