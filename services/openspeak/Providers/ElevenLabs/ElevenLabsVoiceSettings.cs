using System.Text.Json.Serialization;

namespace OpenSpeak.Providers.ElevenLabs;

internal sealed class ElevenLabsVoiceSettings
{
    [JsonPropertyName("speed")]
    public double Speed { get; init; }
}
