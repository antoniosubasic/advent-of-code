﻿int Part1(string[] input)
{
    var sum = 0;

    foreach (var str in input)
    {
        var number = "";

        number += str.First(char.IsDigit);
        number += str.Last(char.IsDigit);

        sum += int.Parse(number);
    }

    return sum;
}

int Part2(string[] input)
{
    string[] numberStrings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    var sum = 0;

    foreach (var str in input)
    {
        var numbers = new List<int>();

        for (var i = 0; i < str.Length; i++)
        {
            if (char.IsDigit(str[i])) { numbers.Add(str[i] - '0'); }
            else
            {
                var buffer = "";

                for (var j = 0; j < str[i..].Length; j++)
                {
                    buffer += str[i + j];

                    if (numberStrings.Contains(buffer))
                    {
                        numbers.Add(Array.IndexOf(numberStrings, buffer) + 1);
                        i += j - 1;
                        break;
                    }
                }
            }
        }

        sum += numbers.First() * 10 + numbers.Last();
    }

    return sum;
}

var input = File.ReadAllLines("../input.txt");

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));
