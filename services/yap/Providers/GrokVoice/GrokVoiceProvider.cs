using System.Net.Http.Headers;
using System.Net.Http.Json;
using Microsoft.Extensions.Options;

namespace Yap.Providers.GrokVoice;

public sealed class GrokVoiceProvider(
    HttpClient httpClient,
    IOptions<GrokVoiceConfig> config)
    : ITextToSpeechProvider
{
    private const GrokVoiceId FallbackVoiceId = GrokVoiceId.Eve;
    private const int FallbackSampleRate = 24000;
    private const int FallbackBitRate = 128000;
    private static readonly string AllowedVoiceIds = string.Join(", ", Enum.GetNames<GrokVoiceId>()).ToLowerInvariant();

    private readonly GrokVoiceConfig _config = config.Value;

    private sealed record ParsedOutputFormat(SpeechCodec Codec, int? SampleRate, int? BitRate);

    public string Name => "grok";

    public async Task<SpeechResult> SpeakAsync(
        TtsSpeechRequest request,
        CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(_config.ApiKey))
        {
            throw new InvalidOperationException($"Missing {GrokVoiceConfig.SectionName}:ApiKey configuration value.");
        }

        var voiceId = ResolveVoiceId(request.Voice, _config.DefaultVoiceId);

        var outputFormat = ResolveOutputFormat(request);
        var payload = new GrokVoiceSpeechPayload
        {
            Text = request.Text,
            VoiceId = voiceId,
            OutputFormat = outputFormat,
            Language = FirstNonEmpty(request.LanguageCode, _config.DefaultLanguage)
        };

        using var message = new HttpRequestMessage(HttpMethod.Post, "v1/tts");
        message.Headers.Authorization = new AuthenticationHeaderValue("Bearer", _config.ApiKey);
        message.Headers.Accept.Add(new MediaTypeWithQualityHeaderValue("application/octet-stream"));
        message.Content = JsonContent.Create(payload, JsonContext.Default.GrokVoiceSpeechPayload);

        using var response = await httpClient.SendAsync(message, cancellationToken);
        if (!response.IsSuccessStatusCode)
        {
            var error = await response.Content.ReadAsStringAsync(cancellationToken);
            throw new InvalidOperationException($"Grok Voice request failed ({(int)response.StatusCode}): {error}");
        }

        var bytes = await response.Content.ReadAsByteArrayAsync(cancellationToken);
        var contentType = response.Content.Headers.ContentType?.MediaType ?? "application/octet-stream";
        return new SpeechResult(bytes, contentType, GuessFileExtension(outputFormat.Codec, contentType));
    }

    private GrokVoiceOutputFormat ResolveOutputFormat(TtsSpeechRequest request)
    {
        var requestedOutput = request.Output;
        var configuredDefaults = ParseOutputFormatToken(_config.DefaultOutputFormat);

        var codec = requestedOutput?.Codec
            ?? configuredDefaults?.Codec
            ?? SpeechCodec.Mp3;
        var sampleRate = requestedOutput?.SampleRate
            ?? (configuredDefaults?.Codec == codec ? configuredDefaults.SampleRate : null)
            ?? _config.DefaultSampleRate
            ?? FallbackSampleRate;

        var bitRate = requestedOutput?.BitRate
            ?? (configuredDefaults?.Codec == codec && requestedOutput?.SampleRate is null ? configuredDefaults.BitRate : null)
            ?? _config.DefaultBitRate;

        if (sampleRate <= 0)
        {
            throw new InvalidOperationException("Sample rate must be a positive integer.");
        }

        if (bitRate <= 0)
        {
            throw new InvalidOperationException("Bit rate must be a positive integer when provided.");
        }

        if (bitRate is not null && !SupportsBitRate(codec))
        {
            throw new InvalidOperationException($"Grok codec '{GetCodecName(codec)}' does not support bit rate.");
        }

        if (bitRate is null && SupportsBitRate(codec))
        {
            bitRate = FallbackBitRate;
        }

        return new GrokVoiceOutputFormat
        {
            Codec = codec,
            SampleRate = sampleRate,
            BitRate = SupportsBitRate(codec) ? bitRate : null
        };
    }

    private static GrokVoiceId ResolveVoiceId(params string?[] values)
    {
        var rawValue = FirstNonEmpty(values);
        if (string.IsNullOrWhiteSpace(rawValue))
        {
            return FallbackVoiceId;
        }

        var normalized = rawValue.Trim();
        if (!int.TryParse(normalized, out _) && Enum.TryParse<GrokVoiceId>(normalized, true, out var voiceId))
        {
            return voiceId;
        }

        throw new InvalidOperationException(
            $"Unsupported Grok voice '{rawValue}'. Allowed voices: {AllowedVoiceIds}.");
    }

    private static ParsedOutputFormat? ParseOutputFormatToken(string? rawValue)
    {
        if (string.IsNullOrWhiteSpace(rawValue))
        {
            return null;
        }

        var parts = rawValue.Split('_', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
        if (parts.Length is 0 or > 3)
        {
            throw new InvalidOperationException(
                $"Unsupported Grok output format '{rawValue}'. Expected codec, codec_sampleRate, or codec_sampleRate_bitRate.");
        }

        int? sampleRate = null;
        if (parts.Length >= 2)
        {
            sampleRate = ParsePositiveInt(parts[1], "sample rate", rawValue);
        }

        int? bitRate = null;
        if (parts.Length == 3)
        {
            bitRate = NormalizeBitRate(ParsePositiveInt(parts[2], "bit rate", rawValue));
        }

        return new ParsedOutputFormat(ParseCodec(parts[0], rawValue), sampleRate, bitRate);
    }

    private static int ParsePositiveInt(string rawValue, string label, string originalValue)
    {
        if (!int.TryParse(rawValue, out var value) || value <= 0)
        {
            throw new InvalidOperationException($"Invalid {label} in Grok output format '{originalValue}'.");
        }

        return value;
    }

    private static int NormalizeBitRate(int bitRate) => bitRate < 1000 ? bitRate * 1000 : bitRate;

    private static SpeechCodec ParseCodec(string rawCodec, string originalValue)
    {
        if (string.IsNullOrWhiteSpace(rawCodec))
        {
            throw new InvalidOperationException($"Missing codec in Grok output format '{originalValue}'.");
        }

        return rawCodec.Trim().ToLowerInvariant() switch
        {
            "mp3" => SpeechCodec.Mp3,
            "wav" => SpeechCodec.Wav,
            "pcm" => SpeechCodec.Pcm,
            "mulaw" or "ulaw" => SpeechCodec.Mulaw,
            "alaw" => SpeechCodec.Alaw,
            _ => throw new InvalidOperationException(
                $"Unsupported Grok codec '{rawCodec}' in output format '{originalValue}'. Allowed codecs: mp3, wav, pcm, mulaw, alaw.")
        };
    }

    private static bool SupportsBitRate(SpeechCodec codec) => codec is SpeechCodec.Mp3;

    private static string GetCodecName(SpeechCodec codec) => codec.ToString().ToLowerInvariant();

    private static string? FirstNonEmpty(params string?[] values)
    {
        foreach (var value in values)
        {
            if (!string.IsNullOrWhiteSpace(value))
            {
                return value;
            }
        }

        return null;
    }

    private static string GuessFileExtension(SpeechCodec outputFormat, string contentType)
    {
        if (outputFormat is SpeechCodec.Mp3 || contentType.Contains("mpeg", StringComparison.OrdinalIgnoreCase))
        {
            return ".mp3";
        }

        if (outputFormat is SpeechCodec.Wav || contentType.Contains("wav", StringComparison.OrdinalIgnoreCase))
        {
            return ".wav";
        }

        if (outputFormat is SpeechCodec.Pcm)
        {
            return ".pcm";
        }

        if (outputFormat is SpeechCodec.Mulaw || contentType.Contains("basic", StringComparison.OrdinalIgnoreCase))
        {
            return ".mulaw";
        }

        if (outputFormat is SpeechCodec.Alaw || contentType.Contains("alaw", StringComparison.OrdinalIgnoreCase))
        {
            return ".alaw";
        }

        return ".bin";
    }
}
