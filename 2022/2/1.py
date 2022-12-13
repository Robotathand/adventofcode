points = 0
count = 0

f = open("data.txt", "r")
for x in f:
   if x == "A X\n": #draw(3) + rock(1) 
      points = points + 4
   elif x == "A Y\n":  #win(6) + paper(2)
      points = points + 8
   elif x == "A Z\n": #loss(0) + scissors(3) 
      points = points + 3
   elif x == "B X\n": #loss(0) + rock(1) 
      points = points + 1
   elif x == "B Y\n": #draw(3) + paper(2) 
      points = points + 5
   elif x == "B Z\n": #win(6) + scissors(3) 
      points = points + 9
   elif x == "C X\n": #win(6) + rock(1) 
      points = points + 7
   elif x == "C Y\n": #loss(0) + paper(2)
      points = points + 2
   elif x == "C Z\n": #draw(3) + scissors(3)
      points = points + 6
   else:
       print("ERROR")
#   print(x, points)
   count = count + 1

#print(count)
print(points)
