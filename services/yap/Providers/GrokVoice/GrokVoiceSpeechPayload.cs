using System.Text.Json.Serialization;

namespace Yap.Providers.GrokVoice;

internal sealed class GrokVoiceSpeechPayload
{
    [JsonPropertyName("model")]
    public required string Model { get; init; }

    [JsonPropertyName("input")]
    public required string Input { get; init; }

    [JsonPropertyName("voice")]
    public required string Voice { get; init; }

    [JsonPropertyName("response_format")]
    public required string ResponseFormat { get; init; }

    [JsonPropertyName("speed")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public double? Speed { get; init; }
}
