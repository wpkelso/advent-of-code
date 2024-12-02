import os, sys

# Get input file name and open it
path = os.path.join("../input/", sys.argv[1])
if not os.path.exists(path):
  print("Bad path!")
  exit()

# extract each line into their respective lists 
left = []
right = []
with open(path, "r") as file:
  for line in file:
    first, second = line.split("   ")
    left.append(int(first))
    right.append(int(second.strip("\n")))

# sort the resulting lists in ascending order
left.sort()
right.sort()

# calculate the distance between each pair 
distance = []
total = 0
for index, value in enumerate(left):
  difference = abs(value - right[index])
  distance.append(difference)

# sum the distances and print the total
for value in distance:
  total += value

print(f"The total distance is: {total}")
