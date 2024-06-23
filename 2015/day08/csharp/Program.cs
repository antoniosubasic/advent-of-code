using System.Text.RegularExpressions;

int Part1(string[] input) => input.Sum(str => str.Length - Regex.Unescape(str).Length + 2);

int Part2(string[] input)
{
    var encodedChars = 0;

    foreach (var str in input)
    {
        var encodedStr = new Regex(@"\\|""|\\x[0-9a-f]{2}").Replace(str, match => match.Value switch
        {
            "\\" => @"\\",
            "\"" => @"\""",
            _ => @"\\x"
        });
        
        encodedChars += encodedStr.Length + 2 - str.Length;
    }

    return encodedChars;
}

var input = File.ReadAllLines("../input.txt");

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));
