namespace Yap.Providers.ElevenLabs;

public sealed class ElevenLabsConfig
{
    public const string SectionName = "elevenlabs";

    public string ApiKey { get; set; } = string.Empty;

    public string ApiBaseUrl { get; set; } = "https://api.elevenlabs.io";

    public string DefaultVoiceId { get; set; } = string.Empty;

    public string DefaultModelId { get; set; } = "eleven_multilingual_v2";

    public string DefaultOutputFormat { get; set; } = "mp3_44100_128";
}
