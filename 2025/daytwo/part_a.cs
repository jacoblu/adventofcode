long answer = 0;

var instructions = File.ReadAllText("input.txt")
    .Split(',')
    .ToList();

foreach (var range in instructions)
{
    var rangeSplit = range.Split('-');

    var start = Int64.Parse(rangeSplit[0]);
    var end = Int64.Parse(rangeSplit[1]);

    for (long i = start; i <= end; i++)
    {
        if (!IsValidId(i))
        {
            answer += i;
        }
    }
}

Console.WriteLine(answer);

bool IsValidId(long id)
{
    var idString = id.ToString();

    var middle = idString.Length / 2;

    var first = idString[..middle];
    var second = idString[middle..];

    return first != second;
}