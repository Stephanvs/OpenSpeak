using System.Net.Http.Headers;
using System.Net.Http.Json;

namespace Yap.Providers.ElevenLabs;

public sealed class ElevenLabsProvider(HttpClient httpClient, IOptions<ElevenLabsConfig> config)
    : ITextToSpeechProvider
{
    private const int DefaultMp3SampleRate = 44100;
    private const int DefaultMp3BitRate = 128000;
    private const int DefaultSampleRate = 24000;
    private const int TelephonySampleRate = 8000;

    private readonly ElevenLabsConfig _config = config.Value;

    private sealed record ParsedOutputFormat(SpeechCodec Codec, int? SampleRate, int? BitRate);

    public string Name => "elevenlabs";

    public async Task<SpeechResult> SpeakAsync(TtsSpeechRequest request, CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(_config.ApiKey))
        {
            throw new InvalidOperationException($"Missing {ElevenLabsConfig.SectionName}:ApiKey configuration value.");
        }

        var voiceId = FirstNonEmpty(request.Voice, _config.DefaultVoiceId);
        if (string.IsNullOrWhiteSpace(voiceId))
        {
            throw new InvalidOperationException(
                $"No ElevenLabs voice configured. Set {ElevenLabsConfig.SectionName}:DefaultVoiceId or pass --voice.");
        }

        var outputFormatValue = ResolveOutputFormat(request);

        var payload = new ElevenLabsSpeechPayload
        {
            Text = request.Text,
            ModelId = FirstNonEmpty(request.Model, _config.DefaultModelId),
            LanguageCode = request.LanguageCode,
            VoiceSettings = request.Speed is { } speed
                ? new ElevenLabsVoiceSettings { Speed = speed }
                : null,
        };

        using var message = new HttpRequestMessage(
            HttpMethod.Post,
            $"v1/text-to-speech/{Uri.EscapeDataString(voiceId)}?output_format={Uri.EscapeDataString(outputFormatValue)}");
        message.Headers.Add("xi-api-key", _config.ApiKey);
        message.Headers.Accept.Add(new MediaTypeWithQualityHeaderValue("application/octet-stream"));
        message.Content = JsonContent.Create(payload, JsonContext.Default.ElevenLabsSpeechPayload);

        using var response = await httpClient.SendAsync(message, cancellationToken);
        if (!response.IsSuccessStatusCode)
        {
            var error = await response.Content.ReadAsStringAsync(cancellationToken);
            throw new InvalidOperationException($"ElevenLabs request failed ({(int)response.StatusCode}): {error}");
        }

        var bytes = await response.Content.ReadAsByteArrayAsync(cancellationToken);
        var contentType = response.Content.Headers.ContentType?.MediaType ?? "application/octet-stream";
        return new SpeechResult(bytes, contentType, GuessFileExtension(outputFormatValue, contentType));
    }

    private static string FirstNonEmpty(string? preferred, string fallback) =>
        string.IsNullOrWhiteSpace(preferred) ? fallback : preferred;

    private string ResolveOutputFormat(TtsSpeechRequest request)
    {
        if (request.Output is null)
        {
            return FirstNonEmpty(_config.DefaultOutputFormat, "mp3_44100_128");
        }

        var configuredDefaults = ParseOutputFormatToken(_config.DefaultOutputFormat);
        var codec = request.Output.Codec
            ?? configuredDefaults?.Codec
            ?? SpeechCodec.Mp3;
        var sampleRate = request.Output.SampleRate
            ?? (configuredDefaults?.Codec == codec ? configuredDefaults.SampleRate : null);
        var bitRate = request.Output.BitRate
            ?? (configuredDefaults?.Codec == codec && request.Output.SampleRate is null ? configuredDefaults.BitRate : null);

        return codec switch
        {
            SpeechCodec.Mp3 => ResolveMp3Format(sampleRate ?? DefaultMp3SampleRate, bitRate ?? DefaultMp3BitRate),
            SpeechCodec.Wav => ResolveLinearFormat("wav", sampleRate ?? DefaultSampleRate, bitRate),
            SpeechCodec.Pcm => ResolveLinearFormat("pcm", sampleRate ?? DefaultSampleRate, bitRate),
            SpeechCodec.Mulaw => ResolveTelephonyFormat("ulaw_8000", sampleRate, bitRate),
            SpeechCodec.Alaw => ResolveTelephonyFormat("alaw_8000", sampleRate, bitRate),
            _ => throw new InvalidOperationException(
                $"ElevenLabs does not support codec '{GetCodecName(codec)}' through the generic output settings.")
        };
    }

    private static ParsedOutputFormat? ParseOutputFormatToken(string? rawValue)
    {
        if (string.IsNullOrWhiteSpace(rawValue))
        {
            return null;
        }

        var parts = rawValue.Split('_', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
        if (parts.Length is 0 or > 4)
        {
            return null;
        }

        var codec = ParseCodec(parts[0]);
        if (codec is null)
        {
            return null;
        }

        if (codec is SpeechCodec.Mulaw && parts.Length == 2 && parts[1] == "8000")
        {
            return new ParsedOutputFormat(SpeechCodec.Mulaw, TelephonySampleRate, null);
        }

        if (codec is SpeechCodec.Alaw && parts.Length == 2 && parts[1] == "8000")
        {
            return new ParsedOutputFormat(SpeechCodec.Alaw, TelephonySampleRate, null);
        }

        if (codec is SpeechCodec.Mp3 or SpeechCodec.Pcm or SpeechCodec.Wav)
        {
            int? sampleRate = null;
            if (parts.Length >= 2 && int.TryParse(parts[1], out var parsedSampleRate) && parsedSampleRate > 0)
            {
                sampleRate = parsedSampleRate;
            }

            int? bitRate = null;
            if (parts.Length >= 3 && int.TryParse(parts[2], out var parsedBitRate) && parsedBitRate > 0)
            {
                bitRate = parsedBitRate < 1000 ? parsedBitRate * 1000 : parsedBitRate;
            }

            return new ParsedOutputFormat(codec.Value, sampleRate, bitRate);
        }

        return null;
    }

    private static SpeechCodec? ParseCodec(string rawCodec) => rawCodec.Trim().ToLowerInvariant() switch
    {
        "mp3" => SpeechCodec.Mp3,
        "wav" => SpeechCodec.Wav,
        "pcm" => SpeechCodec.Pcm,
        "ulaw" => SpeechCodec.Mulaw,
        "alaw" => SpeechCodec.Alaw,
        _ => null,
    };

    private static string ResolveMp3Format(int sampleRate, int bitRate) => (sampleRate, bitRate) switch
    {
        (22050, 32000) => "mp3_22050_32",
        (24000, 48000) => "mp3_24000_48",
        (44100, 32000) => "mp3_44100_32",
        (44100, 64000) => "mp3_44100_64",
        (44100, 96000) => "mp3_44100_96",
        (44100, 128000) => "mp3_44100_128",
        (44100, 192000) => "mp3_44100_192",
        _ => throw new InvalidOperationException(
            $"Unsupported ElevenLabs MP3 format {sampleRate} Hz / {bitRate} bps.")
    };

    private static string ResolveLinearFormat(string codec, int sampleRate, int? bitRate)
    {
        if (bitRate is not null)
        {
            throw new InvalidOperationException($"ElevenLabs codec '{codec}' does not support bit rate.");
        }

        return sampleRate switch
        {
            8000 => $"{codec}_8000",
            16000 => $"{codec}_16000",
            22050 => $"{codec}_22050",
            24000 => $"{codec}_24000",
            32000 => $"{codec}_32000",
            44100 => $"{codec}_44100",
            48000 => $"{codec}_48000",
            _ => throw new InvalidOperationException(
                $"Unsupported ElevenLabs {codec} sample rate '{sampleRate}'.")
        };
    }

    private static string ResolveTelephonyFormat(string format, int? sampleRate, int? bitRate)
    {
        if (bitRate is not null)
        {
            throw new InvalidOperationException($"ElevenLabs codec '{format[..format.IndexOf('_')]}' does not support bit rate.");
        }

        var resolvedSampleRate = sampleRate ?? TelephonySampleRate;
        if (resolvedSampleRate != TelephonySampleRate)
        {
            throw new InvalidOperationException($"ElevenLabs telephony codecs require sample rate {TelephonySampleRate}.");
        }

        return format;
    }

    private static string GetCodecName(SpeechCodec codec) => codec.ToString().ToLowerInvariant();

    private static string GuessFileExtension(string outputFormat, string contentType)
    {
        if (outputFormat.StartsWith("mp3", StringComparison.OrdinalIgnoreCase) || contentType.Contains("mpeg", StringComparison.OrdinalIgnoreCase))
        {
            return ".mp3";
        }

        if (outputFormat.StartsWith("wav", StringComparison.OrdinalIgnoreCase) || contentType.Contains("wav", StringComparison.OrdinalIgnoreCase))
        {
            return ".wav";
        }

        if (outputFormat.StartsWith("pcm", StringComparison.OrdinalIgnoreCase))
        {
            return ".pcm";
        }

        if (outputFormat.StartsWith("opus", StringComparison.OrdinalIgnoreCase) || contentType.Contains("opus", StringComparison.OrdinalIgnoreCase))
        {
            return ".opus";
        }

        return ".bin";
    }
}
