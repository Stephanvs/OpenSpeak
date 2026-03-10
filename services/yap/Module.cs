using Yap.Providers;
using Yap.Providers.ElevenLabs;
using Yap.Providers.GrokVoice;
using yap.Serve;
using yap.Speak;
using yap.Synthesize;

namespace Yap;

internal static class Module
{
    internal static IServiceCollection AddYap(this IServiceCollection services)
        => services
            .AddSingleton<TextToSpeechProviderResolver>()
            .AddElevenLabs()
            .AddGrokVoice()
            .AddSingleton<Command, SpeakCommand>()
            .AddSingleton<Command, SynthesizeCommand>()
            .AddSingleton<Command, ServeCommand>();
}
