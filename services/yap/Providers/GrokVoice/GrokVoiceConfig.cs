namespace Yap.Providers.GrokVoice;

public sealed class GrokVoiceConfig
{
    public const string SectionName = "grok";

    public string ApiKey { get; set; } = string.Empty;

    public string ApiBaseUrl { get; set; } = "https://api.x.ai";

    public string DefaultVoiceId { get; set; } = string.Empty;

    public string DefaultVoice { get; set; } = string.Empty;

    public string DefaultModel { get; set; } = string.Empty;

    public string DefaultOutputFormat { get; set; } = string.Empty;

    public string DefaultResponseFormat { get; set; } = string.Empty;

    public int? DefaultSampleRate { get; set; }

    public int? DefaultBitRate { get; set; }

    public string? DefaultLanguage { get; set; }
}
