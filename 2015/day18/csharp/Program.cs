const int STEPS = 100;

var input = File.ReadAllLines("../input.txt");
var lightmap = new Lightmap(input);

lightmap.Step(STEPS);
Console.WriteLine(lightmap.LightsOn);

lightmap = new Lightmap(input);
lightmap.LockCorners();

lightmap.Step(STEPS);
Console.WriteLine(lightmap.LightsOn);

class Lightmap
{
    private (bool on, bool locked)[,] _lights;
    private (bool on, bool locked)[,] _nextLights;

    public int Height => _lights.GetLength(0);
    public int Width => _lights.GetLength(1);
    public int LightsOn => _lights.Cast<(bool on, bool locked)>().Count(light => light.on);

    public Lightmap(string[] map)
    {
        _lights = new (bool, bool)[map.Length, map[0].Length];
        _nextLights = new (bool, bool)[map.Length, map[0].Length];

        for (int i = 0; i < map.Length; i++)
        {
            for (int j = 0; j < map[i].Length; j++)
            {
                _lights[i, j].on = map[i][j] == '#';
            }
        }
    }

    private int CountNeighbors(int x, int y)
    {
        var count = 0;

        for (int i = -1; i <= 1; i++)
        {
            for (int j = -1; j <= 1; j++)
            {
                if (i != 0 || j != 0)
                {
                    var nx = x + i;
                    var ny = y + j;

                    if (nx >= 0 && nx < _lights.GetLength(0) && ny >= 0 && ny < _lights.GetLength(1) && _lights[nx, ny].on)
                    {
                        count++;
                    }
                }
            }
        }

        return count;
    }

    public void LockCorners()
    {
        foreach (var (x, y) in new (int, int)[] { (0, 0), (0, Width - 1), (Height - 1, 0), (Height - 1, Width - 1) })
        {
            _lights[x, y].on = _lights[x, y].locked = _nextLights[x, y].on = _nextLights[x, y].locked = true;
        }
    }

    public void Step(uint steps = 1)
    {
        for (var step = 0; step < steps; step++)
        {
            for (var i = 0; i < _lights.GetLength(0); i++)
            {
                for (var j = 0; j < _lights.GetLength(1); j++)
                {
                    if (!_lights[i, j].locked)
                    {
                        var count = CountNeighbors(i, j);

                        if (_lights[i, j].on) { _nextLights[i, j].on = count is 2 or 3; }
                        else { _nextLights[i, j].on = count == 3; }
                    }
                }
            }

            _lights = ((bool, bool)[,])_nextLights.Clone();
        }
    }
}