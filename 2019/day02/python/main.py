def part1(input):
    input[1] = 12
    input[2] = 2

    for i in range(0, len(input), 4):
        if input[i] == 99: break

        operand1 = input[input[i + 1]]
        operand2 = input[input[i + 2]]

        if input[i] == 1: input[input[i + 3]] = operand1 + operand2
        elif input[i] == 2: input[input[i + 3]] = operand1 * operand2

        print(input)

    return input[0]

def part2(input):
    for noun in range(100):
        for verb in range(100):
            memory = input.copy()
            memory[1] = noun
            memory[2] = verb

            for i in range(0, len(memory), 4):
                if memory[i] == 99: break

                operand1 = memory[memory[i + 1]]
                operand2 = memory[memory[i + 2]]

                if memory[i] == 1: memory[memory[i + 3]] = operand1 + operand2
                elif memory[i] == 2: memory[memory[i + 3]] = operand1 * operand2

            if memory[0] == 19690720: return 100 * noun + verb

    return -1

input = [int(i) for i in open('../input.txt').read().split(',')]

print(part1(input.copy()))
print(part2(input))
