rows = []
while True:
    try:
        bits = input()
        bits = list([1 if c == '1' else 0 for c in bits])
        rows.append(bits)
    except EOFError:
        break

result = rows

i = 0
while len(rows) > 1 and i < len(rows[0]):
    z, o = [], []

    for bits in rows:
        if bits[i] == 0:
            z.append(bits)
        elif bits[i] == 1:
            o.append(bits)

    if len(z) > len(o):
        rows = z
        print("z")
    else:
        rows = o
        print("o")

    i += 1

rows[0].reverse()
oxygen = sum([2**i if x == 1 else 0 for i,x in enumerate(rows[0])])


rows = result

i = 0
while len(rows) > 1 and i < len(rows[0]):
    z, o = [], []

    for bits in rows:
        if bits[i] == 0:
            z.append(bits)
        elif bits[i] == 1:
            o.append(bits)

    if len(z) <= len(o):
        rows = z
        print("z")
    else:
        rows = o
        print("o")

    i += 1

rows[0].reverse()
co2 = sum([2**i if x == 1 else 0 for i,x in enumerate(rows[0])])

print(f"{co2}, {oxygen}")
