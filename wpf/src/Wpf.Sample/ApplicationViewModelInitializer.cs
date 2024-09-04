using System.Windows;
using System.Windows.Threading;

namespace Wpf.Sample;

public sealed class ApplicationViewModelInitializer(
    Application application,
    ApplicationViewModel viewModel)
    : IHostedService
{
    public async Task StartAsync(CancellationToken cancellationToken)
    {
        await InvokeAsync(Initialize, cancellationToken).ConfigureAwait(false);
        await InvokeAsync(() => { }, cancellationToken).ConfigureAwait(false);
    }

    private Task InvokeAsync(Action action, CancellationToken cancellationToken)
    {
        return application.Dispatcher
            .InvokeAsync(action, DispatcherPriority.ContextIdle, cancellationToken)
            .Task;
    }

    private void Initialize()
    {
        var mainWindow = application.MainWindow ?? throw new InvalidOperationException("Missing main window.");
        mainWindow.DataContext = viewModel;
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        return Task.CompletedTask;
    }
}
