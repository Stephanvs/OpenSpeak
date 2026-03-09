namespace Yap.Providers;

public sealed record SpeechResult(
    byte[] AudioBytes,
    string ContentType,
    string FileExtension);
