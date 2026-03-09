using System.CommandLine;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Yap.Commands.Serve;
using Yap.Commands.Speak;
using Yap.Commands.Synthesize;
using Yap.Providers;
using Yap.Providers.ElevenLabs;
using Yap.Providers.GrokVoice;

namespace Yap;

internal static class Module
{
    internal static IServiceCollection AddYap(this IServiceCollection services, IConfiguration configuration)
    {
        services
            .AddOptions<ElevenLabsConfig>()
            .BindConfiguration(ElevenLabsConfig.SectionName);

        services
            .AddOptions<GrokVoiceConfig>()
            .BindConfiguration(GrokVoiceConfig.SectionName);

        services.AddHttpClient<ElevenLabsProvider>((serviceProvider, httpClient) =>
        {
            var config = serviceProvider.GetRequiredService<Microsoft.Extensions.Options.IOptions<ElevenLabsConfig>>().Value;
            httpClient.BaseAddress = CreateBaseAddress(config.ApiBaseUrl, "https://api.elevenlabs.io");
        });

        services.AddHttpClient<GrokVoiceProvider>((serviceProvider, httpClient) =>
        {
            var config = serviceProvider.GetRequiredService<Microsoft.Extensions.Options.IOptions<GrokVoiceConfig>>().Value;
            httpClient.BaseAddress = CreateBaseAddress(config.ApiBaseUrl, "https://api.x.ai");
        });

        return services
            .AddSingleton<TextToSpeechProviderResolver>()
            .AddTransient<ITextToSpeechProvider>(serviceProvider => serviceProvider.GetRequiredService<ElevenLabsProvider>())
            .AddTransient<ITextToSpeechProvider>(serviceProvider => serviceProvider.GetRequiredService<GrokVoiceProvider>())
            .AddSingleton<Command, SpeakCommand>()
            .AddSingleton<Command, SynthesizeCommand>()
            .AddSingleton<Command, ServeCommand>();

        static Uri CreateBaseAddress(string baseUrl, string fallbackBaseUrl)
        {
            var normalized = string.IsNullOrWhiteSpace(baseUrl) ? fallbackBaseUrl : baseUrl.TrimEnd('/');
            return new Uri($"{normalized}/", UriKind.Absolute);
        }
    }
}
