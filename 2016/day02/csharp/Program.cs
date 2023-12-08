using AoC.API;

string Part1(Direction[][] input)
{
    var keypad = new Keypad([['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']], 1, 1);
    return new string(input.Select(keypad.EvaluateDirections).ToArray());
}

string Part2(Direction[][] input)
{
    var keypad = new Keypad([['1'], ['2', '3', '4'], ['5', '6', '7', '8', '9'], ['A', 'B', 'C'], ['D']], 0, 2);
    return new string(input.Select(keypad.EvaluateDirections).ToArray());
}


var session = new Session(
    File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "cookie")),
    Directory.GetCurrentDirectory(),
    new(File.ReadAllText(Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".aoc", "regex")))
);

var input = (await session.GetInputLinesAsync()).Select(line => line.Select(c => c switch
    {
        'U' => Direction.Up,
        'R' => Direction.Right,
        'D' => Direction.Down,
        'L' => Direction.Left,
        _ => throw new Exception("Invalid direction")
    }).ToArray()).ToArray();

Console.WriteLine(await session.SubmitAnswerAsync(1, Part1(input)));
Console.WriteLine(await session.SubmitAnswerAsync(2, Part2(input)));


enum Direction
{
    Up,
    Right,
    Down,
    Left
}

class Keypad(char[][] keys, int x, int y)
{
    private char[][] _keys { get; } = keys;
    private int _x { get; set; } = x;
    private int _y { get; set; } = y;

    private bool _positionValid => _x >= 0 && _y >= 0 && _y < _keys.Length && _x < _keys[_y].Length;

    private bool TryMoveUp()
    {
        _y--;

        if (_y < 0 || _y >= _keys.Length)
        {
            _y++;
            return false;
        }
        else
        {
            var fromOffset = _x - (_keys[_y + 1].Length / 2);
            var maxToOffset = (_keys[_y].Length - 1 - (_keys[_y].Length / 2)) * (fromOffset < 0 ? -1 : 1);

            if (Math.Abs(fromOffset) > Math.Abs(maxToOffset)) { _y++; return false; }
            else
            {
                var oldX = _x;
                _x = ((_keys[_y].Length - (_keys[_y].Length % 2 == 0 ? 1 : 0)) / 2) + fromOffset;

                if (_x < 0 || _x > _keys[_y].Length)
                {
                    _y++;
                    _x = oldX;
                    return false;
                }
                else { return true; }
            }
        }
    }

    private bool TryMoveRight()
    {
        _x++;

        if (!_positionValid)
        {
            _x--;
            return false;
        }
        else { return true; }
    }

    private bool TryMoveDown()
    {
        _y++;

        if (_y < 0 || _y >= _keys.Length)
        {
            _y--;
            return false;
        }
        else
        {
            var fromOffset = _x - (_keys[_y - 1].Length / 2);
            var maxToOffset = (_keys[_y].Length - 1 - (_keys[_y].Length / 2)) * (fromOffset < 0 ? -1 : 1);

            if (Math.Abs(fromOffset) > Math.Abs(maxToOffset)) { _y--; return false; }
            else
            {
                var oldX = _x;
                _x = ((_keys[_y].Length - (_keys[_y].Length % 2 == 0 ? 1 : 0)) / 2) + fromOffset;

                if (_x < 0 || _x > _keys[_y].Length)
                {
                    _y--;
                    _x = oldX;
                    return false;
                }
                else { return true; }
            }
        }
    }

    private bool TryMoveLeft()
    {
        _x--;

        if (!_positionValid) { _x++; return false; }
        else { return true; }
    }

    public char EvaluateDirections(Direction[] directions)
    {
        foreach (var direction in directions)
        {
            switch (direction)
            {
                case Direction.Up:
                    TryMoveUp();
                    break;
                case Direction.Right:
                    TryMoveRight();
                    break;
                case Direction.Down:
                    TryMoveDown();
                    break;
                case Direction.Left:
                    TryMoveLeft();
                    break;
            }
        }

        return _keys[_y][_x];
    }
}
