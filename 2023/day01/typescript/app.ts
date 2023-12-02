import * as fs from 'fs';

function part1(input: string[]) {
    let sum = 0;

    for (const line of input) {
        let leftMost = 0;
        let rightMost = 0;

        for (let i = 0; i < line.length; i++) {
            let first = parseInt(line[i]);
            let second = parseInt(line[line.length - 1 - i]);

            if (first && leftMost == 0) {
                leftMost = first;
            }

            if (second && rightMost == 0) {
                rightMost = second;
            }

            if (leftMost != 0 && rightMost != 0) {
                break;
            }
        }

        sum += leftMost * 10 + rightMost;
    }

    return sum;
}

function part2(input: string[]) {
    const numberStrings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let sum = 0;

    for (const line of input) {
        let numbers: number[] = [];

        for (let i = 0; i < line.length; i++) {
            var number = parseInt(line[i]);

            if (number) {
                numbers.push(number);
            } else {
                let buffer = "";

                for (let j = 0; j < line.substring(i).length; j++) {
                    buffer += line[i + j];

                    if (numberStrings.includes(buffer)) {
                        numbers.push(numberStrings.indexOf(buffer) + 1);
                        i += j - 1;
                        break;
                    }
                }
            }
        }

        sum += numbers[0] * 10 + numbers[numbers.length - 1];
    }

    return sum;
}


const input = fs.readFileSync("../input.txt").toString().split('\n');
console.log(part1(input));
console.log(part2(input));
