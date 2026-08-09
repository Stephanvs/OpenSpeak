using Microsoft.Extensions.Hosting;
using OpenSpeak;

var builder = Host.CreateApplicationBuilder(args);

builder.Configuration.AddEnvironmentVariables();
builder.Services.AddOpenSpeak();

var host = builder.Build();
var rootCommand = new RootCommand("openspeak - text-to-speech");

var commands = host.Services.GetRequiredService<IEnumerable<Command>>();
foreach (var command in commands)
{
    rootCommand.Subcommands.Add(command);
}

return await rootCommand.Parse(args).InvokeAsync();
