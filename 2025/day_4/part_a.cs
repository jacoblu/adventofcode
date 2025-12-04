var answer = 0;

var lines = File.ReadAllLines("input.txt")
    .ToList();

var size = lines.Count;
var max = size - 1;

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
        // ↘️
        if (x < max && y < max)
        {
            blockers += GetBlocker(x + 1, y + 1, lines);
        }

        // ↗️
        if (x > 0 && y < max)
        {
            blockers += GetBlocker(x - 1, y + 1, lines);
        }

        // ↙️
        if (x < max && y > 0)
        {
            blockers += GetBlocker(x + 1, y - 1, lines);
        }

        // ↖️
        if (x > 0 && y > 0)
        {
            blockers += GetBlocker(x - 1, y - 1, lines);
        }

        // ⬆️
        if (y > 0)
        {
            blockers += GetBlocker(x, y - 1, lines);
        }

        // ➡️
        if (x < max)
        {
            blockers += GetBlocker(x + 1, y, lines);
        }

        // ⬅️
        if (x > 0)
        {
            blockers += GetBlocker(x - 1, y, lines);
        }

        // ⬇️
        if (y < max)
        {
            blockers += GetBlocker(x, y + 1, lines);
        }

        if (blockers <= 3)
        {
            answer++;
        }
    }
}

Console.WriteLine(answer);

static int GetBlocker(int x, int y, List<string> map)
{
    if (map[x][y] == '@')
    {
        return 1;
    }

    return 0;

}