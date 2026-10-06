using System;
using System.IO;
using Sz.ConfigTool;

// Usage: dotnet run -- <g2config.json> [DATA_SOURCE_CODE]
// Adds a data source to the given configuration, prints the data sources,
// and shows how a library error surfaces.
if (args.Length < 1)
{
    Console.Error.WriteLine("usage: Sz.ConfigTool.Example <g2config.json> [DATA_SOURCE_CODE]");
    return 2;
}

string config = File.ReadAllText(args[0]);
string code = args.Length > 1 ? args[1] : "CUSTOMERS";

Console.WriteLine($"libSzConfigTool {SzConfigTool.LibraryVersion}");

string updated = SzConfigTool.AddDataSource(config, code, retentionLevel: "Remember");
Console.WriteLine(SzConfigTool.ListDataSources(updated));

try
{
    SzConfigTool.AddDataSource(updated, code);
}
catch (SzConfigToolException e) when (e.Kind == SzConfigToolErrorKind.AlreadyExists)
{
    Console.WriteLine($"expected error: {e.ReasonCode}: {e.Message}");
}

return 0;
