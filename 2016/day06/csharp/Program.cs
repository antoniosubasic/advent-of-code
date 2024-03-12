using System.Text;

string Part1And2(string[] input, int part)
{
    var sb = new StringBuilder();

    for (var i = 0; i < input[0].Length; i++)
    {
        var letterCount = new Dictionary<char, int>();

        for (var j = 0; j < input.Length; j++)
        {
            if (letterCount.TryGetValue(input[j][i], out var value)) { letterCount[input[j][i]] = ++value; }
            else { letterCount.Add(input[j][i], 1); }
        }

        var c = (part == 1 ? letterCount.MaxBy(letter => letter.Value) : letterCount.MinBy(letter => letter.Value)).Key;
        sb.Append(c);
    }

    return sb.ToString();
}

var input = File.ReadAllLines("../input.txt");

Console.WriteLine(Part1And2(input, 1));
Console.WriteLine(Part1And2(input, 2));