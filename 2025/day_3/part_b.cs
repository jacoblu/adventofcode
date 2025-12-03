long answer = 0;

var instructions = File.ReadAllLines("input.txt")
    .ToList();

foreach (var line in instructions)
{
    var joltagePositions = new Dictionary<int, List<int>>();
    joltagePositions.Add(1, new List<int>());
    joltagePositions.Add(2, new List<int>());
    joltagePositions.Add(3, new List<int>());
    joltagePositions.Add(4, new List<int>());
    joltagePositions.Add(5, new List<int>());
    joltagePositions.Add(6, new List<int>());
    joltagePositions.Add(7, new List<int>());
    joltagePositions.Add(8, new List<int>());
    joltagePositions.Add(9, new List<int>());

    var min = -1;
    var max = line.Length;

    for (int i = 0; i < line.Length; i++)
    {
        var joltage = line[i] - '0';
        joltagePositions[joltage].Add(i);
    }

    long bank = 0;
    for (int i = 11; i >= 0; i--)
    {
        var isBatteryFound = false;
        for (int j = 9; j > 0; j--)
        {
            foreach (var pos in joltagePositions[j])
            {
                if (pos > min && (pos + i) < max)
                {
                    min = pos;
                    isBatteryFound = true;


                    bank += j * ((long)Math.Pow(10, i));
                    break;
                }
            }

            if (isBatteryFound)
            {
                break;
            }
        }
    }

    answer += bank;
}

Console.WriteLine(answer);