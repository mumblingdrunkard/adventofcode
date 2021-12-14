def intersect(a: str, b: str) -> str:
    res = ""
    for c in a:
        if c in b and not c in res:
            res += c
    return res

def subtract(a: str, b: str) -> str:
    res = ""
    for c in a:
        if c not in b and c not in res:
            res += c
    return res

def extract(segments: list[str], l: int) -> list[str]:
    res = []
    for e in segments:
        if len(e) == l:
            res.append(e)
    return res

def decrypt(segments: list[str]) -> list[str]:
    res = []

    one = extract(segments, 2)[0]
    four = extract(segments, 4)[0]
    seven = extract(segments, 3)[0]
    eight = extract(segments, 7)[0]

    len_six = extract(segments, 6)
    len_five = extract(segments, 5)

    three = ""
    for seg in len_five:
        if len(intersect(seg, one)) == 2:
            three = seg

    six = ""
    for seg in len_six:
        if len(intersect(seg, one)) == 1:
            six = seg

    nine = ""
    for seg in len_six:
        if len(intersect(seg, three)) == 5:
            nine = seg

    five = intersect(six, nine)

    two = ""
    for seg in len_five:
        if len(intersect(seg, five)) == 3:
            two = seg

    zero = ""
    for seg in len_six:
        if len(intersect(seg, five)) == 4:
            zero = seg

    res.append(zero)
    res.append(one)
    res.append(two)
    res.append(three)
    res.append(four)
    res.append(five)
    res.append(six)
    res.append(seven)
    res.append(eight)
    res.append(nine)

    return res

def decode(seg: list[str], decrypted: list[str]) -> int:
    res = 0
    for s in seg:
        for i in range(10):
            if len(s) != len(decrypted[i]):
                continue

            if len(subtract(s, decrypted[i])) != 0:
                continue

            if len(subtract(decrypted[i], s)) != 0:
                continue

            res += i;
            res *= 10;
            break
    return res//10

res = 0

while True:
    try:
        line = input().split(" | ")
        signals = line[0].split()

        decrypted = decrypt(signals)

        output = line[1].split()

        res += decode(output, decrypt)
    except EOFError:
        break

print(res)
