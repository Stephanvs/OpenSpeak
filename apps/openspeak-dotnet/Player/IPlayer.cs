using OpenSpeak.Providers;

namespace OpenSpeak.Player;

public interface IPlayer
{
    Task PlayAsync(SpeechResult result, CancellationToken cancellationToken = default);
}
