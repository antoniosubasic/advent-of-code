def part1(input):
    sum = 0

    for items in input:
        compartment1 = items[:len(items) // 2]
        compartment2 = items[len(items) // 2:]
        item = [items for items in compartment1 if items in compartment2][0]
        sum += (ord(item) - ord('a') + 1) if item.islower() else (ord(item) - ord('A') + 27)

    return sum

def part2(input):
    sum = 0

    for i in range(0, len(input), 3):
        item = [items for items in input[i] if items in input[i + 1] and items in input[i + 2]][0]
        sum += (ord(item) - ord('a') + 1) if item.islower() else (ord(item) - ord('A') + 27)

    return sum

input = open('../input.txt').read().splitlines()

print(part1(input))
print(part2(input))
