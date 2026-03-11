using System.Text.Json.Serialization;

namespace Yap.Providers.GrokVoice;

[JsonConverter(typeof(JsonStringEnumConverter<GrokVoiceId>))]
internal enum GrokVoiceId
{
    [JsonStringEnumMemberName("leo")]
    Leo,

    [JsonStringEnumMemberName("ara")]
    Ara,

    [JsonStringEnumMemberName("eve")]
    Eve,

    [JsonStringEnumMemberName("una")]
    Una,

    [JsonStringEnumMemberName("sal")]
    Sal,

    [JsonStringEnumMemberName("rex")]
    Rex,
}
