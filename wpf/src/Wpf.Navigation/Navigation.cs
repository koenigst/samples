using System.Windows.Navigation;
using Wpf.Interaction;

namespace Wpf.Navigation;

public static class Navigation
{
    private static readonly Func<NavigationService, bool> Always = _ => true;

    public static IAsyncCommand Create(
        Action<NavigationService> navigate,
        Func<NavigationService, bool>? canExecute = null,
        ICommandContext? context = null)
    {
        return new AsyncNavigationCommand(navigate, canExecute ?? Always)
            .Wrap(context);
    }

    public static IAsyncCommand Create(
        Func<Uri> navigationUri,
        Func<NavigationService, bool>? canExecute = null,
        ICommandContext? context = null)
    {
        return Create(s => s.Navigate(navigationUri()), canExecute, context);
    }

    public static IAsyncCommand CreateNavigate(
        this ICommandContext context,
        Action<NavigationService> navigate,
        Func<NavigationService, bool>? canExecute = null)
    {
        return Create(navigate, canExecute, context);
    }

    public static IAsyncCommand CreateNavigate(
        this ICommandContext context,
        Func<Uri> navigationUri,
        Func<NavigationService, bool>? canExecute = null)
    {
        return Create(navigationUri, canExecute, context);
    }

    private sealed class AsyncNavigationCommand(
        Action<NavigationService> navigate,
        Func<NavigationService, bool> canExecute)
        : IAsyncCommand
    {
        public event EventHandler? CanExecuteChanged
        {
            add { }
            remove { }
        }

        public bool CanExecute(object? parameter)
        {
            return parameter is NavigationService service
                && canExecute(service);
        }

        public ValueTask ExecuteAsync(object? parameter)
        {
            if (parameter is not NavigationService service)
            {
                throw new InvalidOperationException($"Enforced by {nameof(CanExecute)}.");
            }

            navigate(service);

            return ValueTask.CompletedTask;
        }
    }
}
