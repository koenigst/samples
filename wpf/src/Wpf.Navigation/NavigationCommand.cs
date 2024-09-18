using System.Windows.Input;
using System.Windows.Navigation;

namespace Wpf.Navigation;

public sealed class NavigationCommand(
    Func<NavigationService, bool> canExecute,
    Action<NavigationService> execute)
    : ICommand
{
    public event EventHandler? CanExecuteChanged;

    public static NavigationCommand Create(Func<Uri> navigationUri)
    {
        return new(_ => true, s => s.Navigate(navigationUri()));
    }

    public bool CanExecute(object? parameter)
    {
        return parameter is NavigationService service
            && canExecute(service);
    }

    public void Execute(object? parameter)
    {
        if (parameter is not NavigationService service)
        {
            throw new InvalidOperationException($"Enforced by {nameof(CanExecute)}.");
        }

        execute(service);
    }

    public void TriggerCanExecuteChanged()
    {
        CanExecuteChanged?.Invoke(this, EventArgs.Empty);
    }
}
