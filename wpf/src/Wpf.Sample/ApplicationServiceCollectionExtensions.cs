namespace Wpf.Sample;

public static class ApplicationServiceCollectionExtensions
{
    public static IServiceCollection AddApplication(this IServiceCollection services)
    {
        services.TryAddSingleton<ApplicationViewModel>();

        services.AddHostedService<ApplicationViewModelInitializer>();

        return services;
    }
}
