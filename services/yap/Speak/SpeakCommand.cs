using Yap.Player;
using Yap.Providers;

namespace Yap.Speak;

public sealed class SpeakCommand : Command
{
    private readonly TextToSpeechProviderResolver _providerResolver;
    private readonly IPlayer _player;

    private Argument<string> TextArgument { get; } = new("text")
    {
        Arity = ArgumentArity.ExactlyOne,
        Description = "Text to speak"
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

    private Option<SpeechCodec?> CodecOption { get; } = new("--codec", "-c")
    {
        Description = "Output codec, such as mp3, wav, pcm, mulaw, or alaw"
    };

    private Option<double?> SpeedOption { get; } = new("--speed", "-s")
    {
        Description = "Speech speed override"
    };

    private Option<int?> SampleRateOption { get; } = new("--sample-rate")
    {
        Description = "Output sample rate in Hz when the provider supports it"
    };

    private Option<int?> BitRateOption { get; } = new("--bit-rate")
    {
        Description = "Output bit rate in bits per second when the provider supports it"
    };

    private Option<string?> LanguageOption { get; } = new("--language")
    {
        Description = "Language code when the provider supports it"
    };

    public SpeakCommand(TextToSpeechProviderResolver providerResolver, IPlayer player)
        : base("speak", "Convert text to speech and play immediately")
    {
        _providerResolver = providerResolver;
        _player = player;
        Aliases.Add("say");
        Add(TextArgument);
        Add(ProviderOption);
        Add(VoiceOption);
        Add(ModelOption);
        Add(CodecOption);
        Add(SpeedOption);
        Add(SampleRateOption);
        Add(BitRateOption);
        Add(LanguageOption);
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
            await _player.PlayAsync(result, cancellationToken);

            Console.WriteLine($"[{provider.Name}] played {result.AudioBytes.Length} bytes ({result.ContentType}) on the default audio device");
            return 0;
        }
        catch (Exception exception)
        {
            await Console.Error.WriteLineAsync(exception.Message);
            return 1;
        }
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
