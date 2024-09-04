using System.Windows;

namespace Wpf.Hosting;

public static class WpfHostingServiceCollectionExtensions
{
    public static IServiceCollection AddWpfLifetime(
        this IServiceCollection services,
        Application application,
        Action<WpfLifetimeOptions>? configure = null)
    {
        services.AddLogging();

        services.TryAddSingleton(application);
        services.TryAddSingleton(application.Dispatcher);

        services.RemoveAll<IHostLifetime>();
        services.AddSingleton<IHostLifetime, WpfLifetime>();

        if (configure is not null)
        {
            services.Configure(configure);
        }

        return services;
    }
}
