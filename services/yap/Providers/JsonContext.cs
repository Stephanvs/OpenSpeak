using System.Text.Json.Serialization;
using Yap.Providers.ElevenLabs;
using Yap.Providers.GrokVoice;

namespace Yap.Providers;

[JsonSerializable(typeof(ElevenLabsSpeechPayload))]
[JsonSerializable(typeof(GrokVoiceSpeechPayload))]
internal sealed partial class JsonContext : JsonSerializerContext;
