class Entry:
    def __init__(self, entry):
        (items, self.password) = entry.split(': ')
        (range, self.letter) = items.split(' ')
        (self.val1, self.val2) = map(int, range.split('-'))

def part1(input):
    valid = 0
    for entry in input:
        count = entry.password.count(entry.letter)
        if count >= entry.val1 and count <= entry.val2:
            valid += 1
    return valid

def part2(input):
    valid = 0
    for entry in input:
        if (entry.password[entry.val1 - 1] == entry.letter) ^ (entry.password[entry.val2 - 1] == entry.letter):
            valid += 1
    return valid

input = [Entry(entry) for entry in open('../input.txt').read().splitlines()]

print(part1(input))
print(part2(input))
