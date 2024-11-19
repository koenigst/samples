using System.Windows;
using System.Windows.Navigation;
using System.Windows.Threading;
using Microsoft.Extensions.Hosting;

namespace Wpf.Navigation;

internal sealed class NavigationHostedService(
    Application application,
    NavigationScopeService navigation)
    : BackgroundService
{
    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        var handler = await application.Dispatcher
            .InvokeAsync(Start, DispatcherPriority.Normal, stoppingToken)
            .Task
            .ConfigureAwait(false);

        try
        {
            await Task.Delay(Timeout.Infinite, stoppingToken).ConfigureAwait(false);
        }
        finally
        {
            application.Navigated -= handler;
        }
    }

    private NavigatedEventHandler Start()
    {
        NavigatedEventHandler handler = navigation.Navigate;
        application.Navigated += handler;
        try
        {
            if (application.MainWindow is { Content: { } content, } window)
            {
                navigation.SetScope(window, content);
            }

            return handler;
        }
        catch
        {
            application.Navigated -= handler;
            throw;
        }
    }
}
