using System.Net.Http.Headers;
using System.Net.Http.Json;
using Microsoft.Extensions.Options;

namespace Yap.Providers.GrokVoice;

public sealed class GrokVoiceProvider(
    HttpClient httpClient,
    IOptions<GrokVoiceConfig> config)
    : ITextToSpeechProvider
{
    private readonly GrokVoiceConfig _config = config.Value;

    public string Name => "grok";

    public async Task<SpeechResult> SpeakAsync(
        TtsSpeechRequest request,
        CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(_config.ApiKey))
        {
            throw new InvalidOperationException($"Missing {GrokVoiceConfig.SectionName}:ApiKey configuration value.");
        }

        var payload = new GrokVoiceSpeechPayload
        {
            Input = request.Text,
            Model = FirstNonEmpty(request.Model, _config.DefaultModel),
            Voice = FirstNonEmpty(request.Voice, _config.DefaultVoice),
            ResponseFormat = FirstNonEmpty(request.OutputFormat, _config.DefaultResponseFormat),
            Speed = request.Speed
        };

        using var message = new HttpRequestMessage(HttpMethod.Post, "v1/audio/speech");
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
        return new SpeechResult(bytes, contentType, GuessFileExtension(payload.ResponseFormat, contentType));
    }

    private static string FirstNonEmpty(string? preferred, string fallback) =>
        string.IsNullOrWhiteSpace(preferred) ? fallback : preferred;

    private static string GuessFileExtension(string outputFormat, string contentType)
    {
        if (outputFormat.Equals("mp3", StringComparison.OrdinalIgnoreCase) || contentType.Contains("mpeg", StringComparison.OrdinalIgnoreCase))
        {
            return ".mp3";
        }

        if (outputFormat.Equals("wav", StringComparison.OrdinalIgnoreCase) || contentType.Contains("wav", StringComparison.OrdinalIgnoreCase))
        {
            return ".wav";
        }

        if (outputFormat.Equals("opus", StringComparison.OrdinalIgnoreCase) || contentType.Contains("opus", StringComparison.OrdinalIgnoreCase))
        {
            return ".opus";
        }

        if (outputFormat.Equals("aac", StringComparison.OrdinalIgnoreCase) || contentType.Contains("aac", StringComparison.OrdinalIgnoreCase))
        {
            return ".aac";
        }

        if (outputFormat.Equals("pcm", StringComparison.OrdinalIgnoreCase))
        {
            return ".pcm";
        }

        return ".bin";
    }
}
