using System.Text.Json.Serialization;

namespace OpenSpeak.Providers;

[JsonConverter(typeof(JsonStringEnumConverter<SpeechCodec>))]
public enum SpeechCodec
{
    [JsonStringEnumMemberName("mp3")]
    Mp3,

    [JsonStringEnumMemberName("wav")]
    Wav,

    [JsonStringEnumMemberName("pcm")]
    Pcm,

    [JsonStringEnumMemberName("mulaw")]
    Mulaw,

    [JsonStringEnumMemberName("alaw")]
    Alaw,
}
