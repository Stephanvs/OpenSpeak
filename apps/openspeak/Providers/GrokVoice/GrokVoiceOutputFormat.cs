using System.Text.Json.Serialization;

namespace OpenSpeak.Providers.GrokVoice;

internal sealed class GrokVoiceOutputFormat
{
    [JsonPropertyName("codec")]
    public required SpeechCodec Codec { get; init; }

    [JsonPropertyName("sample_rate")]
    public required int SampleRate { get; init; }

    [JsonPropertyName("bit_rate")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public int? BitRate { get; init; }
}
