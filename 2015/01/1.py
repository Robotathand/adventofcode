floor = 0

f = open("./data.txt", "r", encoding="utf-8")

for l in f:
    for c in l:
        if c == "(":
            floor += 1
        elif c == ")":
            floor -= 1

f.close()

print(floor)