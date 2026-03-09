namespace Yap.Providers.GrokVoice;

public sealed class GrokVoiceConfig
{
    public const string SectionName = "grok";

    public string ApiKey { get; set; } = string.Empty;

    public string ApiBaseUrl { get; set; } = "https://api.x.ai";

    public string DefaultVoice { get; set; } = "alloy";

    public string DefaultModel { get; set; } = "tts-1";

    public string DefaultResponseFormat { get; set; } = "mp3";
}
