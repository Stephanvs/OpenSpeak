using System.Net.Http.Headers;
using System.Net.Http.Json;
using Microsoft.Extensions.Options;

namespace Yap.Providers.ElevenLabs;

public sealed class ElevenLabsProvider(HttpClient httpClient, IOptions<ElevenLabsConfig> config) : ITextToSpeechProvider
{
    private readonly ElevenLabsConfig _config = config.Value;

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

        var outputFormat = FirstNonEmpty(request.OutputFormat, _config.DefaultOutputFormat);
        var payload = new ElevenLabsSpeechPayload
        {
            Text = request.Text,
            ModelId = FirstNonEmpty(request.Model, _config.DefaultModelId),
            LanguageCode = request.LanguageCode,
            VoiceSettings = request.Speed is double speed
                ? new ElevenLabsVoiceSettings { Speed = speed }
                : null
        };

        using var message = new HttpRequestMessage(
            HttpMethod.Post,
            $"v1/text-to-speech/{Uri.EscapeDataString(voiceId)}?output_format={Uri.EscapeDataString(outputFormat)}");
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
        return new SpeechResult(bytes, contentType, GuessFileExtension(outputFormat, contentType));
    }

    private static string FirstNonEmpty(string? preferred, string fallback) =>
        string.IsNullOrWhiteSpace(preferred) ? fallback : preferred;

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
