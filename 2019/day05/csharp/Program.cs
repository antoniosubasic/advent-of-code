int Part1And2(int[] input, int inputParameter)
{
    var diagnosticCode = 0;

    for (var i = 0; i < input.Length;)
    {
        var opcode = (Opcode)(input[i] % 100);
        var mode1 = (Mode)(input[i] / 100 % 10);
        var mode2 = (Mode)(input[i] / 1000 % 10);
        var mode3 = (Mode)(input[i] / 10000 % 10);

        var first = input.ElementAtOrDefault(i + 1);
        var second = input.ElementAtOrDefault(i + 2);
        var third = input.ElementAtOrDefault(i + 3);

        var param1 = mode1 == Mode.Position ? input.ElementAtOrDefault(first) : first;
        var param2 = mode2 == Mode.Position ? input.ElementAtOrDefault(second) : second;
        var param3 = mode3 == Mode.Position ? input.ElementAtOrDefault(third) : third;

        switch (opcode)
        {
            case Opcode.Add:
            case Opcode.Multiply:
                input[input[i + 3]] = opcode == Opcode.Add ? param1 + param2 : param1 * param2;
                i += 4;
                break;

            case Opcode.Input:
                input[input[i + 1]] = inputParameter;
                i += 2;
                break;

            case Opcode.Output:
                diagnosticCode = param1;
                i += 2;
                break;

            case Opcode.JumpIfTrue:
            case Opcode.JumpIfFalse:
                if (opcode == Opcode.JumpIfTrue ? param1 != 0 : param1 == 0) { i = param2; }
                else { i += 3; }
                break;

            case Opcode.LessThan:
            case Opcode.Equals:
                input[input[i + 3]] = (opcode == Opcode.LessThan ? param1 < param2 : param1 == param2) ? 1 : 0;
                i += 4;
                break;

            case Opcode.Halt:
                return diagnosticCode;

            default:
                throw new InvalidOperationException("invalid opcode");
        }
    }

    throw new Exception("no diagnostic code found");
}


var input = File.ReadAllText("../input.txt").Split(',').Select(int.Parse).ToArray();

Console.WriteLine(Part1And2([.. input], 1));
Console.WriteLine(Part1And2(input, 5));


enum Mode
{
    Position,
    Immediate
}

enum Opcode
{
    Add = 1,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt = 99
}
