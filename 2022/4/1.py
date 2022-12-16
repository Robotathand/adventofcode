count = 0

f = open("data.txt", "r")
for x in f:
    pair = x.strip().split(",")
#    print(pair)
    a = pair[0].split("-")
    b = pair[1].split("-")
    aa = a[0]
    ab = a[1]
    ba = b[0]
    bb = b[1]
    if aa <= ba and ab >= bb or ba <= aa and bb >= ab:
#    if (aa <= ba and ab >= bb or ba <= aa and bb >= ab) and ((aa != ba)and(ab != bb)):
        count = count + 1
    print(x, pair, a, b, aa, ab, ba, bb, count)

print(count)
