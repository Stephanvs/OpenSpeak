using Yap.Providers;

namespace Yap.Player;

public interface IPlayer
{
    Task PlayAsync(SpeechResult result, CancellationToken cancellationToken = default);
}
