namespace Yap.Providers;

public sealed class TextToSpeechProviderResolver(IEnumerable<ITextToSpeechProvider> providers, IConfiguration config)
{
    private readonly IReadOnlyDictionary<string, ITextToSpeechProvider> _providers
        = providers.ToDictionary(provider => provider.Name, StringComparer.OrdinalIgnoreCase);

    public IReadOnlyCollection<string> ProviderNames => [.. _providers.Keys.OrderBy(name => name)];

    public ITextToSpeechProvider Resolve(string? providerName)
    {
        var requestedName = string.IsNullOrWhiteSpace(providerName)
            ? config.GetValue<string?>("defaultProvider", "elevenlabs")
            : providerName;

        if (_providers.TryGetValue(requestedName, out var provider))
        {
            return provider;
        }

        throw new InvalidOperationException(
            $"Unknown TTS provider '{requestedName}'. Available providers: {string.Join(", ", ProviderNames)}.");
    }
}
