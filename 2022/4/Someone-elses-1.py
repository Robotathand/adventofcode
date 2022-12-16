t = 0

for line in open("data.txt"):
    a, b, x, y = map(int, line.replace(",", "-").split("-"))
    if a <= x and b >= y or x <= a and y >= b:
        t += 1

print(t)
