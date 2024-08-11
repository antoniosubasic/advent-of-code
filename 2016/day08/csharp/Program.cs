using System.Text;
using System.Text.RegularExpressions;

var input = File.ReadAllLines("../input.txt");
var rectRegex = new Regex(@"rect (\d+)x(\d+)");
var rotateRegex = new Regex(@"rotate (row|column) (x|y)=(\d+) by (\d+)");

var screen = new Screen();

foreach (var instruction in input)
{
    var rectMatch = rectRegex.Match(instruction);
    var rotateMatch = rotateRegex.Match(instruction);

    if (rectMatch.Success)
    {
        screen.Rect(int.Parse(rectMatch.Groups[1].Value), int.Parse(rectMatch.Groups[2].Value));
    }
    else if (rotateMatch.Success)
    {
        var a = int.Parse(rotateMatch.Groups[3].Value);
        var b = int.Parse(rotateMatch.Groups[4].Value);

        if (rotateMatch.Groups[1].Value == "row") { screen.RotateRow(a, b); }
        else { screen.RotateColumn(a, b); }
    }
}

Console.WriteLine(screen.PixelsLit);
Console.WriteLine(screen.ToString());

class Screen
{
    private bool[,] _pixels = new bool[6, 50];

    public void Rect(int a, int b)
    {
        for (var y = 0; y < b; y++)
        {
            for (var x = 0; x < a; x++)
            {
                _pixels[y, x] = true;
            }
        }
    }

    public void RotateRow(int a, int b)
    {
        var row = new bool[50];

        for (var x = 0; x < 50; x++)
        {
            row[(x + b) % 50] = _pixels[a, x];
        }

        for (var x = 0; x < 50; x++)
        {
            _pixels[a, x] = row[x];
        }
    }

    public void RotateColumn(int a, int b)
    {
        var column = new bool[6];

        for (var y = 0; y < 6; y++)
        {
            column[(y + b) % 6] = _pixels[y, a];
        }

        for (var y = 0; y < 6; y++)
        {
            _pixels[y, a] = column[y];
        }
    }

    public int PixelsLit => _pixels.Cast<bool>().Count(p => p);

    public override string ToString()
    {
        var sb = new StringBuilder();

        for (var y = 0; y < 6; y++)
        {
            for (var x = 0; x < 50; x++)
            {
                sb.Append(_pixels[y, x] ? '#' : '.');
            }

            sb.AppendLine();
        }

        return sb.ToString();
    }
}
