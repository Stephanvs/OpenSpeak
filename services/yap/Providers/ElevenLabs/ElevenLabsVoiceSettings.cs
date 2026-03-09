using System.Text.Json.Serialization;

namespace Yap.Providers.ElevenLabs;

internal sealed class ElevenLabsVoiceSettings
{
    [JsonPropertyName("speed")]
    public double Speed { get; init; }
}
