points = 0
count = 0

f = open("data.txt", "r")
for x in f:
   if x == "A X\n": 
      points = points + 3
   elif x == "A Y\n":
      points = points + 4
   elif x == "A Z\n":
      points = points + 8
   elif x == "B X\n":
      points = points + 1
   elif x == "B Y\n":
      points = points + 5
   elif x == "B Z\n":
      points = points + 9
   elif x == "C X\n": 
      points = points + 2
   elif x == "C Y\n":
      points = points + 6
   elif x == "C Z\n":
      points = points + 7
   else:
       print("ERROR")
#   print(x, points)
   count = count + 1

#print(count)
print(points)
