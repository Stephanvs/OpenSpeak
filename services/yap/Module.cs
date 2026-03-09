using System.CommandLine;
using Microsoft.Extensions.DependencyInjection;
using Yap.Commands.Serve;
using Yap.Commands.Speak;
using Yap.Commands.Synthesize;

namespace Yap;

internal static class Module
{
    internal static IServiceCollection AddYapCommands(this IServiceCollection services) => services
        .AddSingleton<Command, SpeakCommand>()
        .AddSingleton<Command, SynthesizeCommand>()
        .AddSingleton<Command, ServeCommand>();
}
