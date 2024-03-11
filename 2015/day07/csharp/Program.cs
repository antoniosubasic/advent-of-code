ushort Part1(Instruction[] input) => new Circuit(input).GetValue("a");

ushort Part2(Instruction[] input, ushort valueToOverrideB)
{
    var circuit = new Circuit();

    foreach (var instruction in input)
    {
        if (instruction.Destination == "b") { circuit.AddWire(instruction.Destination, new Value(valueToOverrideB)); }
        else { circuit.AddWire(instruction.Destination, new Value(instruction.Operands.first, instruction.Operands.second, instruction.Operation)); }
    }

    return circuit.GetValue("a");
}


var input = File.ReadAllLines("../input.txt")
    .Select(line =>
    {
        var split = line.Split(' ');
        return split.Length switch
        {
            4 => new Instruction(
                (split[1], null),
                Operation.Not,
                split[^1]
            ),
            3 => new Instruction(
                (split[0], null),
                Operation.Set,
                split[^1]
            ),
            _ => new Instruction(
                (split[0], split[2]),
                Enum.Parse<Operation>(split[1], true),
                split[4]
            ),
        };
    }).ToArray();

var part1WireA = Part1(input);
var part2WireA = Part2(input, part1WireA);
Console.WriteLine(part1WireA);
Console.WriteLine(part2WireA);


record Instruction((string first, string? second) Operands, Operation Operation, string Destination);

class Circuit
{
    private Dictionary<string, Value> wires = [];

    public Circuit(Instruction[] instructions)
    {
        foreach (var instruction in instructions)
        {
            wires.Add(instruction.Destination, new Value(instruction.Operands.first, instruction.Operands.second, instruction.Operation));
        }
    }

    public Circuit() { }

    public void AddWire(string name, Value value)
    {
        wires.Add(name, value);
    }

    public ushort GetValue(string wire)
    {
        if (wires.TryGetValue(wire, out Value? value))
        {
            if (value.Numerical.HasValue) { return value.Numerical.Value; }
            else if (value.Wires is null) { throw new NullReferenceException("Wire has no value"); }
            else
            {
                var (first, second) = value.Wires.Value;

                if (first is null) { throw new NullReferenceException("First source wire is null"); }
                else
                {
                    var firstValue = ushort.TryParse(first, out var firstNumerical) ? firstNumerical : GetValue(first);
                    ushort wireValue;

                    if (second is null)
                    {
                        wireValue = value.Operation switch
                        {
                            Operation.Not => (ushort)~firstValue,
                            Operation.Set => firstValue,
                            _ => throw new NotImplementedException()
                        };
                    }
                    else
                    {
                        var secondValue = ushort.TryParse(second, out var secondNumerical) ? secondNumerical : GetValue(second);
                        wireValue = value.Operation switch
                        {
                            Operation.And => (ushort)(firstValue & secondValue),
                            Operation.Or => (ushort)(firstValue | secondValue),
                            Operation.LShift => (ushort)(firstValue << secondValue),
                            Operation.RShift => (ushort)(firstValue >> secondValue),
                            _ => throw new NotImplementedException()
                        };
                    }

                    wires[wire] = new Value(wireValue);
                    return wireValue;
                }
            }
        }
        else { throw new KeyNotFoundException($"Wire {wire} not found"); }
    }
}

class Value
{
    public ushort? Numerical { get; set; }
    public (string? first, string? second)? Wires { get; set; }
    public Operation? Operation { get; set; }

    public Value(string first, string? second, Operation operation)
    {
        Wires = (first, second);
        Operation = operation;
    }

    public Value(ushort value)
    {
        Numerical = value;
        Operation = global::Operation.Set;
        Wires = null;
    }
}

enum Operation
{
    And,
    Or,
    LShift,
    RShift,
    Not,
    Set
}
