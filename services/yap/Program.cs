using System.CommandLine;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Yap;

var builder = Host.CreateApplicationBuilder(args);

builder.Configuration.AddEnvironmentVariables();
builder.Services.AddYap(builder.Configuration);

var host = builder.Build();
var rootCommand = new RootCommand("yap - text-to-speech");

var commands = host.Services.GetRequiredService<IEnumerable<Command>>();
foreach (var command in commands)
{
    rootCommand.Subcommands.Add(command);
}

return await rootCommand.Parse(args).InvokeAsync();
