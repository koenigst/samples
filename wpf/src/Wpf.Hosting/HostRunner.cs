namespace Wpf.Hosting;

public sealed class HostRunner : IDisposable, IAsyncDisposable
{
    private volatile object? _state;

    public IHostBuilder Builder { get; } = new HostBuilder();

    public void Start()
    {
        var run = new HostRun(this);

        if (Interlocked.CompareExchange(ref _state, run, null) is not null)
        {
            throw new InvalidOperationException("Can only run once.");
        }

        run.EnsureStarted();
    }

    public void Dispose()
    {
        Wait(DisposeAsync());
    }

    public async ValueTask DisposeAsync()
    {
        if (Interlocked.Exchange(ref _state, "disposed") is HostRun run)
        {
            await run.DisposeAsync().ConfigureAwait(false);
        }
    }

    private async Task RunAsync(Task stopTrigger)
    {
        Task runTask;

        using var cancellation = new CancellationTokenSource();
        try
        {
            var cancellationToken = cancellation.Token;
            runTask = Task.Run(() => RunHostAsync(cancellationToken), cancellationToken);

            await Task.WhenAny(runTask, stopTrigger).ConfigureAwait(false);
        }
        finally
        {
            await cancellation.CancelAsync().ConfigureAwait(false);
        }

        await runTask.ConfigureAwait(false);
    }

    private async Task RunHostAsync(CancellationToken cancellationToken)
    {
        await Builder
            .Build()
            .RunAsync(cancellationToken)
            .ConfigureAwait(false);
    }

    private static void Wait(ValueTask task)
    {
        if (task.IsCompletedSuccessfully)
        {
            return;
        }

        task.AsTask().GetAwaiter().GetResult();
    }

    private sealed class HostRun : IAsyncDisposable
    {
        private readonly TaskCompletionSource _stopSource = new();
        private readonly Lazy<Task> _runCache;

        public HostRun(HostRunner runner)
        {
            _runCache = new(() => runner.RunAsync(_stopSource.Task));
        }

        // ReSharper disable once AsyncVoidMethod - By design to crash the application if the run fails
        public async void EnsureStarted()
        {
            await _runCache.Value.ConfigureAwait(false);
        }

        public async ValueTask DisposeAsync()
        {
            _stopSource.TrySetResult();
            await _runCache.Value.ConfigureAwait(false);
        }
    }
}
