var lines = File.ReadAllLines("input.txt").ToList();

var machines = new List<Machine>();
foreach (var line in lines)
{
    var groups = line.Split(' ');

    var targetString = groups.First()
                        .Replace("[", "")
                        .Replace("]", "")
                        .Replace(".", "0")
                        .Replace("#", "1")
                        .ToCharArray();
    Array.Reverse(targetString);
    var target = Convert.ToInt32(new string(targetString), 2);

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
    var targetHit = false;
    var presses = 0;

    var guesses = new List<int>() { 0 };
    while (!targetHit)
    {
        presses++;

        var newGuesses = new List<int>();
        foreach (var g in guesses)
        {
            foreach (var b in machine.Buttons)
            {
                var newGuess = b.Press(g);

                if (newGuess == machine.Target)
                {
                    targetHit = true;
                    answer += presses;
                    break;
                }

                newGuesses.Add(b.Press(g));
            }

            if (targetHit)
            {
                break;
            }
        }

        guesses = newGuesses;
    }

}

Console.WriteLine("answer: " + answer);

class Machine(int target, IEnumerable<Button> buttons)
{
    public int Target => target;
    public IEnumerable<Button> Buttons => buttons;

    public override string ToString()
    {
        return $"{target}: {string.Join(" - ", buttons.Select(b => b.ToString()))}";
    }
}

class Button(IEnumerable<int> lights)
{
    public override string ToString()
    {
        return String.Join(',', lights);
    }

    public int Press(int value)
    {
        foreach (var l in lights)
        {
            value ^= 1 << l;
        }
        return value;
    }
}