using Wpf.Interaction;
using Wpf.Navigation;

namespace Wpf.Sample;

public static class ApplicationServiceCollectionExtensions
{
    public static IServiceCollection AddApplication(this IServiceCollection services)
    {
        services.AddNavigation();

        services.TryAddSingleton<ICommandContext, CommandContext>();

        services.TryAddScoped<HomePageViewModel>();

        services.Configure<ViewModelProviderOptions>(Configure);

        return services;
    }

    private static void Configure(ViewModelProviderOptions options)
    {
        options.ViewToViewModelMap[typeof(HomePage)] = typeof(HomePageViewModel);
    }
}
