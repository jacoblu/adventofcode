var answer = 0;

var lines = File.ReadAllLines("input.txt")
    .ToList();

var size = lines.Count;
var max = size - 1;

var papers = new Dictionary<(int, int), (int blockers, List<(int x, int y)> blocking)>();

for (int x = 0; x <= max; x++)
{
    var line = lines[x];

    for (int y = 0; y <= max; y++)
    {
        var item = line[y];
        if (item != '@')
        {
            continue;
        }

        var blockers = 0;
        var blocking = new List<(int x, int y)>();
        // ↘️
        if (x < max && y < max)
        {
            blockers += Process(x + 1, y + 1, lines, blocking);
        }

        // ↗️
        if (x > 0 && y < max)
        {
            blockers += Process(x - 1, y + 1, lines, blocking);
        }

        // ↙️
        if (x < max && y > 0)
        {
            blockers += Process(x + 1, y - 1, lines, blocking);
        }

        // ↖️
        if (x > 0 && y > 0)
        {
            blockers += Process(x - 1, y - 1, lines, blocking);
        }

        // ⬆️
        if (y > 0)
        {
            blockers += Process(x, y - 1, lines, blocking);
        }

        // ➡️
        if (x < max)
        {
            blockers += Process(x + 1, y, lines, blocking);
        }

        // ⬅️
        if (x > 0)
        {
            blockers += Process(x - 1, y, lines, blocking);
        }

        // ⬇️
        if (y < max)
        {
            blockers += Process(x, y + 1, lines, blocking);
        }

        papers[(x, y)] = (blockers, blocking);
    }
}

while (true)
{
    var removable = papers.Where(p => p.Value.blockers <= 3);
    if (removable.Count() == 0)
    {
        break;
    }

    foreach (var item in removable)
    {
        answer++;
        papers.Remove(item.Key);

        foreach (var blocked in item.Value.blocking)
        {
            if (papers.ContainsKey(blocked))
            {
                var current = papers[blocked];
                papers[blocked] = (current.blockers - 1, current.blocking);
            }
        }
    }
}

Console.WriteLine("answer: " + answer);

static int Process(int x, int y, List<string> map, List<(int x, int y)> blocking)
{
    if (map[x][y] == '@')
    {
        blocking.Add((x, y));
        return 1;
    }

    return 0;
}