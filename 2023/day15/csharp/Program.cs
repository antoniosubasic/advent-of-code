int Part1(Lens[] input) => input.Sum(lens => lens.GetHash());

int Part2(Lens[] input)
{
    var hashmap = new Dictionary<int, List<Lens>>();

    foreach (var lens in input)
    {
        var label = lens.GetLabelHash();
        hashmap.TryGetValue(label, out List<Lens>? value);

        if (lens.Operation == '-')
        {
            if (value is not null)
            {
                for (var i = 0; i < value.Count; i++)
                {
                    if (value[i].Label == lens.Label)
                    {
                        value.RemoveAt(i);
                        break;
                    }
                }

                if (value.Count == 0) { hashmap.Remove(label); }
            }
        }
        else
        {
            if (value is null) { hashmap.Add(label, [lens]); }
            else
            {
                var existingLens = value.FirstOrDefault(l => l.Label == lens.Label);

                if (existingLens is not null) { existingLens.Value = lens.Value; }
                else { value.Add(lens); }
            }
        }
    }

    var sum = 0;

    for (var i = 0; i < hashmap.Count; i++)
    {
        var boxLabel = hashmap.Keys.ElementAt(i);

        for (var j = 0; j < hashmap[boxLabel].Count; j++)
        {
            sum += (boxLabel + 1) * (j + 1) * hashmap[boxLabel][j].Value!.Value;
        }
    }

    return sum;
}


var input = File.ReadAllText("../input.txt").Split(',').Select(box => new Lens(box)).ToArray();

Console.WriteLine(Part1(input));
Console.WriteLine(Part2(input));


class Lens
{
    public string Label { get; } = "";
    public char Operation { get; }
    public int? Value { get; set; }

    public Lens(string input)
    {
        var index = input.Select((chr, i) => (chr, i)).FirstOrDefault(item => item.chr is '=' or '-').i;
        
        Label = input[..index];
        Operation = input[index];
        Value = Operation == '-' ? null : int.Parse(input[(index + 1)..]);
    }

    public int GetHash()
    {
        var hash = 0;

        foreach (var c in $"{Label}{Operation}{Value}")
        {
            hash += c;
            hash *= 17;
            hash %= 256;
        }

        return hash;
    }

    public int GetLabelHash()
    {
        var hash = 0;

        foreach (var c in Label)
        {
            hash += c;
            hash *= 17;
            hash %= 256;
        }

        return hash;
    }
}
