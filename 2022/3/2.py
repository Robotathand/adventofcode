three = 1
total = 0

f = open("data.txt", "r")
for x in f:
    if three == 1:
        r1 = x.strip()
    elif three == 2:
        r2 = x.strip()
    elif three == 3:
        r3 = x.strip()
        strings = [r1, r2, r3]
        cc = set.intersection(*map(set,strings))
#        print(cc, str(cc).strip())
        if str(cc).isupper() == True:
            priority = ord(str(cc)[2]) - 38
        else:
            priority = ord(str(cc)[2]) - 96
        total = total + priority
        three = 0
#        print(priority)
    else:
        print("error: ", x)
    three = three + 1

print(total)
