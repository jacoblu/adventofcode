var lines = File.ReadAllLines("input.txt")
    .ToList();

var operations = lines.Last();
var numbers = lines.Take(lines.Count - 1).ToList();

long answer = 0;
long partial = 0;
var operation = ' ';
for (int i = 0; i < operations.Length; i++)
{
    if (operations[i] != ' ')
    {
        operation = operations[i];

        partial = GetColumnNumber(i, numbers);
        continue;
    }

    var number = GetColumnNumber(i, numbers);
    if (number == 0)
    {
        answer += partial;
    }
    else
    {
        if (operation == '*')
        {
            partial *= number;
        }
        else
        {
            partial += number;
        }
    }
}

// Add the last partial
answer += partial;

Console.WriteLine("answer: " + answer);

long GetColumnNumber(int column, List<string> rows)
{
    long number = 0;
    for (int i = 0; i < rows.Count; i++)
    {
        var character = rows[i][column];
        if (character != ' ')
        {
            number = (number * 10) + long.Parse(character.ToString());
        }
    }
    return number;
}