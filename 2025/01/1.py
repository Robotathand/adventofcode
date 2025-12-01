f = open("./data.txt", "r")

d = 50
t = 0

for l in f:
    if l[0] == "L":
        d -= int(l[1:])
    elif l[0] == "R":
        d += int(l[1:])
    
    while d < 0:
        d += 100
    while d > 99:
        d -= 100
    
    if d == 0:
        t += 1
    # print(f"{l} {d}")

print(t)