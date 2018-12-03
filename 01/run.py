from time import perf_counter

def run(input):
  return sum([int(line) for line in input])

start = perf_counter()
result = run(open("input.txt"))
end = perf_counter()
print(result)
print(f"--- done in {(end - start) * 1000}ms ---")
