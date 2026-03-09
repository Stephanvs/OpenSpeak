namespace Yap.Providers;

public interface ITextToSpeechProvider
{
    string Name { get; }

    Task<SpeechResult> SpeakAsync(
        TtsSpeechRequest request,
        CancellationToken cancellationToken = default);
}
