using System.Collections;
using System.Text;
using System.Text.RegularExpressions;

int Part1(string[] input)
{
    var sum = 0;

    for (var i = 0; i < input.Length; i++)
    {
        var regex = new Regex(@"\d{1,3}");
        var matches = regex.Matches(input[i]);

        foreach (var match in matches.Cast<Match>())
        {
            var characters = new List<char>();

            var hasCharLeft = match.Index > 0;
            var hasCharRight = match.Index + match.Length < input[i].Length;

            (int left, int right) indices = (
                hasCharLeft ? match.Index - 1 : match.Index,
                hasCharRight ? (match.Index + match.Length) : (match.Index + match.Length - 1)
            );

            if (hasCharLeft) { characters.Add(input[i][indices.left]); }
            if (hasCharRight) { characters.Add(input[i][indices.right]); }
            if (i > 0) { characters.AddRange(input[i - 1][indices.left..(indices.right + 1)]); }
            if (i + 1 < input.Length) { characters.AddRange(input[i + 1][indices.left..(indices.right + 1)]); }

            if (characters.Any(c => !char.IsDigit(c) && c != '.')) { sum += int.Parse(match.ToString()); }
        }
    }

    return sum;
}

int Part2(string[] input)
{
    var sum = 0;

    for (var i = 0; i < input.Length; i++)
    {
        var regex = new Regex(@"\*");
        var matches = regex.Matches(input[i]);

        foreach (var match in matches.Cast<Match>())
        {
            var currentGear = new Character(new(i, match.Index), '*');

            var hasCharLeft = match.Index > 0;
            var hasCharRight = match.Index + match.Length < input[i].Length;

            (int left, int right) indices = (
                hasCharLeft ? match.Index - 1 : match.Index,
                hasCharRight ? (match.Index + match.Length) : (match.Index + match.Length - 1)
            );

            if (hasCharLeft) { currentGear.AddAdjacentLeft(input[i][indices.left]); }
            if (hasCharRight) { currentGear.AddAdjacentRight(input[i][indices.right]); }
            for (var j = indices.left; j <= indices.right; j++)
            {
                if (i > 0) { currentGear.AddAdjacent(new(new(i - 1, j), input[i - 1][j])); }
                if (i + 1 < input.Length) { currentGear.AddAdjacent(new(new(i + 1, j), input[i + 1][j])); }
            }


            var gears = new Dictionary<Coordinate, HashSet<int>>();

            foreach (var character in currentGear.Adjacent?.Where(character => char.IsDigit(character)) ?? [])
            {
                var buffer = new StringBuilder(character.ToString());
                (bool start, bool end) reached = (false, false);

                for (var j = 1; j <= 3 && (!reached.start || !reached.end); j++)
                {
                    reached = (
                        reached.start || character.Cords.Y - j < 0,
                        reached.end || character.Cords.Y + j >= input[character.Cords.Y].Length
                    );

                    if (!reached.end && char.IsDigit(input[character.Cords.X][character.Cords.Y + j]))
                    {
                        buffer.Append(input[character.Cords.X][character.Cords.Y + j]);
                    }
                    else { reached.end = true; }

                    if (!reached.start && char.IsDigit(input[character.Cords.X][character.Cords.Y - j]))
                    {
                        buffer.Insert(0, input[character.Cords.X][character.Cords.Y - j]);
                    }
                    else { reached.start = true; }
                }

                if (gears.ContainsKey(currentGear.Cords)) { gears[currentGear.Cords].Add(int.Parse(buffer.ToString())); }
                else { gears.Add(currentGear.Cords, [int.Parse(buffer.ToString())]); }
            }

            foreach (var gear in gears)
            {
                if (gear.Value.Count == 2) { sum += gear.Value.First() * gear.Value.ElementAt(1); }
            }
        }
    }

    return sum;
}


var input = File.ReadAllLines("../input.txt");

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));


class Character(Coordinate cords, char @char) : IEnumerable<Character>
{
    public Coordinate Cords { get; private set; } = cords;
    public char Char { get; private set; } = @char;
    public HashSet<Character>? Adjacent { get; private set; }

    public void AddAdjacent(Character character)
    {
        Adjacent ??= [];
        Adjacent.Add(character);
    }

    public void AddAdjacentLeft(char character)
    {
        AddAdjacent(new Character(new(Cords.X, Cords.Y - 1), character));
    }

    public void AddAdjacentRight(char character)
    {
        AddAdjacent(new Character(new(Cords.X, Cords.Y + 1), character));
    }

    public IEnumerator<Character> GetEnumerator() => Adjacent?.GetEnumerator() ?? Enumerable.Empty<Character>().GetEnumerator();

    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

    public override string ToString() => Char.ToString();

    public static implicit operator char(Character character) => character.Char;
}

record struct Coordinate(int X, int Y);
