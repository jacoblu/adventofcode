var lines = File.ReadAllLines("input.txt").ToList();

var deviceOutputs = new Dictionary<string, string[]>();

foreach (var line in lines)
{
    var device = line[0..3];

    var outputs = line[5..line.Length].Split(' ');
    deviceOutputs[device] = outputs;
}

var svrToFftPaths = CountPaths("svr", "fft", deviceOutputs, new(), "dac");
var fftToDacPaths = CountPaths("fft", "dac", deviceOutputs, new());
var dacToOutPaths = CountPaths("dac", "out", deviceOutputs, new());
var answer = svrToFftPaths * fftToDacPaths * dacToOutPaths;

Console.WriteLine("answer: " + answer);

static long CountPaths(string start, string end, Dictionary<string, string[]> deviceOutputs, Dictionary<string, long> pathCounts, string trap = "")
{
    if (start == end)
    {
        return 1;
    }

    if (start == trap)
    {
        return 0;
    }

    if (pathCounts.ContainsKey(start))
    {
        return pathCounts[start];
    }

    if (!deviceOutputs.ContainsKey(start))
    {
        return 0;
    }

    long pathCount = 0;
    foreach (var output in deviceOutputs[start])
    {
        pathCount += CountPaths(output, end, deviceOutputs, pathCounts, trap);
    }
    pathCounts[start] = pathCount;
    return pathCount;
}
