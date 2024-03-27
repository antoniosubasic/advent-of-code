def part1(input):
    return sum(int(i) for i in input)

def part2(input):
    frequencies = set()
    current = 0
    i = 0
    
    while True:
        current += int(input[i % len(input)])
        if current in frequencies: return current
        else: frequencies.add(current)
        i += 1

input = open('../input.txt').read().splitlines()

print(part1(input))
print(part2(input))
