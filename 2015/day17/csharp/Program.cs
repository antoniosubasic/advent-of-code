const int EGGNOG = 150;

var input = File.ReadAllLines("../input.txt").Select(int.Parse).ToArray();

int? minContainersUsed = null;
int minCombinations = 0, combinations = 0;

for (var i = 1; i < Math.Pow(2, input.Length); i++)
{
    var totalEggnog = 0;
    var containersUsed = 0;

    for (var j = 0; j < input.Length; j++)
    {
        if ((i & (1 << j)) != 0)
        {
            totalEggnog += input[j];
            containersUsed++;
        }
    }

    if (totalEggnog == EGGNOG)
    {
        combinations++;

        if (minContainersUsed is null || containersUsed < minContainersUsed)
        {
            minContainersUsed = containersUsed;
            minCombinations = 1;
        }
        else if (containersUsed == minContainersUsed) { minCombinations++; }
    }
}

Console.WriteLine(combinations);
Console.WriteLine(minCombinations);