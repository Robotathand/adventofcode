count = 0

for line in open("data.txt"):
    pair = line.split(",")
    #print(pair)

    a, b = (pair[0]).split("-")
    x, y = (pair[1]).split("-")
    
#    print ("line", x.strip())
#    print ("pair_one_start", pair_one_start)
#    print ("pair_one_end", pair_one_end)
#    print ("pair_two_start", pair_two_start)
    #print ("pair_two_end", pair_two_end)

    if (a <= x and b >= y) or (x <= a and y >= b):
        count += 1
        #print("count", count)

print(count)
