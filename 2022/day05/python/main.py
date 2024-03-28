import re, copy

def part1and2(crates, instructions, part):
    for instruction in instructions:
        cratesToMove = crates[instruction[1] - 1][-instruction[0]:]
        crates[instruction[2] - 1].extend(cratesToMove[::-1] if part == 1 else cratesToMove)
        crates[instruction[1] - 1] = crates[instruction[1] - 1][:-instruction[0]]

    return ''.join([stack[-1] for stack in crates])


input = [entry.split('\n') for entry in open('../input.txt').read().split('\n\n')]
instructions = [[int(num) for num in re.findall(r'\d+', instruction)] for instruction in input[1] if instruction != '']
crates = [[] for _ in range([int(num) for num in re.findall(r'\d+', input[0][-1])][-1])]

for i in range(len(input[0]) - 2, -1, -1):
    for j in range(1, len(input[0][i]), 4):
        if (crate := input[0][i][j]) != ' ':
            crates[int(input[0][-1][j]) - 1].append(crate)


print(part1and2(copy.deepcopy(crates), instructions, 1))
print(part1and2(crates, instructions, 2))
