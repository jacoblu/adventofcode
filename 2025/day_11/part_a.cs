var lines = File.ReadAllLines("input.txt").ToList();

var deviceOutputs = new Dictionary<string, string[]>();

foreach (var line in lines)
{
    var device = line[0..3];

    var outputs = line[5..line.Length].Split(' ');
    deviceOutputs[device] = outputs;
}

var paths = new List<string[]>();
paths.Add(new string[] { "you" });

var answer = 0;

while (paths.Any())
{
    var newPaths = new List<string[]>();

    foreach (var path in paths)
    {
        var last = path.Last();
        var outputs = deviceOutputs[last];

        foreach (var output in outputs)
        {
            if (output == "out")
            {
                answer++;
            }
            else
            {
                var newPath = new string[path.Count() + 1];
                path.CopyTo(newPath);
                newPath[newPath.Length - 1] = output;
                newPaths.Add(newPath);
            }
        }
    }

    paths = newPaths;
}

Console.WriteLine("answer: " + answer);