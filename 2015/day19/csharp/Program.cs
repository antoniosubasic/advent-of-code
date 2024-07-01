using System.Text.RegularExpressions;

var input = File.ReadAllLines("../input.txt");

var molecule = input[^1];
var replacements = input[..^2].Select(line =>
{
    var splitted = line.Split(" => ");
    return new Replacement(splitted[0], splitted[1]);
}).ToArray();

var molecules = new HashSet<string>();

for (var i = 0; i < replacements.Length; i++)
{
    var re = new Regex(replacements[i].From);

    foreach (Match match in re.Matches(molecule))
    {
        molecules.Add($"{molecule[..match.Index]}{replacements[i].To}{molecule[(match.Index + match.Length)..]}");
    }
}

Console.WriteLine(molecules.Count);

var steps = 0;

while (molecule != "e")
{
    var temp = molecule;

    foreach (var replacement in replacements)
    {
        if (molecule.Contains(replacement.To))
        {
            var index = molecule.IndexOf(replacement.To);
            molecule = $"{molecule[..index]}{replacement.From}{molecule[(index + replacement.To.Length)..]}";
            steps++;
        }
    }

    if (temp == molecule)
    {
        molecule = input[^1];
        steps = 0;
        replacements = [.. replacements.OrderBy(_ => Random.Shared.Next())];
    }
}

Console.WriteLine(steps);

record struct Replacement(string From, string To);
