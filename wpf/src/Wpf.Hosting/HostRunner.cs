namespace Wpf.Hosting;

public sealed class HostRunner : IDisposable
{
    private readonly CancellationTokenSource _runHostCancellation = new();
    private volatile ManualResetEventSlim? _runHostCompletedEvent;

    public IHostBuilder Builder { get; } = new HostBuilder();

    public async void Run()
    {
        if (Interlocked.CompareExchange(ref _runHostCompletedEvent, new(), null) is not null)
        {
            throw new InvalidOperationException("Can only run once.");
        }

        try
        {
            await Task
                .Run(RunHostAsync, _runHostCancellation.Token)
                .ConfigureAwait(false);
        }
        finally
        {
            _runHostCompletedEvent.Set();
        }
    }

    public void StopAndWait(TimeSpan timeout)
    {
        _runHostCancellation.Cancel();
        _runHostCompletedEvent?.Wait(timeout);
    }

    public void Dispose()
    {
        _runHostCompletedEvent?.Dispose();
        _runHostCancellation.Dispose();
    }

    private async Task RunHostAsync()
    {
        await Builder
            .Build()
            .RunAsync(_runHostCancellation.Token)
            .ConfigureAwait(false);
    }
}
