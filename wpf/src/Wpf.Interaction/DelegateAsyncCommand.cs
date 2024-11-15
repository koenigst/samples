namespace Wpf.Interaction;

internal sealed class DelegateAsyncCommand(
    Func<object?, ValueTask> executeAsync,
    Func<object?, bool> canExecute)
    : IAsyncCommand
{
    public event EventHandler? CanExecuteChanged
    {
        add { }
        remove { }
    }

    public bool CanExecute(object? parameter)
    {
        return canExecute.Invoke(parameter);
    }

    public ValueTask ExecuteAsync(object? parameter)
    {
        return executeAsync(parameter);
    }
}
