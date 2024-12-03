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

# iterate over the right list, creating a dictionary where each key:value corresponds with number:frequency
frequencyTable = {} 
for num in right:
  if num not in frequencyTable:
    frequencyTable[num] = 0

  frequencyTable[num] += 1

# iterate over left list, multiplying the value by 
# the value in frequencyTable at the corresponding key
total = 0
for value in left:
  # look for the key in the frequency table
  # if it doesn't exist, record a zero
  freq = frequencyTable.get(value, 0)
  score = value * freq
  total += score

print(f"Similarity Score: {total}")
