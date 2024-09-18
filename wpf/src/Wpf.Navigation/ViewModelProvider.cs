using Microsoft.Extensions.Options;

namespace Wpf.Navigation;

public sealed class ViewModelProviderOptions
{
    public IDictionary<Type, Type> ViewToViewModelMap { get; } = new Dictionary<Type, Type>();
}

internal sealed class ViewModelProvider(
    IOptions<ViewModelProviderOptions> options)
{
    public Func<IServiceProvider, object>? TryGetFactory(Type viewType)
    {
        return options.Value.ViewToViewModelMap.TryGetValue(viewType, out var viewModelType)
            ? s => s.GetRequiredService(viewModelType)
            : null;
    }
}
