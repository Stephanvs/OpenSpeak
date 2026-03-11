using Yap.Providers;
using Yap.Providers.ElevenLabs;
using Yap.Providers.GrokVoice;
using Yap.Player;
using Yap.Serve;
using Yap.Speak;
using Yap.Synthesize;

namespace Yap;

internal static class Module
{
    internal static IServiceCollection AddYap(this IServiceCollection services)
        => services
            .AddSingleton<TextToSpeechProviderResolver>()
            .AddSingleton<IPlayer, NetCoreAudioPlayer>()
            .AddElevenLabs()
            .AddGrokVoice()
            .AddSingleton<Command, SpeakCommand>()
            .AddSingleton<Command, SynthesizeCommand>()
            .AddSingleton<Command, ServeCommand>();
}
