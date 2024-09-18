namespace Wpf.Navigation;

public static class NavigationServiceCollectionExtensions
{
    public static IServiceCollection AddNavigation(this IServiceCollection services)
    {
        services.AddOptions();

        services.TryAddTransient<ViewModelProvider>();
        services.TryAddSingleton<NavigationScopeService>();

        services.AddHostedService<NavigationHostedService>();

        return services;
    }
}
