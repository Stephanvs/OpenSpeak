using System.Text.Json.Serialization;

namespace Yap.Providers.GrokVoice;

internal sealed class GrokVoiceSpeechPayload
{
    [JsonPropertyName("text")]
    public required string Text { get; init; }

    [JsonPropertyName("voice_id")]
    public required GrokVoiceId VoiceId { get; init; }

    [JsonPropertyName("output_format")]
    public required GrokVoiceOutputFormat OutputFormat { get; init; }

    [JsonPropertyName("language")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? Language { get; init; }
}
