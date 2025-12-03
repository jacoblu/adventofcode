var dial = 50;
var password = 0;

var instructions = File.ReadAllLines("input.txt")
    .Select(line => (turn: line[0], steps: int.Parse(line[1..])))
    .ToList();

foreach (var (line, steps) in instructions)
{
    var revolutions = steps / 100;
    var remainder = steps % 100;

    password += revolutions;

    var initialDial = dial;
    var newDial = line == 'R' ? dial + remainder : dial - remainder;

    dial = newDial % 100;

    if (dial < 0)
        dial = 100 + dial;
    Console.WriteLine(dial);

    if (initialDial == 0)
    {
        continue;
    }

    if (dial == 0)
    {
        password++;
        continue;
    }

    if (newDial > 100 || newDial < 0)
    {
        password++;
    }
}

Console.WriteLine("password: " + password);