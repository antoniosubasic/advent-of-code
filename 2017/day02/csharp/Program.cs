int Part1(HashSet<int>[] input) => input.Sum(numbers => numbers.Max() - numbers.Min());

int Part2(HashSet<int>[] input)
{
    var checksum = 0;

    foreach (var numbers in input)
    {
        var found = false;

        for (var i = 0; i < numbers.Count && !found; i++)
        {
            for (var j = i + 1; j < numbers.Count && !found; j++)
            {
                var a = numbers.ElementAt(i);
                var b = numbers.ElementAt(j);

                if (a % b == 0)
                {
                    checksum += a / b;
                    found = true;
                }
                else if (b % a == 0)
                {
                    checksum += b / a;
                    found = true;
                }
            }
        }
    }

    return checksum;
}

var input = File.ReadAllLines("../input.txt")
    .Select(line => line.Split('\t').Select(int.Parse).ToHashSet())
    .ToArray();

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));