using OpenSpeak.Providers;
using OpenSpeak.Providers.ElevenLabs;
using OpenSpeak.Providers.GrokVoice;
using OpenSpeak.Player;
using OpenSpeak.Serve;
using OpenSpeak.Speak;
using OpenSpeak.Synthesize;

namespace OpenSpeak;

internal static class Module
{
    internal static IServiceCollection AddOpenSpeak(this IServiceCollection services)
        => services
            .AddSingleton<TextToSpeechProviderResolver>()
            .AddSingleton<IPlayer, NetCoreAudioPlayer>()
            .AddElevenLabs()
            .AddGrokVoice()
            .AddSingleton<Command, SpeakCommand>()
            .AddSingleton<Command, SynthesizeCommand>()
            .AddSingleton<Command, ServeCommand>();
}
