using System.Text.RegularExpressions;

List<List<T>> GetPermutations<T>(List<T> input)
{
    if (input.Count == 1) { return [input]; }
    else
    {
        var result = new List<List<T>>();

        for (int i = 0; i < input.Count; i++)
        {
            var item = input[i];
            var remainingItems = input.Take(i).Concat(input.Skip(i + 1)).ToList();

            foreach (var subPermutation in GetPermutations(remainingItems))
            {
                subPermutation.Insert(0, item);
                result.Add(subPermutation);
            }
        }

        return result;
    }
}

var regex = new Regex(@"(\w+) to (\w+) = (\d+)");
var input = new HashSet<Route>();

foreach (var line in File.ReadLines("../input.txt"))
{
    var match = regex.Match(line);
    var route = new Route(match.Groups[1].Value, match.Groups[2].Value, int.Parse(match.Groups[3].Value));
    input.Add(route);
    input.Add(route.Invert());
}

var permutations = GetPermutations(input.Select(route => route.From).Distinct().ToList());
var computedPermutations = permutations.Select(permutation => permutation.Zip(permutation.Skip(1), (from, to) => input.Single(route => route.From == from && route.To == to).Distance).Sum()).ToList();

Console.WriteLine(computedPermutations.Min());
Console.WriteLine(computedPermutations.Max());

record struct Route(string From, string To, int Distance)
{
    public readonly Route Invert() => new(To, From, Distance);
}