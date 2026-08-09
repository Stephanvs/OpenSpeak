namespace OpenSpeak.Providers;

public sealed record SpeechOutputSettings
{
    public SpeechCodec? Codec { get; init; }

    public int? SampleRate { get; init; }

    public int? BitRate { get; init; }
}
