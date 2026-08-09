namespace OpenSpeak.Providers.GrokVoice;

public sealed class GrokVoiceConfig
{
    public const string SectionName = "grok";

    public string ApiKey { get; set; } = string.Empty;

    public string ApiBaseUrl { get; set; } = "https://api.x.ai";

    public string DefaultVoiceId { get; set; } = "eve";

    public string DefaultModel { get; set; } = string.Empty;

    public string DefaultOutputFormat { get; set; } = "mp3";

    public int? DefaultSampleRate { get; set; } = 24000;

    public int? DefaultBitRate { get; set; } = 128000;

    public string? DefaultLanguage { get; set; }
}
