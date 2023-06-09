namespace SampleApp;

public static class AppServiceCollectionExtensions
{
    public static IServiceCollection AddApp(this IServiceCollection services)
    {
        services.TryAddTransient<Random>();
        services.TryAddSingleton<IGuessingService, GuessingService>();

        return services;
    }

    public static IServiceCollection AddAppInfrastructure(this IServiceCollection services)
    {
        services.AddControllers();

        return services;
    }
}
