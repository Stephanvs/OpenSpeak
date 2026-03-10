namespace Yap.Providers.ElevenLabs;

public static class Module 
{
  public static IServiceCollection AddElevenLabs(this IServiceCollection services)
    => services
      .AddOptions<ElevenLabsConfig>()
      .BindConfiguration(ElevenLabsConfig.SectionName)
      .Services
      .AddTransient<ITextToSpeechProvider>(serviceProvider => serviceProvider.GetRequiredService<ElevenLabsProvider>())
      .AddHttpClient<ElevenLabsProvider>((serviceProvider, httpClient) =>
      {
        var config = serviceProvider.GetRequiredService<IOptions<ElevenLabsConfig>>().Value;
        httpClient.BaseAddress = CreateBaseAddress(config.ApiBaseUrl, "https://api.elevenlabs.io");
      })
      .Services;

  private static Uri CreateBaseAddress(string baseUrl, string fallbackBaseUrl)
  {
      var normalized = string.IsNullOrWhiteSpace(baseUrl) ? fallbackBaseUrl : baseUrl.TrimEnd('/');
      return new Uri($"{normalized}/", UriKind.Absolute);
  }
}
