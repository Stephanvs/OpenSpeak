using System.Text.Json.Serialization;

namespace OpenSpeak.Providers.ElevenLabs;

internal sealed class ElevenLabsSpeechPayload
{
    [JsonPropertyName("text")]
    public required string Text { get; init; }

    [JsonPropertyName("model_id")]
    public required string ModelId { get; init; }

    [JsonPropertyName("language_code")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? LanguageCode { get; init; }

    [JsonPropertyName("voice_settings")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public ElevenLabsVoiceSettings? VoiceSettings { get; init; }
}
