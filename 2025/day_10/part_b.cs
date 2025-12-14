#:package Microsoft.Z3@*

var lines = File.ReadAllLines("input.txt").ToList();

var machines = new List<Machine>();
foreach (var line in lines)
{
    var groups = line.Split(' ');

    var target = groups.Last()[1..^1].Split(',').Select(int.Parse).ToArray();

    var buttons = groups
        .Where(g => g[0] == '(')
        .Select(g =>
                {
                    var temp = g[1..(g.Length - 1)].Split(',').Select(b => int.Parse(b));

                    return new Button(temp);

                });

    machines.Add(new Machine(target, buttons));
}

var answer = 0;
foreach (var machine in machines)
{
    using var ctx = new Microsoft.Z3.Context();
    var optimizer = ctx.MkOptimize();

    // Create variables for each button (how many times each button is pressed)
    var buttonVars = machine.Buttons
        .Select((_, i) => ctx.MkIntConst($"button_{i}"))
        .ToArray();

    // Constraint: each button must be pressed >= 0 times
    foreach (var buttonVar in buttonVars)
    {
        optimizer.Assert(ctx.MkGe(buttonVar, ctx.MkInt(0)));
    }

    // Constraint: for each counter, the sum of button presses that affect it must equal the target
    for (int counter = 0; counter < machine.Target.Length; counter++)
    {
        var terms = new List<Microsoft.Z3.ArithExpr>();

        for (int buttonIx = 0; buttonIx < machine.Buttons.Count(); buttonIx++)
        {
            var button = machine.Buttons.ElementAt(buttonIx);

            if (button.Lights.Contains(counter))
            {
                terms.Add(buttonVars[buttonIx]);
            }
        }

        var sum = ctx.MkAdd(terms);
        optimizer.Assert(ctx.MkEq(sum, ctx.MkInt(machine.Target[counter])));
    }

    // Minimize the total number of button presses
    var totalPresses = ctx.MkAdd(buttonVars);
    optimizer.MkMinimize(totalPresses);

    // Solve
    if (optimizer.Check() == Microsoft.Z3.Status.SATISFIABLE)
    {
        var model = optimizer.Model;
        var minPresses = 0;

        foreach (var buttonVar in buttonVars)
        {
            var presses = ((Microsoft.Z3.IntNum)model.Evaluate(buttonVar)).Int;
            minPresses += presses;
        }

        answer += minPresses;
        Console.WriteLine($"Machine solved: {minPresses} total presses");
    }
    else
    {
        Console.WriteLine("No solution found for machine");
    }
}

Console.WriteLine("answer: " + answer);

class Machine
{
    public Machine(int[] target, IEnumerable<Button> buttons)
    {
        Target = target;
        Buttons = buttons;
    }

    public int[] Target;

    public IEnumerable<Button> Buttons;

    public override string ToString()
    {
        return $"{string.Join(",", Target)}: {string.Join(" - ", Buttons.Select(b => b.ToString()))}";
    }
}

class Button(IEnumerable<int> lights)
{
    public IEnumerable<int> Lights => lights;

    public override string ToString()
    {
        return String.Join(',', lights);
    }
}