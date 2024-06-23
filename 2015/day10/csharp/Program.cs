using System.Text;

int Part1And2(string input, int part)
{
    var sb = new StringBuilder(input);

    foreach (var _ in Enumerable.Range(0, part == 1 ? 40 : 50))
    {
        var newSb = new StringBuilder();

        for (var i = 0; i < sb.Length; i++)
        {
            var c = sb[i];
            var count = 1;
            
            while (i + 1 < sb.Length && sb[i + 1] == c)
            {
                count++;
                i++;
            }

            newSb.Append(count);
            newSb.Append(c);
        }

        sb = newSb;
    }

    return sb.Length;
}

var input = File.ReadAllText("../input.txt");

Console.WriteLine(Part1And2(input, 1));
Console.WriteLine(Part1And2(input, 2));