with open("./input.txt") as f:
  sum = 0
  for line in f:
    sum += int(line)
  print(sum)