lenth = 0

f = open("data.txt", "r")
for x in f:
    rucksack = x
    lenth = len(rucksack)
    c1 = rucksack[0:int(lenth / 2)]
    c2 = rucksack[int(lenth / 2 ):lenth]

print(rucksack, c1, c2)
