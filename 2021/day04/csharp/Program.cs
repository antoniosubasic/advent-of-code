var input = File.ReadAllText("../input.txt");

var drawn = input.Split('\n')[0].Split(',').Select(int.Parse).ToArray();
var boards = input.Split("\n\n")[1..].Select(board => new Board(board)).OrderBy(board => board.MarkRange(drawn)).ToArray();

Console.WriteLine(boards.First().Score);
Console.WriteLine(boards.Last().Score);

class Board
{
    public (int value, bool marked)[][] Numbers { get; }
    public int Height => Numbers.Length;
    public int Width => Numbers[0].Length;

    public bool IsBingo => Numbers.Any(row => row.All(num => num.marked)) || Enumerable.Range(0, Width).Any(i => Numbers.All(row => row[i].marked));
    public int Score
    {
        get
        {
            var score = 0;

            for (var y = 0; y < Height; y++)
            {
                for (var x = 0; x < Width; x++)
                {
                    if (!Numbers[y][x].marked)
                    {
                        score += Numbers[y][x].value;
                    }
                }
            }

            return score * Numbers[_latestMove.y][_latestMove.x].value;
        }
    }

    private (Index y, Index x) _latestMove { get; set; }

    public Board(string board)
    {
        var lines = board.Split('\n', StringSplitOptions.RemoveEmptyEntries).Select(line => line.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray()).ToArray();
        Numbers = new (int, bool)[lines.Length][];

        for (var y = 0; y < Numbers.Length; y++)
        {
            Numbers[y] = new (int, bool)[lines[y].Length];

            for (var x = 0; x < Numbers[y].Length; x++)
            {
                Numbers[y][x] = (lines[y][x], false);
            }
        }
    }

    public int MarkRange(int[] numbers)
    {
        for (var i = 0; i < numbers.Length; i++)
        {
            for (var y = 0; y < Height; y++)
            {
                for (var x = 0; x < Width; x++)
                {
                    if (Numbers[y][x].value == numbers[i])
                    {
                        Numbers[y][x].marked = true;
                        _latestMove = (y, x);

                        if (IsBingo) { return i; }
                    }
                }
            }
        }

        return numbers.Length - 1;
    }
}
