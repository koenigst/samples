using System.Windows;
using System.Windows.Navigation;

namespace Wpf.Navigation;

internal sealed class NavigationScopeService(
    IServiceScopeFactory scopeFactory,
    ViewModelProvider viewModelProvider)
    : IAsyncDisposable, IDisposable
{
    private readonly Dictionary<object, NavigationScope> _scopes = new(ReferenceEqualityComparer.Instance);

    public void SetScope(object navigator, object content)
    {
        if (content is FrameworkElement element &&
            viewModelProvider.TryGetFactory(content.GetType()) is { } factory)
        {
            SetScope(navigator, element, factory);
        }
    }

    public async void Navigate(object _, NavigationEventArgs args)
    {
        if (_scopes.Remove(args.Navigator, out var oldScope))
        {
            oldScope.Element.CheckAccess();
            oldScope.Element.DataContext = null;
            await oldScope.Services.DisposeAsync().ConfigureAwait(true);
        }

        SetScope(args.Navigator, args.Content);
    }

    public async ValueTask DisposeAsync()
    {
        foreach (var scope in _scopes.Values)
        {
            await scope.Services.DisposeAsync().ConfigureAwait(false);
        }
    }

    public void Dispose()
    {
        foreach (var scope in _scopes.Values)
        {
            scope.Services.Dispose();
        }
    }

    private void SetScope(object navigator, FrameworkElement element, Func<IServiceProvider, object> viewModelFactory)
    {
        var scope = scopeFactory.CreateAsyncScope();
        try
        {
            _scopes.Add(navigator, new(scope, element));
            element.DataContext = viewModelFactory(scope.ServiceProvider);
        }
        catch
        {
            scope.Dispose();
            throw;
        }
    }

    private readonly record struct NavigationScope(AsyncServiceScope Services, FrameworkElement Element);
}
