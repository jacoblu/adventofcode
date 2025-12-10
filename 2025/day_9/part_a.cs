var tiles = File.ReadAllLines("input.txt")
    .Select(l => (x: long.Parse(l.Split(',')[0]), y: long.Parse(l.Split(',')[1])))
    .ToList();

var combinations = tiles.SelectMany(x => tiles, (a, b) => (a, b)).ToList();

long answer = 0;
foreach (var (a, b) in combinations)
{
    var area = (Math.Abs(a.x - b.x) + 1) * (Math.Abs(a.y - b.y) + 1);

    if (area > answer)
    {
        answer = Math.Max(answer, area);
    }
}

Console.WriteLine("answer: " + answer);