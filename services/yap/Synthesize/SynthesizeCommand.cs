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

    private Option<SpeechCodec> CodecOption { get; } = new("--codec", "-c")
    {
        Description = "Output codec, such as mp3, wav, pcm, mulaw, or alaw",
        DefaultValueFactory = (_) => SpeechCodec.Mp3
    };

    private Option<double?> SpeedOption { get; } = new("--speed", "-s")
    {
        Description = "Speech speed override"
    };

    private Option<int?> SampleRateOption { get; } = new("--sample-rate")
    {
        Description = "Output sample rate in Hz when the provider supports it",
        DefaultValueFactory = (_) => 24000
    };

    private Option<int?> BitRateOption { get; } = new("--bit-rate")
    {
        Description = "Output bit rate in bits per second when the provider supports it",
        DefaultValueFactory = (_) => 192000
    };

    private Option<string?> LanguageOption { get; } = new("--language")
    {
        Description = "Language code when the provider supports it"
    };

    private Option<DirectoryInfo?> OutputOption { get; } = new("--output", "-o")
    {
        Description = "Directory where to save the generated audio file"
    };

    public SynthesizeCommand(TextToSpeechProviderResolver providerResolver)
        : base("synthesize", "Generate audio file from text")
    {
        _providerResolver = providerResolver;
        Add(TextArgument);
        Add(ProviderOption);
        Add(VoiceOption);
        Add(ModelOption);
        Add(CodecOption);
        Add(SpeedOption);
        Add(SampleRateOption);
        Add(BitRateOption);
        Add(LanguageOption);
        Add(OutputOption);
        SetAction(ExecuteAsync);
    }

    private async Task<int> ExecuteAsync(ParseResult parseResult, CancellationToken cancellationToken)
    {
        var text = parseResult.GetValue(TextArgument);

        if (string.IsNullOrWhiteSpace(text))
        {
            await Console.Error.WriteLineAsync("No text supplied for synthesize.");
            return 1;
        }

        try
        {
            var providerName = parseResult.GetValue(ProviderOption);
            var provider = _providerResolver.Resolve(providerName);
            var codec = parseResult.GetValue(CodecOption);
            var sampleRate = parseResult.GetValue(SampleRateOption);
            var bitRate = parseResult.GetValue(BitRateOption);
            var request = new TtsSpeechRequest
            {
                Text = text,
                Voice = parseResult.GetValue(VoiceOption),
                Model = parseResult.GetValue(ModelOption),
                Output = CreateOutputSettings(codec, sampleRate, bitRate),
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

    private static string ResolveOutputPath(DirectoryInfo? configuredPath, string fileExtension)
    {
        var fileName = $"yap-{DateTimeOffset.UtcNow:yyyyMMddHHmmss}-{Guid.NewGuid():N}{fileExtension}";

        return Path.Combine(configuredPath is not null
            ? configuredPath.FullName
            : Environment.CurrentDirectory, fileName);
    }

    private static SpeechOutputSettings? CreateOutputSettings(
        SpeechCodec? codec,
        int? sampleRate,
        int? bitRate)
    {
        if (codec is null && sampleRate is null && bitRate is null)
        {
            return null;
        }

        return new SpeechOutputSettings
        {
            Codec = codec,
            SampleRate = sampleRate,
            BitRate = bitRate,
        };
    }
}
