int Part1And2(int[][] input, int part)
{
    var extrapolatedValues = 0;

    foreach (var history in input)
    {
        var differences = new List<List<int>> { new(history) };

        while (!differences.Last().All(num => num == 0))
        {
            differences.Add([]);
            var currentValues = differences.ElementAt(^2);

            for (var i = 0; i < currentValues.Count - 1; i++)
            {
                differences.Last().Add(currentValues.ElementAt(i + 1) - currentValues.ElementAt(i));
            }
        }

        for (var i = differences.Count - 1; i > 0; i--)
        {
            if (part == 1)
            {
                differences.ElementAt(i - 1).Add(differences.ElementAt(i - 1).Last() + differences.ElementAt(i).Last());
            }
            else
            {

                differences.ElementAt(i - 1).Insert(0, differences.ElementAt(i - 1).First() - differences.ElementAt(i).First());
            }
        }

        extrapolatedValues += part == 1 ? differences.First().Last() : differences.First().First();
    }

    return extrapolatedValues;
}

var input = File.ReadAllLines("../input.txt").Select(line => line.Split(' ').Select(int.Parse).ToArray()).ToArray();

Console.WriteLine(Part1And2(input, 1));
Console.WriteLine(Part1And2(input, 2));
