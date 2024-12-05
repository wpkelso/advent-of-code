import os, sys

# Get input file name and open it
path = os.path.join("../input/", sys.argv[1])
if not os.path.exists(path):
  print("Bad path!")
  exit()

# for each line (report), calculate whether the report is safe
# and increment the total if it is
total = 0
with open(path, "r") as file:
  for line in file:
    report = line.strip("\n").split(" ")

    print(report)

    increase = False 
    decrease = False
    errors = False

    # continue checking as long as there are still levels to check,
    # and both an increase and a decrease haven't been registered
    for i in range(1, len(report)):
       # calculate the difference between the current level and it's predecessor
      difference = int(report[i]) - int(report[i - 1])
      abs_diff = abs(difference)

     # if abs_diff is outside of the range 0 < x < 4
      if not (1 <= abs_diff <= 3):
        errors = True
        print(f"Unsafe found with difference {abs_diff}")
        break

      if difference > 0:
        increase = True
      elif difference < 0:
        decrease = True

      if increase and decrease:
        errors = True
        print(f"Unsafe found with increase {increase} and decrease {decrease}")
        break

    # increment the safe total if the script got through the report without finding a violation
    total += 1 if not errors else 0

print(f"Total safe: {total}")
