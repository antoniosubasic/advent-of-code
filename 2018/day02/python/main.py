def part1(input):
    two = three = 0

    for line in input:
        counts = {}
        for char in line: counts[char] = counts.get(char, 0) + 1
        if 2 in counts.values(): two += 1
        if 3 in counts.values(): three += 1

    return two * three

def part2(input):
    for i in range(len(input)):
        for j in range(i + 1, len(input)):
            diff = [a != b for a, b in zip(input[i], input[j])]
            if sum(diff) == 1:
                return ''.join([input[i][k] for k in range(len(input[i])) if not diff[k]])

input = open('../input.txt').read().splitlines()

print(part1(input))
print(part2(input))
