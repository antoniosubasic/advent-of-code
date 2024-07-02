var targetProperties = """
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
""".Split('\n').Select(line =>
{
    var parts = line.Split(": ");
    return new { Name = parts[0], Value = int.Parse(parts[1]) };
}).ToDictionary(part => part.Name, part => part.Value);

var input = File.ReadAllLines("../input.txt").Select(line =>
{
    var parts = line.Split(", ");

    var number = int.Parse(parts[0].Split(": ")[0].Split(' ')[1]);
    var properties = parts.Select(part =>
    {
        var propertyParts = part.Split(": ");
        return new { Name = propertyParts[^2], Value = int.Parse(propertyParts[^1]) };
    }).ToDictionary(part => part.Name, part => part.Value);

    return new { Number = number, Properties = properties };
}).ToArray();

int? part1 = null, part2 = null;

foreach (var sue in input)
{
    if (!part1.HasValue && sue.Properties.All(property => property.Value == targetProperties[property.Key]))
    {
        part1 = sue.Number;
    }

    if (!part2.HasValue)
    {
        var isMatch = sue.Properties.All(property =>
        {
            var value = property.Value;
            var targetValue = targetProperties[property.Key];

            return property.Key switch
            {
                "cats" => value > targetValue,
                "trees" => value > targetValue,
                "pomeranians" => value < targetValue,
                "goldfish" => value < targetValue,
                _ => value == targetValue
            };
        });

        if (isMatch)
        {
            part2 = sue.Number;
        }
    }

    if (part1.HasValue && part2.HasValue) { break; }
}

Console.WriteLine(part1);
Console.WriteLine(part2);
