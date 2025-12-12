var lines = File.ReadAllLines("input.txt").ToList();

var shapes = new List<Shape>();
for (int i = 0; i <= 5; i++)
{
    var ix = 1 + i * 5;
    shapes.Add(new Shape(lines[ix..(ix + 3)]));
}

foreach (var shape in shapes)
{
    Console.WriteLine(shape);
}

var answer = 0;
foreach (var line in lines.Skip(30))
{
    var lineSplit = line.Split(":");
    var gridSize = lineSplit[0].Split("x").Select(int.Parse);
    var grid = new bool[gridSize.First(), gridSize.Last()];

    var shapeCounts = lineSplit[1].Trim().Split(" ").Select(int.Parse).ToList();

    if (shapeCounts.Sum() * 9 <= gridSize.First() * gridSize.Last())
    {
        answer++;
        continue;
    }

    var occupiedSize = 0;
    for (int i = 0; i < shapeCounts.Count; i++)
    {
        occupiedSize += shapeCounts[i] * shapes[i].Size;
    }

    if (occupiedSize > gridSize.First() * gridSize.Last())
    {
        continue;
    }
    else
    {
        Console.WriteLine("Valid by occupied size");
    }
}

Console.WriteLine($"Answer: {answer}");

public class Shape
{
    public List<(int x, int y)> ShapePositions = new();

    public int Size;

    public Shape(List<string> input)
    {
        var size = input.Count();

        for (int y = 0; y < size; y++)
        {
            var row = input[y];
            for (int x = 0; x < size; x++)
            {
                if (row[x] == '#')
                {
                    Size++;
                    ShapePositions.Add((x, y));
                }
            }
        }
    }

    public override string ToString()
    {
        var stringBuilder = new System.Text.StringBuilder();
        for (int y = 0; y < 3; y++)
        {
            for (int x = 0; x < 3; x++)
            {
                if (ShapePositions.Contains((x, y)))
                {
                    stringBuilder.Append('#');
                }
                else
                {
                    stringBuilder.Append('.');
                }
            }
            stringBuilder.AppendLine();
        }
        stringBuilder.AppendLine($"Size: {Size}");

        return stringBuilder.ToString();
    }
}