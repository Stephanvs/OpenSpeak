namespace Yap.Providers.GrokVoice;

public static class Module
{
  public static IServiceCollection AddGrokVoice(this IServiceCollection services)
    => services
      .AddOptions<GrokVoiceConfig>()
      .BindConfiguration(GrokVoiceConfig.SectionName)
      .Services
      .AddTransient<ITextToSpeechProvider>(serviceProvider => serviceProvider.GetRequiredService<GrokVoiceProvider>())
      .AddHttpClient<GrokVoiceProvider>((serviceProvider, httpClient) =>
      {
        var config = serviceProvider.GetRequiredService<IOptions<GrokVoiceConfig>>().Value;
        httpClient.BaseAddress = CreateBaseAddress(config.ApiBaseUrl, "https://api.x.ai");
      })
      .Services;

  private static Uri CreateBaseAddress(string baseUrl, string fallbackBaseUrl)
  {
      var normalized = string.IsNullOrWhiteSpace(baseUrl) ? fallbackBaseUrl : baseUrl.TrimEnd('/');
      return new Uri($"{normalized}/", UriKind.Absolute);
  }
}
