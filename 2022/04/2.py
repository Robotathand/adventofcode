count = 0

for line in open("data.txt"):
    pair = line.split(",")
    #print(pair)

    a, b = map(int, pair[0].split("-"))
    x, y = map(int, pair[1].split("-"))

    if a >= x and a <= y or b >= x and b <= y or x >= a and x <= b or y >= a and y <= b:
        count += 1

#    print(line, a, b, x, y, count)


print(count)
