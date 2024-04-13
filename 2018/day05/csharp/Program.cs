int Part1(string input)
{
    bool reacted;

    do
    {
        reacted = false;

        for (var i = 0; i < input.Length - 1; i++)
        {
            var first = input[i];
            var second = input[i + 1];

            if (first != second && char.ToLower(first) == char.ToLower(second))
            {
                input = $"{input[..i]}{input[(i + 2)..]}";
                reacted = true;
            }
        }
    } while (reacted);

    return input.Length;
}

int Part2(string input)
{
    var min = input.Length;

    foreach (var c in input.ToLower().ToHashSet())
    {
        min = Math.Min(min, Part1(input.Replace(c.ToString(), "").Replace(char.ToUpper(c).ToString(), "")));
    }

    return min;
}

var input = File.ReadAllText("../input.txt").TrimEnd('\n');

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));
