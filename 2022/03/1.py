lenth = 0
total = 0

f = open("data.txt", "r")
for x in f:
    rucksack = x
    lenth = len(rucksack)
    c1 = rucksack[0:int(lenth / 2)]
    c2 = rucksack[int(lenth / 2 ):lenth]
    cc = ''.join(
        set(c1).intersection(c2)
    )
    if cc.isupper() == True:
        priority = ord(cc) - 38
    else:
        priority = ord(cc) - 96
    total = total + priority
#    print(cc, priority)

print(total)
