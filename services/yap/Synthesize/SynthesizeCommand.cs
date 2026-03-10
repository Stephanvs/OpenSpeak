using Yap.Providers;

namespace Yap.Synthesize;

public sealed class SynthesizeCommand : Command
{
    private readonly TextToSpeechProviderResolver _providerResolver;

    private Argument<string> TextArgument { get; } = new("text")
    {
        Arity = ArgumentArity.ExactlyOne,
        Description = "Text to synthesize"
    };

    private Option<string?> ProviderOption { get; } = new("--provider", "-p")
    {
        Description = "TTS provider to use, such as elevenlabs or grok"
    };

    private Option<string?> VoiceOption { get; } = new("--voice", "-v")
    {
        Description = "Provider-specific voice or voice ID"
    };

    private Option<string?> ModelOption { get; } = new("--model", "-m")
    {
        Description = "Provider-specific model override"
    };

    private Option<string?> FormatOption { get; } = new("--format", "-f")
    {
        Description = "Output audio format, for example mp3 or wav"
    };

    private Option<double?> SpeedOption { get; } = new("--speed", "-s")
    {
        Description = "Speech speed override"
    };

    private Option<string?> LanguageOption { get; } = new("--language")
    {
        Description = "Language code when the provider supports it"
    };

    private Option<string?> OutputOption { get; } = new("--output", "-o")
    {
        Description = "Where to save the generated audio file"
    };

    public SynthesizeCommand(TextToSpeechProviderResolver providerResolver)
        : base("synthesize", "Generate audio file from text")
    {
        _providerResolver = providerResolver;
        Add(TextArgument);
        Add(ProviderOption);
        Add(VoiceOption);
        Add(ModelOption);
        Add(FormatOption);
        Add(SpeedOption);
        Add(LanguageOption);
        Add(OutputOption);
        SetAction(ExecuteAsync);
    }

    private async Task<int> ExecuteAsync(ParseResult parseResult, CancellationToken cancellationToken)
    {
        var text = parseResult.GetValue(TextArgument);

        if (string.IsNullOrWhiteSpace(text))
        {
            await Console.Error.WriteLineAsync("No text supplied for speak.");
            return 1;
        }

        try
        {
            var providerName = parseResult.GetValue(ProviderOption);
            var provider = _providerResolver.Resolve(providerName);
            var request = new TtsSpeechRequest
            {
                Text = text,
                Voice = parseResult.GetValue(VoiceOption),
                Model = parseResult.GetValue(ModelOption),
                OutputFormat = parseResult.GetValue(FormatOption),
                Speed = parseResult.GetValue(SpeedOption),
                LanguageCode = parseResult.GetValue(LanguageOption)
            };

            var result = await provider.SpeakAsync(request, cancellationToken);
            var outputPath = ResolveOutputPath(parseResult.GetValue(OutputOption), result.FileExtension);
            await File.WriteAllBytesAsync(outputPath, result.AudioBytes, cancellationToken);

            Console.WriteLine($"[{provider.Name}] wrote {result.AudioBytes.Length} bytes to {outputPath}");
            return 0;
        }
        catch (Exception exception)
        {
            await Console.Error.WriteLineAsync(exception.Message);
            return 1;
        }
    }

    private static string ResolveOutputPath(string? configuredPath, string fileExtension)
    {
        if (!string.IsNullOrWhiteSpace(configuredPath))
        {
            return configuredPath;
        }

        var fileName = $"yap-{DateTimeOffset.UtcNow:yyyyMMddHHmmss}-{Guid.NewGuid():N}{fileExtension}";
        return Path.Combine(Environment.CurrentDirectory, fileName);
    }
}
