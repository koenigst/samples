using System.Windows;
using System.Windows.Threading;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;

namespace Wpf.Hosting;

public sealed partial class WpfLifetime(
    Application application,
    Dispatcher dispatcher,
    IOptions<WpfLifetimeOptions> options,
    ILogger<WpfLifetime> logger)
    : IHostLifetime
{
    private readonly Guid _id = Guid.NewGuid();

    public async Task WaitForStartAsync(CancellationToken cancellationToken)
    {
        using var _ = logger.BeginScope(_id);
        if (options.Value.WaitForApplicationIdle)
        {
            LogStarting(logger);

            await dispatcher
                .InvokeAsync(
                    () => { },
                    DispatcherPriority.ContextIdle,
                    cancellationToken)
                .Task
                .ConfigureAwait(false);
        }

        LogStarted(logger);
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        dispatcher.BeginInvoke(
            (Action)application.Shutdown,
            DispatcherPriority.ContextIdle);

        LogStopped(logger);

        return Task.CompletedTask;
    }

    [LoggerMessage(1, LogLevel.Information, "Application starting.")]
    private static partial void LogStarting(ILogger logger);

    [LoggerMessage(2, LogLevel.Information, "Application started.")]
    private static partial void LogStarted(ILogger logger);

    [LoggerMessage(3, LogLevel.Information, "Application stopped.")]
    private static partial void LogStopped(ILogger logger);
}
