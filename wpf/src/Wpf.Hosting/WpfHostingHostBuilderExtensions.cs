using System.Windows;

namespace Wpf.Hosting;

public static class WpfHostingHostBuilderExtensions
{
    public static IHostBuilder UseWpfLifetime(
        this IHostBuilder builder,
        Application application,
        Action<WpfLifetimeOptions>? configure = null)
    {
        return builder.ConfigureServices(s => s.AddWpfLifetime(application, configure));
    }
}
