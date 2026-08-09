using OpenSpeak.Providers;

namespace OpenSpeak.Player;

public sealed class NetCoreAudioPlayer : IPlayer
{
    public async Task PlayAsync(SpeechResult result, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(result);

        cancellationToken.ThrowIfCancellationRequested();

        var tempFilePath = Path.Combine(
            Path.GetTempPath(),
            $"openspeak-play-{Guid.NewGuid():N}{NormalizeFileExtension(result.FileExtension)}");

        await File.WriteAllBytesAsync(tempFilePath, result.AudioBytes, cancellationToken);

        var player = new NetCoreAudio.Player();
        var playbackFinished = new TaskCompletionSource<bool>(TaskCreationOptions.RunContinuationsAsynchronously);
        EventHandler playbackFinishedHandler = (_, _) => playbackFinished.TrySetResult(true);
        player.PlaybackFinished += playbackFinishedHandler;

        await using var registration = cancellationToken.Register(() =>
        {
            _ = player.Stop();
            playbackFinished.TrySetCanceled(cancellationToken);
        });

        try
        {
            await player.Play(tempFilePath);

            if (!player.Playing && !playbackFinished.Task.IsCompleted)
            {
                throw new InvalidOperationException(
                    $"NetCoreAudio could not start playback for content type '{result.ContentType}' and extension '{result.FileExtension}'.");
            }

            await playbackFinished.Task;
        }
        finally
        {
            player.PlaybackFinished -= playbackFinishedHandler;

            try
            {
                await player.Stop();
            }
            catch
            {
                // ignored
            }

            TryDeleteFile(tempFilePath);
        }
    }

    private static string NormalizeFileExtension(string? fileExtension)
    {
        if (string.IsNullOrWhiteSpace(fileExtension))
        {
            return ".bin";
        }

        return fileExtension.StartsWith('.') ? fileExtension : $".{fileExtension}";
    }

    private static void TryDeleteFile(string filePath)
    {
        try
        {
            if (File.Exists(filePath))
            {
                File.Delete(filePath);
            }
        }
        catch
        {
            // ignored
        }
    }
}
