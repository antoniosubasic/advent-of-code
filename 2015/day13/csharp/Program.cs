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

int GetHighestPossibleHappiness(List<Rule> input)
{
    var permutations = GetPermutations(input.Select(rule => rule.Person).Distinct().ToList());

    return permutations.Select(permutation =>
    {
        var total = 0;

        for (int i = 0; i < permutation.Count; i++)
        {
            var current = permutation[i];
            var next = permutation[(i + 1) % permutation.Count];

            var currentRule = input.First(rule => rule.Person == current && rule.NextTo == next);
            var nextRule = input.First(rule => rule.Person == next && rule.NextTo == current);

            total += currentRule.Gain + nextRule.Gain;
        }

        return total;
    }).Max();
}

var regex = new Regex(@"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)");
var input = File.ReadAllLines("../input.txt").Select(line =>
{
    var match = regex.Match(line);
    return new Rule(match.Groups[1].Value, int.Parse(match.Groups[3].Value) * (match.Groups[2].Value == "gain" ? 1 : -1), match.Groups[4].Value);
}).ToList();

Console.WriteLine(GetHighestPossibleHappiness(input));

foreach (var person in input.Select(rule => rule.Person).Distinct().ToArray())
{
    input.Add(new Rule("Me", 0, person));
    input.Add(new Rule(person, 0, "Me"));
}

Console.WriteLine(GetHighestPossibleHappiness(input));

record struct Rule(string Person, int Gain, string NextTo);