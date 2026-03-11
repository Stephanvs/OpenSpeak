namespace Yap.Providers;

public sealed record TtsSpeechRequest
{
    public required string Text { get; init; }

    public string? Voice { get; init; }

    public string? Model { get; init; }

    public SpeechOutputSettings? Output { get; init; }

    public double? Speed { get; init; }

    public string? LanguageCode { get; init; }
}
