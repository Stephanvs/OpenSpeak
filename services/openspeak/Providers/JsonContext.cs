using System.Text.Json.Serialization;
using OpenSpeak.Providers.ElevenLabs;
using OpenSpeak.Providers.GrokVoice;

namespace OpenSpeak.Providers;

[JsonSerializable(typeof(ElevenLabsSpeechPayload))]
[JsonSerializable(typeof(GrokVoiceOutputFormat))]
[JsonSerializable(typeof(GrokVoiceSpeechPayload))]
internal sealed partial class JsonContext : JsonSerializerContext;
